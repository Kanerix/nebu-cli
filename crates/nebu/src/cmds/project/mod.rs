use super::project;

mod add;
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
    Add(add::AddArgs),
    Init(init::InitArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub(crate) struct ProjectArgs {
    #[arg(
        global = true,
        long,
        env = "NEBU_TEMPLATE_REPO",
        default_value = "y",
        value_parser = clap::builder::BoolishValueParser::new(),
    )]
    pub no_cache: bool,
}

pub(crate) async fn run(
    project: Project,
    global_args: Box<crate::GlobalArgs>,
) -> crate::error::CommandResult {
    match project.command {
        ProjectCmds::Add(add_args) => {
            project::add::run(global_args, project.args, add_args).await
        }
        ProjectCmds::Init(init_args) => {
            project::init::run(global_args, project.args, init_args).await
        }
    }
}
