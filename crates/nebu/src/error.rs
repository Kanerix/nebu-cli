use std::process::ExitCode;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub(crate) type Result<T = ()> = miette::Result<T>;

#[derive(Error, Diagnostic, Debug)]
#[error("command failed with exit code {}", exit_code)]
#[diagnostic(
    code(command::execution_failed),
    help("Use the `--help` flag to see available options and arguments"),
    url("https://nebu.lerpz.com/docs")
)]
pub(crate) struct CommandError {
    /// The exit code of the command that failed.
    ///
    /// This wil be [`ExitCode::SUCCESS`] if the command was successful.
    exit_code: u32,
    /// The source of the error.
    ///
    /// This is an `anyhow::Error` containing more details.
    #[source]
    #[diagnostic_source]
    source: CommandErrorKind,
    /// The command that was executed.
    #[diagnostic(transparent)]
    command: Option<String>,
    /// The source code where the command was executed.
    #[source_code]
    source_code: Option<String>,
    /// The part of the source code that caused the failure.
    #[label("this part of the command failed")]
    source_span: Option<SourceSpan>,
    /// Additional details about the error.
    ///
    /// This can help provide more context for the error and how to resolve it.
    #[related]
    related: Vec<CommandErrorKind>,
}

#[derive(Error, Diagnostic, Debug)]
pub(crate) enum CommandErrorKind {
    #[error("invalid argument: {arg}")]
    #[diagnostic(
        code(command::invalid_argument),
        help("Check the argument and ensure it is valid")
    )]
    InvalidArgument {
        arg: String,
        #[label("invalid argument")]
        span: Option<SourceSpan>,
    },
    #[error("Permission denied")]
    #[diagnostic(
        code(command::permission_denied),
        help("Try running with elevated privileges or check file permissions")
    )]
    PermissionDenied,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl CommandError {
    pub(crate) fn new(exit_code: u32, source: anyhow::Error) -> Self {
        Self {
            exit_code,
            source: source.into(),
            command: None,
            source_code: None,
            source_span: None,
            related: Vec::new(),
        }
    }

    pub(crate) fn exit_code(&self) -> u32 {
        self.exit_code
    }

    pub(crate) fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    pub(crate) fn with_source_context(
        mut self,
        code: impl Into<String>,
        span: impl Into<SourceSpan>,
    ) -> Self {
        self.source_code = Some(code.into());
        self.source_span = Some(span.into());
        self
    }

    pub(crate) fn with_related(mut self, related: CommandErrorKind) -> Self {
        self.related.push(related);
        self
    }

    pub(crate) fn from_err<E>(err: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        Self::new(1, err.into())
    }
}
