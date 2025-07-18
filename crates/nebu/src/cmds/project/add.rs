use crate::error::CommandResult;

#[derive(clap::Args, Debug, Clone)]
pub(crate) struct AddArgs {
    /// URL of the repository to use as a template.
    #[arg(
        short = 'u',
        long,
        default_value = "https://github.com/kanerix/nebu-cli.git",
        env = "NEBU_TEMPLATE_REPO"
    )]
    repo_url: String,
    /// Branch of the repository to use.
    #[arg(
        short = 'b',
        long,
        default_value = "main",
        env = "NEBU_TEMPLATE_BRANCH"
    )]
    repo_branch: String,
    /// Remote of the repository to use.
    #[arg(
        short = 'r',
        long,
        default_value = "origin",
        env = "NEBU_TEMPLATE_remote"
    )]
    repo_remote: String,
}

pub async fn run(
    _global_args: Box<crate::GlobalArgs>,
    _project_args: super::ProjectArgs,
    _init_args: AddArgs,
) -> CommandResult {
    tracing::trace!("running project add command");
    Ok(())
}
