use std::path::Path;

use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{
    BranchType, Config, Cred, CredentialHelper, CredentialType, FetchOptions, Oid, RemoteCallbacks, Repository
};

use crate::Refresh;
use crate::error::Result;

#[derive(Debug, Clone, Hash)]
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

    /// Returns the local and remote OIDs of the specified branch in the repository.
    pub fn get_local_and_remote_oids(&self, repo: &Repository) -> Result<(Oid, Oid)> {
        let branch = repo.find_branch(&self.branch, BranchType::Local)?;
        let local_oid = branch
            .get()
            .target()
            .ok_or(git2::Error::from_str("No local OID found"))?;

        let remote = branch.upstream()?;
        let remote_oid = remote
            .get()
            .target()
            .ok_or(git2::Error::from_str("No remote OID found"))?;

        Ok((local_oid, remote_oid))
    }

    /// Returns a RemoteCallbacks instance that handles authentication.
    pub fn get_callbacks<'a>() -> RemoteCallbacks<'a> {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username, allowed_types| {
            // Prepare the authentication callbacks.
            //
            // We check the `allowed` types of credentials, and we try to do as much as
            // possible based on that:
            //
            // * Prioritize SSH keys from the local ssh agent as they're likely the most
            //   reliable. The username here is prioritized from the credential
            //   callback, then from whatever is configured in git itself, and finally
            //   we fall back to the generic user of `git`.
            //
            // * If a username/password is allowed, then we fallback to git2-rs's
            //   implementation of the credential helper. This is what is configured
            //   with `credential.helper` in git, and is the interface for the OSX
            //   keychain, for example.
            //
            // * After the above two have failed, we just kinda grapple attempting to
            //   return *something*.
            let mut cred_helper = CredentialHelper::new(url);
            let cfg = &Config::open_default()?;
            cred_helper.config(cfg);
            if allowed_types.contains(CredentialType::SSH_KEY) {
                let mut attempts = vec![String::from("git")];
                if let Some(s) = cred_helper.username {
                    attempts.push(s);
                }

                while let Some(s) = attempts.pop() {
                    return git2::Cred::ssh_key_from_agent(&s);
                }
            }

            if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                let username = username.unwrap_or("git");
                return Cred::userpass_plaintext(username, "");
            }


            if allowed_types.contains(CredentialType::DEFAULT) {
                return Cred::default()
            }

            Err(git2::Error::from_str("no suitable credentials found"))
        });
        callbacks
    }

    /// Clones the repository to the specified location.
    ///
    /// This adds remote callbacks to handle authentication and fetch options.
    pub fn clone_repository(
        &self,
        location: &Path,
    ) -> std::result::Result<Repository, git2::Error> {
        let mut builder = RepoBuilder::new();
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(Self::get_callbacks());
        builder.fetch_options(fetch_options);
        builder.clone(&self.repo, location)
    }
}

impl Refresh for RepoCache {
    fn is_fresh(&self, location: &Path) -> Result<bool> {
        if !location.exists() || !location.is_dir() {
            return Ok(false);
        }

        let repo = match Repository::open(location) {
            Ok(repo) => Ok(repo),
            Err(err) => {
                if err.code() == git2::ErrorCode::NotFound {
                    return Ok(false);
                } else {
                    Err(err)
                }
            }
        }?;

        let (local_oid, remote_oid) = self.get_local_and_remote_oids(&repo)?;
        let (ahead, behind) = repo.graph_ahead_behind(local_oid, remote_oid)?;

        if ahead > 0 || behind > 0 {
            return Ok(false);
        }

        Ok(true)
    }

    fn refresh(&mut self, location: &Path) -> Result<bool> {
        if !location.exists() || !location.is_dir() {
            std::fs::create_dir_all(&location)?;
        }

        let repo = match Repository::open(location) {
            Ok(repo) => Ok(repo),
            Err(err) => {
                if err.code() == git2::ErrorCode::NotFound {
                    self.clone_repository(location)
                } else {
                    Err(err)
                }
            }
        }?;

        let mut remote = repo.find_remote(&self.remote)?;

        let callbacks = RepoCache::get_callbacks();
        let mut options = FetchOptions::new();
        options.remote_callbacks(callbacks);

        let refspecs = remote.fetch_refspecs()?;
        let collect = refspecs
            .iter()
            .map(|refspec| refspec.unwrap())
            .collect::<Vec<&str>>();
        remote.fetch(&collect, Some(&mut options), None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_fast_forward() {
            let head = repo.head()?;
            let refname = head
                .name()
                .ok_or(git2::Error::from_str("No HEAD reference found"))?;
            let mut reference = repo.find_reference(refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            repo.set_head(refname)?;
            repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        }

        Ok(true)
    }
}
