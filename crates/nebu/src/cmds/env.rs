#[derive(clap::Args)]
#[command(
    name = "env",
    about = "Environment specific commands",
    alias = "environment",
    long_about = "Commands for managing and checking the environment configuration."
)]
pub(crate) struct Env {
    #[command(subcommand)]
    command: EnvCmds,
}

#[derive(clap::Subcommand)]
pub(crate) enum EnvCmds {
    Check,
}
