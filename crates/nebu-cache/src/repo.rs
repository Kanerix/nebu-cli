use std::path::Path;

use git2::{FetchOptions, Repository};

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

        let repo = Repository::open(location).unwrap_or_default();

        let remote = match repo.find_remote("origin") {
            Ok(remote) => remote,
            Err(_) => return false,
        };

        let fetch = remote.fetch(&[], Some(FetchOptions::), None);

        true
    }

    fn refresh_force(&mut self, location: &Path) -> Result<()> {
        let _ = Repository::clone(&self.repo, "some path")?;
        Ok(())
    }
}
