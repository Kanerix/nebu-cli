pub mod version;

use clap::{crate_authors, crate_version, ArgAction, Parser, Subcommand, ValueEnum};
use clap_cargo::style::CLAP_STYLING;

#[derive(Parser)]
#[command(name = "nebu", author = crate_authors!(), version = crate_version!())]
#[command(about = "A command-line interface for Lerpz")]
#[command(after_help = "For more information, visit https://github.com/lerpz-com")]
#[command(bin_name = "nebu")]
#[command(styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    pub global_args: GlobalArgs,
}

#[derive(Subcommand)]
#[command(next_help_heading = "Commands", next_display_order = 1)]
enum Commands {
    /// Show the version of the Nebu CLI
    Version,
}

#[derive(Parser, Debug)]
struct GlobalArgs {
    #[arg(
        global = true,
        short,
        long,
        env = "NEBU_OUTPUT_FORMAT",
        action = ArgAction::Set,
        default_value = OutputFormats::default()
    )]
    format: OutputFormats,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Version) => match cli.global_args.format {
            OutputFormats::Json => {
                let out = serde_json::to_string_pretty(&version::nebu_version())
                    .expect("Failed to serialize version info to JSON");
                println!("{}", out)
            }
            OutputFormats::Text => {
                println!("{}", version::nebu_version())
            }
            #[cfg(feature = "schema")]
            OutputFormats::JsonSchema => {
                let schema = schemars::schema_for!(version::VersionInfo);
                let out =
                    serde_json::to_string_pretty(&schema).expect("Failed to serialize JSON schema");
                println!("{}", out)
            }
        },
        None => {
            println!("{}", version::nebu_version());
        }
    }
}
