use std::fmt;

use owo_colors::OwoColorize;
use serde::Serialize;

use crate::{OutputFormats, cmds};

#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub(crate) struct VersionInfo {
    /// SemVer version, such as "0.1.0"
    version: &'static str,
    /// Information about the commit this version was built from.
    ///
    /// Might not be present if the version was not built from a git repository.
    commit_info: Option<CommitInfo>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub(crate) struct CommitInfo {
    short_hash: &'static str,
    hash: &'static str,
    date: &'static str,
    commit_tag_info: Option<CommitTagInfo>,
}

#[derive(Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub(crate) struct CommitTagInfo {
    pub(crate) last_tag: &'static str,
    pub(crate) commits_since: &'static str,
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " ({} {})", self.short_hash, self.date)?;
        Ok(())
    }
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version.blue())?;
        if let Some(commit_info) = &self.commit_info {
            write!(f, " {} (", commit_info.date.cyan())?;
            if let Some(tag_info) = &commit_info.commit_tag_info {
                write!(f, "{}+{} ", tag_info.last_tag, tag_info.commits_since)?;
            }
            write!(f, "{})", commit_info.short_hash.green())?;
        }
        Ok(())
    }
}

impl From<VersionInfo> for clap::builder::Str {
    fn from(value: VersionInfo) -> Self {
        value.to_string().into()
    }
}

pub(crate) fn run(global_args: Box<crate::GlobalArgs>) {
    tracing::trace!("running version command");
    tracing::trace!("reading version information from build variables");

    let commit_info = option_env!("NEBU_COMMIT_HASH")
        .zip(option_env!("NEBU_COMMIT_SHORT_HASH"))
        .zip(option_env!("NEBU_COMMIT_DATE"))
        .map(|((hash, short_hash), date)| CommitInfo {
            hash,
            short_hash,
            date,
            commit_tag_info: option_env!("NEBU_LAST_TAG")
                .zip(option_env!("NEBU_LAST_TAG_DISTANCE"))
                .map(|(last_tag, commits_since)| CommitTagInfo {
                    last_tag,
                    commits_since,
                }),
        });

    let version_info = VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
        commit_info,
    };

    tracing::trace!("creating output for version information");

    match global_args.format {
        OutputFormats::Json => {
            let out = serde_json::to_string_pretty(&version_info)
                .expect("Failed to serialize version info to JSON");
            println!("{}", out);
        }
        OutputFormats::Text => {
            println!("{}", version_info);
        }
        #[cfg(feature = "schema")]
        OutputFormats::JsonSchema => {
            let schema = schemars::schema_for!(cmds::version::VersionInfo);
            let out = serde_json::to_string_pretty(&schema)
                .expect("Failed to serialize JSON schema");
            println!("{}", out)
        }
    }
}
