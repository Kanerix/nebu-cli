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
    Init,
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

pub fn run(project: Project, global_args: Box<crate::GlobalArgs>) -> crate::error::CommandResult {
    match project.command {
        ProjectCmds::Init => project::init::run(global_args, project.args),
    }
}
