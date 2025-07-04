use dialoguer::{Confirm, theme::ColorfulTheme};
use owo_colors::OwoColorize;

pub fn run(global_args: Box<crate::GlobalArgs>, project_args: super::ProjectArgs) {
    tracing::trace!("running project init command");

    let cache_path = global_args.home_path.join("cache/repo");
    tracing::trace!("Using cache path: {}", cache_path.display());

    if !cache_path.exists() {
        if let Err(e) = std::fs::create_dir_all(&cache_path) {
            tracing::error!("Failed to create cache directory: {}", e);
            return;
        }
    } else {
        tracing::trace!("Cache directory already exists: {}", cache_path.display());
    }

    tracing::trace!("Trying to clone template repository: {}", project_args.template_repo);
    let repo = git2::Repository::clone(
        &project_args.template_repo,
        &cache_path,
    );

    if let Err(e) = repo {
        tracing::error!("Failed to clone repository: {}", e);
    } else {
        tracing::info!("Successfully cloned repository to: {}", cache_path.display());
    }

    let theme = ColorfulTheme::default();
    let _has_batch = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a python batch job?")
        .interact()
        .unwrap();
    let _has_backend = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a Fast API backend?")
        .interact()
        .unwrap();
    let _has_frontend = Confirm::with_theme(&theme)
        .with_prompt("Do you want to add a NextJS frontend?")
        .interact()
        .unwrap();

    println!(
        "Project initialized with template from: {}",
        &project_args.template_repo.red().bold()
    );
}
