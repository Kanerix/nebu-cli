use std::path::Path;

use tempfile::tempdir;

use crate::error::{CommandError, CommandResult};

#[derive(clap::Args, Debug, Clone)]
pub(crate) struct InitArgs {
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
    global_args: Box<crate::GlobalArgs>,
    project_args: super::ProjectArgs,
    args: InitArgs,
) -> CommandResult {
    tracing::trace!("running project init command");

    let tempdir = tempdir()?;

    clone_to_tempdir(tempdir.path(), &global_args, &project_args, &args)?;

    Ok(())
}

fn clone_to_tempdir(
    path: &Path,
    global_args: &crate::GlobalArgs,
    project_args: &super::ProjectArgs,
    args: &InitArgs,
) -> CommandResult {
    if project_args.no_cache {
        git2::Repository::clone(&args.repo_url, path).map_err(CommandError::from_git2)?;
    } else {
        let _ = &global_args.cache_path;
        todo!("Implement caching logic");
    }

    Ok(())
}
