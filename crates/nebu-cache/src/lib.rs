use std::path::{Path, PathBuf};

use crate::error::Result;
pub use crate::repo::*;

mod error;
mod repo;

/// Trait for refreshing cached data.
pub trait Refresh {
    /// Check if the cached data is fresh at the given location.
    ///
    /// This might error if the location is invalid or the data is corrupted.
    fn is_fresh(&self, location: &Path) -> Result<bool>;
    /// Refresh the cached data, without checking if it is fresh.
    ///
    /// Returns `true` if the data was refreshed, `false` otherwise.
    fn refresh(&mut self, location: &Path) -> Result<bool>;
    /// Refresh the cached data if it is not fresh.
    ///
    /// Returns `true` if the data was refreshed, `false` otherwise.
    fn try_refresh(&mut self, location: &Path) -> Result<bool> {
        if !self.is_fresh(location)? {
            self.refresh(location)
        } else {
            Ok(false)
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
    ///
    /// Returns `true` if the data is fresh, `false` otherwise.
    pub fn is_fresh(&self) -> Result<bool> {
        Ok(self.inner.is_fresh(&self.location)?)
    }

    /// Refresh the cached data, without checking if it is fresh.
    ///
    /// Returns `true` if the data was refreshed, `false` otherwise.
    pub fn refresh(&mut self) -> Result<bool> {
        self.inner.refresh(&self.location)
    }

    /// Checks if the cache is fresh and refreshes it if not.
    ///
    /// Returns `true` if the data was refreshed, `false` otherwise.
    pub fn try_refresh(&mut self) -> Result<bool> {
        self.inner.try_refresh(&self.location)
    }
}
