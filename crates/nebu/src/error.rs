use miette::Diagnostic;
use thiserror::Error;

pub(crate) type CommandResult<T = ()> = miette::Result<T, CommandError>;

pub(crate) struct CommandError {
    /// The inner error kind.
    pub inner: CommandErrorKind,
}

#[derive(Error, Diagnostic, Debug)]
pub(crate) enum CommandErrorKind {
    /// Something went wrong using git.
    #[error(transparent)]
    #[diagnostic(
        code(command::git_error),
        help("Check the documentation for more information"),
        url("https://nebu.lerpz.com/docs/cli/errors#git_error")
    )]
    GitError(#[from] git2::Error),
    /// Something is wrong with the command arguments.
    #[error(transparent)]
    #[diagnostic(
        code(command::io_error),
        help("Check the file path or directory and permissions"),
        url("https://nebu.lerpz.com/docs/cli/errors#arguments")
    )]
    IoError(#[from] std::io::Error),
    /// All other errors that do not fit into a specific category.
    #[error(transparent)]
    #[diagnostic(
        code(command::execution_failed),
        help("Use the `--help` flag to see available options and arguments"),
        url("https://nebu.lerpz.com/docs/cli/errors#execution_failed")
    )]
    Other(#[from] anyhow::Error),
}

impl CommandError {
    /// Create a new [`CommandError`] from a [`git2::Error`].
    pub fn from_git2(err: git2::Error) -> Self {
        CommandError {
            inner: CommandErrorKind::GitError(err),
        }
    }
}

impl<E> From<E> for CommandError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        CommandError {
            inner: CommandErrorKind::Other(err.into()),
        }
    }
}
