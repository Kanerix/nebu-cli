use dialoguer::{Confirm, theme::ColorfulTheme};
use miette::IntoDiagnostic;
use owo_colors::OwoColorize;

use crate::error::CommandError;

pub fn run(
    global_args: Box<crate::GlobalArgs>,
    project_args: super::ProjectArgs,
) -> crate::error::Result {
    tracing::trace!("running project init command");

    let cache_path = global_args.home_path.join("cache/repo");
    tracing::trace!("using cache path: {}", cache_path.display());

    if !cache_path.exists() {
        tracing::trace!("cache dir does not exists, creating a new");
        std::fs::create_dir_all(&cache_path).map_err(|e| {
            CommandError::from_err(e)
                .with_command("project init")
                .with_source_context("creating cache directory", 0..0)
        }).into_diagnostic()?;
    } else {
        tracing::trace!("cache directory already exists: {}", cache_path.display());
    }

    let template_repo = project_args.template_repo;

    tracing::trace!("cloning template repository into cache: {}", template_repo);
    git2::Repository::clone(&template_repo, &cache_path).into_diagnostic()?;

    let theme = ColorfulTheme::default();
    let _has_batch = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a python batch job?")
        .interact();
    let _has_backend = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a Fast API backend?")
        .interact();
    let _has_frontend = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a NextJS frontend?")
        .interact();

    println!(
        "Project initialized with template from: {}",
        &template_repo.red().bold()
    );

    Ok(())
}
