use super::types::Template;
use crate::core::file_system::copy_fs_objects;
use crate::core::printer::{
    print_action, print_blocked_text, print_error_text, print_list_item, print_success_text,
};
use crate::modules::core::{cli_theme::CliTheme, get_constants};
use crate::modules::registry::types::Registry;
use clap::{Arg, ArgMatches, Command, builder::BoolValueParser};
use dialoguer::{Input, Select};
use std::process::exit;
use std::thread::current;
use std::{env, fs, path};

pub fn register_template_cli_args() -> Command {
    Command::new("templates")
        .about("Commands for the template resource")
        .subcommand(
            Command::new("init").about("Initialises a template config in the current directory"),
        )
        .subcommand(
            Command::new("add")
                .about("Creates a new template")
                .arg(
                    Arg::new("current_directory")
                        .help("Adds a template from the config in the current directory")
                        .short('c')
                        .long("dir")
                        .num_args(0)
                        .value_parser(BoolValueParser::new()),
                )
                .arg(
                    Arg::new("name")
                        .help("The name of the template to be created")
                        .index(1),
                )
                .arg(
                    Arg::new("description")
                        .short('d')
                        .long("desc")
                        .help("The description of the template"),
                )
                .arg(
                    Arg::new("version")
                        .short('v')
                        .long("version")
                        .help("The version of the template"),
                )
                .arg(
                    Arg::new("author")
                        .short('a')
                        .long("author")
                        .help("The author of the template"),
                )
                .arg(
                    Arg::new("init_git_repo")
                        .short('g')
                        .long("git")
                        .help("Initialise a Git repo for the template")
                        .num_args(0)
                        .value_parser(BoolValueParser::new()),
                )
                .arg(
                    Arg::new("source")
                        .short('s')
                        .long("source")
                        .help("The path to the directory to be used as the template"),
                ),
        )
        .subcommand(
            Command::new("rm")
                .about("Removes an existing template")
                .arg(
                    Arg::new("name")
                        .help("The name of the template to be deleted")
                        .index(1),
                ),
        )
        .subcommand(
            Command::new("cp")
                .about("Copies an existing template into a directory")
                .arg(
                    Arg::new("name")
                        .help("The name of the template to be copied")
                        .index(1),
                )
                .arg(
                    Arg::new("destination")
                        .help("The destination where the template will be copied")
                        .index(2),
                ),
        )
        .subcommand(Command::new("ls").about("Lists all templates"))
}

pub fn match_template_cli_args(matches: &ArgMatches) {
    if matches.subcommand_matches("init").is_some() {
        handle_init_template_config()
    }
    if let Some(matched) = matches.subcommand_matches("add") {
        handle_create_new_template(&matched)
    }
    if let Some(matched) = matches.subcommand_matches("cp") {
        handle_copy_template(&matched)
    }
    if let Some(matched) = matches.subcommand_matches("rm") {
        handle_remove_template(&matched)
    }
    if matches.subcommand_matches("ls").is_some() {
        handle_list_templates()
    }
}

pub fn handle_init_template_config() {
    let constants = get_constants();

    let current_dir = env::current_dir().unwrap();

    let template = Template {
        name: current_dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        version: "1.0.0".to_string(),
        author: whoami::realname(),
        description: "".to_string(),
        path: current_dir.to_str().unwrap().to_string(),
        initialise_git: false,
    };

    match template.create_config(constants.myra_config_name, None) {
        Ok(_) => (),
        Err(_) => print_error_text("The template config was not created.", false),
    };
}

pub fn handle_list_templates() {
    let constants = get_constants();

    print_blocked_text("myra", "List templates");

    let mut position = 1;
    for entry in fs::read_dir(constants.myra_templates_dir).unwrap() {
        let entry = entry.unwrap();
        let object_type = entry.file_type().unwrap();

        if object_type.is_dir() {
            print_list_item(
                &format!("{}.", position.to_string().as_str()),
                entry.file_name().to_str().unwrap(),
            );

            position += 1;
        }
    }

    if position == 1 {
        print_success_text("No templates found.", true);
    } else {
        print_success_text("Templates listed.", false);
    }
}

