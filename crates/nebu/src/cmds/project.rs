#[derive(clap::Subcommand, Debug, Clone)]
enum Project {
    Init
}

#[derive(clap::Args)]
struct ProjectArgs {
    pub template_repo: Option<String>,
}
