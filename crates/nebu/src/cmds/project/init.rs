use dialoguer::{Confirm, theme::ColorfulTheme};
use owo_colors::OwoColorize;

pub fn run(_global_args: Box<crate::GlobalArgs>, project_args: super::ProjectArgs) {
    tracing::trace!("running project init command");

    tracing::info!("Using template repository: {}", &project_args.template_repo);

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
