use std::path::Path;

use git2::Repository;

use crate::Refresh;
use crate::error::Result;

pub struct RepoCache {
    pub repo: String,
}

impl RepoCache {
    pub fn new(repo: impl Into<String>) -> Self {
        RepoCache { repo: repo.into() }
    }
}

impl Refresh for RepoCache {
    fn is_fresh(&self, location: &Path) -> bool {
        if !location.is_dir() {
            return false;
        }

        let repo = if let Ok(repo) = Repository::open(location) {
            repo
        } else {
            return false;
        };

        let head = if let Ok(head) = repo.head() {
            head
        } else {
            return false;
        };

        if !head.is_branch() {
            return false
        }

        let remote = if let Ok(remote) = repo.find_remote("origin") {
            remote
        } else {
            return false;
        };

        let current_branch = repo.branch_upstream_remote("/ref/head/main");

        // let fetch = remote.fetch(&[], None, None);

        true
    }

    fn refresh_force(&mut self, location: &Path) -> Result<()> {
        let _ = Repository::clone(&self.repo, "some path")?;
        Ok(())
    }
}
