use super::project;

mod init;

#[derive(clap::Args)]
pub(crate) struct Project {
    #[command(subcommand)]
    command: ProjectCmds,

    /// Arguments for the project command
    #[command(flatten)]
    args: ProjectArgs,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub(crate) enum ProjectCmds {
    Init(InitArgs),
}

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
    #[arg(
        short = 'r',
        long,
        default_value = "origin",
        env = "NEBU_TEMPLATE_remote"
    )]
    /// Remote of the repository to use.
    repo_remote: String,
}

#[derive(clap::Args, Debug, Clone)]
pub(crate) struct ProjectArgs {
    #[arg(
        global = true,
        short,
        long,
        env = "NEBU_TEMPLATE_REPO",
        default_value = "https://github.com/Kanerix/nebu-cli.git"
    )]
    pub template_repo: String,
}

pub(crate) async fn run(
    project: Project,
    global_args: Box<crate::GlobalArgs>,
) -> crate::error::CommandResult {
    match project.command {
        ProjectCmds::Init(init_args) => {
            project::init::run(global_args, project.args, init_args).await
        }
    }
}
