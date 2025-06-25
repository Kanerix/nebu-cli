use std::fmt;

use serde::Serialize;

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
    pub(crate) commits_since: Option<&'static str>,
}

impl fmt::Display for CommitInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " ({} {})", self.short_hash, self.date)?;
        Ok(())
    }
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CLI Version:\t{}\n", self.version)?;
        if let Some(commit_info) = &self.commit_info {
            if let Some(tag_info) = &commit_info.commit_tag_info {
                write!(f, "Latest tag:\t{}", tag_info.last_tag)?;

                if let Some(commits_since) = tag_info.commits_since {
                    write!(f, "+{}", commits_since)?;
                }

                write!(f, "\n")?;
            }

            write!(f, "Commit hash:\t{}\n", commit_info.short_hash)?;
            write!(f, "Commit date:\t{}\n", commit_info.date)?;
        }
        Ok(())
    }
}

impl From<VersionInfo> for clap::builder::Str {
    fn from(value: VersionInfo) -> Self {
        value.into()
    }
}

pub(crate) fn nebu_version() -> VersionInfo {
    let commit_info = option_env!("NEBU_COMMIT_HASH").map(|hash| CommitInfo {
        hash: hash,
        short_hash: option_env!("NEBU_SHORT_HASH").unwrap(),
        date: option_env!("NEBU_COMMIT_DATE").unwrap(),
        commit_tag_info: option_env!("NEBU_LAST_TAG").map(|last_tag| CommitTagInfo {
            last_tag,
            commits_since: option_env!("NEBU_LAST_TAG_DISTANCE"),
        }),
    });

    VersionInfo {
        version: env!("CARGO_PKG_VERSION").into(),
        commit_info: commit_info,
    }
}
