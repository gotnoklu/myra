use crate::core::printer::{
    print_action, print_blocked_text, print_error_text, print_list_item, print_success_text,
};
use crate::modules::{cli::theme::CliTheme, constants::Constants, registry::config::Registry};
use clap::{Arg, ArgMatches, Command};
use dialoguer::Input;
use std::fs;
use std::process::exit;

pub fn register_registry_cli_args() -> Command {
    Command::new("registry")
        .about("Commands for the registry resource")
        .subcommand(
            Command::new("add")
                .about("Creates a new registry")
                .arg(
                    Arg::new("name")
                        .help("The name of the registry to be created")
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("desc")
                        .help("The description of the registry"),
                )
                .arg(
                    Arg::new("author")
                        .short('a')
                        .long("author")
                        .help("The author of the registry"),
                )
               ,
        )
        .subcommand(
            Command::new("rm")
                .about("Removes an existing registry")
                .arg(
                    Arg::new("name")
                        .help("The name of the registry to be deleted")
                        .index(1),
                ),
        )
        .subcommand(Command::new("ls").about("Lists all registries"))
        .subcommand(Command::new("origin").about("Sets the default registry"))
        .arg(
            Arg::new("name")
                .help("The name of the registry to be used as the default. Can be an existing or non-existing registry.")
                .index(1),
        )
}

pub fn match_registry_cli_args(matches: &ArgMatches) {
    if let Some(matched) = matches.subcommand_matches("add") {
        handle_create_new_registry(&matched)
    }
    if let Some(matched) = matches.subcommand_matches("rm") {
        handle_remove_registry(&matched)
    }
    if matches.subcommand_matches("ls").is_some() {
        handle_list_registries()
    }
    if matches.subcommand_matches("origin").is_some() {
        handle_set_default_registry()
    }
}

pub fn handle_set_default_registry() {}

pub fn handle_list_registries() {
    print_blocked_text("myra", "List registries");

    let registries = Registry::get_all();

    if registries.len() == 0 {
        return print_success_text("No registries found.", true);
    }

    let mut position = 1;
    for registry in registries {
        print_list_item(
            &format!("{}.", position.to_string().as_str()),
            &registry.name,
        );
        position += 1;
    }

    print_success_text("Registries listed.", false);
}

pub fn handle_remove_registry(matches: &ArgMatches) {
    let constants = Constants::get_all();

    print_blocked_text("myra", "Remove registry");

    let mut registry_name = String::new();

    let registry_path = if let Some(name) = matches.get_one::<String>("name") {
        registry_name = name.clone();
        format!("{}/{}", constants.registries_dir, name)
    } else {
        print_error_text("The registry to be removed was not found", true);
        exit(0)
    };

    if fs::exists(&registry_path).unwrap() {
        print_action(
            "REMOVE",
            &format!("Registry '{}' found. Removing...", registry_name).as_str(),
        );
        fs::remove_dir_all(&registry_path).unwrap();

        print_success_text("The registry was removed successfully.", false);
    } else {
        print_error_text(
            &format!("The registry '{}' was not found. Exiting...", registry_name).as_str(),
            true,
        );
        exit(0)
    }
}

pub fn handle_create_new_registry(matches: &ArgMatches) {
    let constants = Constants::get_all();

    print_blocked_text("myra", "Create a new registry");

    let registry_name = if let Some(name) = matches.get_one::<String>("name") {
        name
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the registry's name (Required)")
            .with_post_completion_text("Registry name")
            .allow_empty(false)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let registry_author = if let Some(author) = matches.get_one::<String>("author") {
        author
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the registry's author")
            .with_post_completion_text("Registry Author")
            .allow_empty(true)
            .show_default(true)
            .default(whoami::realname())
            .interact()
            .unwrap();

        &input.to_string()
    };

    let registry_description = if let Some(description) = matches.get_one::<String>("description") {
        description
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the registry's description")
            .with_post_completion_text("Registry Description")
            .allow_empty(true)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let mut registry = Registry {
        name: registry_name.to_string(),
        author: registry_author.to_string(),
        description: registry_description.to_string(),
        path: registry_name.to_string(),
    };

    match registry.create() {
        Ok(_) => print_success_text("Registry created!", false),
        Err(message) => print_error_text(message, false),
    }
}
