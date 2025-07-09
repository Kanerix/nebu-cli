use std::path::Path;

use git2::build::CheckoutBuilder;
use git2::{BranchType, Cred, CredentialType, FetchOptions, Oid, RemoteCallbacks, Repository};

use crate::Refresh;
use crate::error::Result;

pub struct RepoCache {
    pub repo: String,
    pub branch: String,
    pub remote: String,
}

impl RepoCache {
    pub fn new(
        repo: impl Into<String>,
        branch: impl Into<String>,
        remote: impl Into<String>,
    ) -> Self {
        RepoCache {
            repo: repo.into(),
            branch: branch.into(),
            remote: remote.into(),
        }
    }

    pub fn get_local_and_remote_oids(&self, repo: &Repository) -> Result<(Oid, Oid)> {
        let branch = repo.find_branch(&self.branch, BranchType::Local)?;
        let local_oid = branch.get().target().unwrap();

        let upstream = branch.upstream()?;
        let remote_oid = upstream.get().target().unwrap();

        Ok((local_oid, remote_oid))
    }
}

impl Refresh for RepoCache {
    fn is_fresh(&self, location: &Path) -> Result<bool> {
        if !location.is_dir() {
            return Ok(false);
        }

        let repo = Repository::open(location)?;
        let (local_oid, remote_oid) = self.get_local_and_remote_oids(&repo)?;
        let (ahead, behind) = repo.graph_ahead_behind(local_oid, remote_oid)?;

        if ahead > 0 || behind > 0 {
            return Ok(false);
        }

        Ok(true)
    }

    fn refresh(&mut self, location: &Path) -> Result<bool> {
        let repo = Repository::open(location)?;
        let mut remote = repo.find_remote(&self.remote)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_, username, allowed_types| {
            if let Some(username) = username
                && allowed_types.contains(CredentialType::SSH_CUSTOM)
                && let Ok(cred) = Cred::ssh_key_from_agent(username)
            {
                return Ok(cred);
            } else {
                Cred::default()
            }
        });

        let mut options = FetchOptions::new();
        options.remote_callbacks(callbacks);

        let refspecs = remote.fetch_refspecs()?;
        let collect = refspecs
            .iter()
            .map(|refspec| refspec.unwrap())
            .collect::<Vec<_>>();
        remote.fetch(&collect, Some(&mut options), None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_fast_forward() {
            let refname = &format!("refs/heads/{}", self.branch);
            let mut reference = repo.find_reference(refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            repo.set_head(refname)?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        }

        Ok(true)
    }
}
