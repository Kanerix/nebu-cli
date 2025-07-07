use tokio::fs::create_dir;

use crate::error::Result;

use std::path::PathBuf;

mod error;

pub async fn init_cache_folder(home_path: PathBuf) -> Result<()> {
    if home_path.exists() && home_path.is_dir() {
        return Ok(());
    }

    create_dir(&home_path).await?;

    return Ok(());
}