pub fn handle_copy_template(matches: &ArgMatches) {
    let constants = get_constants();

    print_blocked_text("myra", "Copy template");

    let mut template_name = String::new();

    let template_path = if let Some(name) = matches.get_one::<String>("name") {
        template_name = name.clone();
        format!("{}/{}", constants.myra_templates_dir, name)
    } else {
        print_error_text("The template to be removed was not found", true);
        exit(0)
    };

    let destination = if let Some(dest) = matches.get_one::<String>("destination") {
        path::absolute(dest).unwrap().to_str().unwrap().to_string()
    } else {
        print_error_text("The destination for the copy operation was not found", true);
        exit(0)
    };

    print_action(
        "COPY",
        &format!("Copying '{}' into '{}'...", template_name, destination).as_str(),
    );

    let _ = copy_fs_objects(template_path, destination, &vec![]);

    print_success_text("The template was copied successfully.", false);
}

pub fn handle_remove_template(matches: &ArgMatches) {
    let constants = get_constants();

    print_blocked_text("myra", "Remove template");

    let mut template_name = String::new();

    let template_path = if let Some(name) = matches.get_one::<String>("name") {
        template_name = name.clone();
        format!("{}/{}", constants.myra_templates_dir, name)
    } else {
        print_error_text("The template to be removed was not found", true);
        exit(0)
    };

    if fs::exists(&template_path).unwrap() {
        print_action(
            "REMOVE",
            &format!("Template '{}' found. Removing...", template_name).as_str(),
        );
        fs::remove_dir_all(&template_path).unwrap();

        print_success_text("The template was removed successfully.", false);
    } else {
        print_error_text(
            &format!("The template '{}' was not found. Exiting...", template_name).as_str(),
            true,
        );
        exit(0)
    }
}

pub fn handle_create_new_template(matches: &ArgMatches) {
    let constants = get_constants();

    print_blocked_text("myra", "Create a new template");

    let init_from_dir = if let Some(init) = matches.get_one::<bool>("current_directory") {
        init.clone()
    } else {
        false
    };

    if init_from_dir {
        let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();

        let config = Template::get_config(
            env::current_dir().unwrap().to_str().unwrap().to_string(),
            constants.myra_config_name,
        );

        let _ = copy_fs_objects(
            &current_path,
            format!("{}/{}", constants.myra_templates_dir, &config.name),
            &vec![],
        );

        return print_success_text("Template created!", false);
    }

    let template_name = if let Some(name) = matches.get_one::<String>("name") {
        name
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's name (Required)")
            .with_post_completion_text("Template name")
            .allow_empty(false)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_author = if let Some(author) = matches.get_one::<String>("author") {
        author
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's author")
            .with_post_completion_text("Template Author")
            .allow_empty(true)
            .show_default(true)
            .default(whoami::realname())
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_version = if let Some(version) = matches.get_one::<String>("version") {
        version
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's version")
            .with_post_completion_text("Template Version")
            .allow_empty(true)
            .show_default(true)
            .default(String::from("1.0.0"))
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_description = if let Some(description) = matches.get_one::<String>("description") {
        description
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's description")
            .with_post_completion_text("Template Description")
            .allow_empty(true)
            .interact()
            .unwrap();

        &input.to_string()
    };

    let mut initialise_git: bool = false;

    if let Some(value) = matches.get_one::<bool>("init_git_repo") {
        if value == &true {
            initialise_git = *value;
        } else {
            let input: usize = Select::with_theme(&CliTheme::default())
                .with_prompt("Initialise Git when creating project")
                .default(0)
                .items(&[String::from("Yes"), String::from("No")])
                .interact()
                .unwrap();

            initialise_git = input == 0;
        }
    }

    let template_output = &path::absolute(format!(
        "{}/{}",
        constants.myra_templates_dir, template_name
    ))
    .unwrap()
    .to_str()
    .unwrap()
    .to_string();

    let template_source = if let Some(source) = matches.get_one::<String>("source") {
        if source.is_empty() {
            &source.to_string()
        } else {
            &path::absolute(source)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("The folder to use when creating the template")
            .with_post_completion_text("Template Source")
            .allow_empty(true)
            .default("./".to_string())
            .interact()
            .unwrap();

        if input.is_empty() {
            &input.to_string()
        } else {
            &path::absolute(input).unwrap().to_str().unwrap().to_string()
        }
    };

    let template = Template {
        name: template_name.to_string(),
        version: template_version.to_string(),
        author: template_author.to_string(),
        description: template_description.to_string(),
        path: template_output.to_string(),
        initialise_git,
    };

    let registry = Registry::new("templates".to_string(), constants.myra_templates_dir);

    let _ = registry.add_template(&template, template_source);

    print_success_text("Template created!", false);
}
