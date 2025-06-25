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
        default_value = "https://github.com/lerpz-com/nebu-template.git",
    )]
    pub template_repo: String,
}
