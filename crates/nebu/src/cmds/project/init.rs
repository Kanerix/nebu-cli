use dialoguer::{Confirm, theme::ColorfulTheme};
use nebu_cache::{CacheManager, RepoCache};
// use nebu_cache::Cache;
use owo_colors::OwoColorize;

use crate::error::CommandError;

pub async fn run(
    global_args: Box<crate::GlobalArgs>,
    project_args: super::ProjectArgs,
) -> crate::error::CommandResult {
    tracing::trace!("running project init command");

    let cache_path = global_args.cache_path;
    let cache_repo = RepoCache::new(&project_args.template_repo);
    let mut cache_manager = CacheManager::new(cache_path, cache_repo);
    cache_manager.refresh().map_err(CommandError::from_err)?;

    // let theme = ColorfulTheme::default();
    // let _has_batch = Confirm::with_theme(&theme)
    //     .with_prompt("Do you want to add a python batch job?")
    //     .interact();
    // let _has_backend = Confirm::with_theme(&theme)
    //     .with_prompt("Do you want to add a Fast API backend?")
    //     .interact();
    // let _has_frontend = Confirm::with_theme(&theme)
    //     .with_prompt("Do you want to add a NextJS frontend?")
    //     .interact();

    println!(
        "Project initialized with template from: {}",
        &project_args.template_repo.red().bold()
    );

    Ok(())
}
