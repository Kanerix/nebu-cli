use std::{path::PathBuf, process::ExitCode};

use clap::{Parser, crate_authors, crate_version};
use clap_cargo::style::CLAP_STYLING;

mod cmds;

#[derive(Parser)]
#[command(
    name = "nebu",
    about = "A command-line interface for Lerpz",
    author = crate_authors!(),
    version = crate_version!(),
    after_help = "For more information, visit https://github.com/lerpz-com",
    disable_version_flag = true,
    disable_help_subcommand = true,
    styles = CLAP_STYLING,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Global arguments for all commands
    #[command(flatten)]
    global_args: Box<GlobalArgs>,
}

#[derive(clap::Subcommand)]
#[command(next_help_heading = "Commands", next_display_order = 1)]
enum Commands {
    /// Show the version of the CLI
    Version,
    /// Environment subcommands
    /// 
    /// Can be used to check the environment configuration or perform other
    /// environment-related tasks.
    Env(cmds::env::Env),
    /// Project subcommands
    /// 
    /// Creates new project, manage existing projects, or perform
    /// project-related tasks.
    Project(cmds::project::Project),
    /// Infrastructure subcommands
    /// 
    /// Manage infrastructure resources, such as key vaults, databases and more.
    Infra,
}

#[derive(clap::Parser, Debug)]
struct GlobalArgs {
    #[arg(
        global = true,
        short = 'F',
        long,
        env = "NEBU_OUTPUT_FORMAT",
        default_value = OutputFormats::default()
    )]
    format: OutputFormats,

    #[arg(
        global = true,
        long,
        env = "NEBU_CONFIG_PATH",
        default_value = "~/.config/nebu"
    )]
    config_path: PathBuf,

    #[arg(
        global = true,
        hide = true,
        long,
        env = "NEBU_HOME_PATH",
        default_value = "~/.nebu"
    )]
    home_path: PathBuf,

    #[arg(
        global = true,
        short = 'v',
        env = "NEBU_VERBOSE",
        action = clap::ArgAction::Count,
    )]
    verbose: u8
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
enum OutputFormats {
    #[default]
    Text,
    Json,
    #[cfg(feature = "schema")]
    JsonSchema,
}

impl From<OutputFormats> for clap::builder::OsStr {
    fn from(value: OutputFormats) -> Self {
        match value {
            OutputFormats::Text => "text".into(),
            OutputFormats::Json => "json".into(),
            #[cfg(feature = "schema")]
            OutputFormats::JsonSchema => "json-schema".into(),
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => match cli.global_args.format {
            OutputFormats::Json => {
                let out = serde_json::to_string_pretty(&cmds::nebu_version())
                    .expect("Failed to serialize version info to JSON");
                println!("{}", out)
            }
            OutputFormats::Text => {
                println!("{}", cmds::nebu_version())
            }
            #[cfg(feature = "schema")]
            OutputFormats::JsonSchema => {
                let schema = schemars::schema_for!(cmds::version::VersionInfo);
                let out =
                    serde_json::to_string_pretty(&schema).expect("Failed to serialize JSON schema");
                println!("{}", out)
            },
        },
        Some(Commands::Env(_env)) => todo!(),
        Some(Commands::Project(_project)) => todo!(),
        Some(Commands::Infra) => todo!(),
        None => todo!(), // Make a decision on what the default command should be.
    }

    ExitCode::SUCCESS
}
