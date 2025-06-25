#[derive(clap::Args)]
pub(crate) struct Env {
    #[command(subcommand)]
    command: EnvCmds,
}

#[derive(clap::Subcommand)]
pub(crate) enum EnvCmds {
    Check,
}
