use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    GitError(#[from] git2::Error),
    
    #[error(transparent)]
    Other(#[from] anyhow::Error)
}
