use std::path::PathBuf;

use clap::{Parser, crate_authors, crate_version};
use clap_cargo::style::CLAP_STYLING;
use tracing_subscriber::EnvFilter;

mod cmds;
mod error;

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
    command: Commands,

    /// Global arguments for all commands
    #[command(flatten)]
    global_args: Box<GlobalArgs>,
}

#[derive(clap::Subcommand)]
#[command(next_help_heading = "Commands", next_display_order = 2)]
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
    /// Creates new project, manage existing projects, or perform other
    /// project-related tasks.
    Project(cmds::project::Project),
    /// Infrastructure subcommands
    ///
    /// Manage infrastructure resources, such as key vaults, databases and more.
    Infra,
}

#[derive(clap::Parser, Debug)]
#[command(next_help_heading = "Global options", next_display_order = 1000)]
struct GlobalArgs {
    /// Output format for the command results.
    ///
    /// This is not supported by all commands, but the CLI will attempt to
    /// format the output in the specified format if possible. This does not
    /// error if the format is not supported by the command.
    #[arg(
        global = true,
        help = "Output format for the command results.",
        short = 'F',
        long,
        env = "NEBU_OUTPUT_FORMAT",
        default_value = OutputFormats::default(),
        verbatim_doc_comment,
    )]
    format: OutputFormats,

    /// Path to the configuration directory for nebu.
    ///
    /// This will default to `~/.config/nebu` if not specified, where `~` is the
    /// user's home directory.
    #[arg(
        global = true,
        long,
        env = "NEBU_CONFIG_PATH",
        default_value = "~/.config/nebu",
        verbatim_doc_comment,
    )]
    config_path: PathBuf,

    /// Path to the cache directory for nebu.
    ///
    /// This will default to `~/.nebu` if not specified, where `~` is the user's
    /// home directory.
    #[arg(
        global = true,
        long,
        env = "NEBU_CACHE_PATH",
        default_value = "~/.nebu",
        verbatim_doc_comment,
    )]
    cache_path: PathBuf,

    /// Enable verbose output.
    ///
    /// This will enable more detailed logging output, which can be useful for
    /// debugging. The maximum verbosity is level 3 (TRACE).
    ///
    /// Levels:
    /// - 0: No verbose output
    /// - 1: Info level output
    /// - 2: Debug level output
    /// - 3: Trace level output
    ///
    /// This uses the `RUST_LOG` environment variable to set the logging level.
    /// That means if the `RUST_LOG` environment variable is set, it will
    /// override the verbosity level set by this argument.
    #[arg(
        global = true,
        help = "Enable verbose output. Use multiple times for more verbosity.",
        short = 'v',
        env = "NEBU_VERBOSE",
        action = clap::ArgAction::Count,
        verbatim_doc_comment,
    )]
    verbose: u8,
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

fn expand_dir(path: &PathBuf) -> miette::Result<PathBuf> {
    nebu_fs::expand_home_dir(path).ok_or_else(|| {
        miette::miette!("Failed to expand home directory for path: {}", path.display())
    })
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let mut cli = Cli::parse();

    cli.global_args.cache_path = expand_dir(&cli.global_args.cache_path)?;
    cli.global_args.config_path = expand_dir(&cli.global_args.config_path)?;

    tracing_subscriber::fmt()
        .without_time()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            match cli.global_args.verbose {
                0 => EnvFilter::from("none"),
                1 => EnvFilter::from("none,nebu=info"),
                2 => EnvFilter::from("none,nebu=debug"),
                _ => EnvFilter::from("none,nebu=trace"),
            }
        }))
        .init();

    if std::env::var("RUST_LOG").is_ok() && cli.global_args.verbose > 0 {
        tracing::warn!(
            "RUST_LOG environment variable is present; this will override the \
            verbosity level set by the -v flag."
        );
    }

    let result = match cli.command {
        Commands::Version => cmds::version::run(cli.global_args),
        Commands::Env(_env) => todo!(),
        Commands::Project(project) => cmds::project::run(project, cli.global_args).await,
        Commands::Infra => todo!(),
    };

    if let Err(err) = result {
        return Err(err.into());
    };

    Ok(())
}
