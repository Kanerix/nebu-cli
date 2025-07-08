use std::path::{Path, PathBuf};

use crate::error::Result;
pub use crate::repo::*;

mod error;
mod repo;

/// Trait for refreshing cached data.
pub trait Refresh {
    /// Check if the cached data is fresh at the given location.
    fn is_fresh(&self, location: &Path) -> bool;
    /// Refresh the cached data, without checking if it is fresh.
    fn refresh_force(&mut self, location: &Path) -> Result<()>;
    /// Refresh the cached data if it is not fresh.
    fn refresh(&mut self, location: &Path) -> Result<()> {
        if !self.is_fresh(location) {
            self.refresh_force(location)
        } else {
            Ok(())
        }
    }
}

/// A cache context that can hold different kinds of cached data.
pub struct CacheManager<T>
where
    T: Refresh,
{
    location: PathBuf,
    inner: T,
}

impl<T> CacheManager<T>
where
    T: Refresh,
{
    /// Create a new cache context with the specified root directory and inner
    /// cache.
    pub fn new(location: PathBuf, inner: T) -> Self {
        Self { location, inner }
    }

    /// Checks if the cache is fresh.
    pub fn is_fresh(&self) -> bool {
        self.inner.is_fresh(&self.location)
    }

    /// Checks if the cache is fresh and refreshes it if not.
    pub fn refresh(&mut self) -> Result<()> {
        self.inner.refresh(&self.location)
    }
}
