use super::types::Template;
use crate::core::printer::{print_action, print_blocked_text, print_success_text};
use crate::modules::core::{cli_theme::CliTheme, get_constants};
use crate::modules::registry::types::Registry;
use clap::{Arg, ArgMatches, Command, builder::BoolValueParser};
use dialoguer::{Input, Select};
use std::path;

pub fn register_template_cli_args() -> Command {
    Command::new("templates")
        .about("Commands for the template resource")
        .subcommand(
            Command::new("add")
                .about("Creates a new template")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .help("The name of the template to be created"),
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
                        .short('n')
                        .long("name")
                        .help("The name of the template to be deleted"),
                ),
        )
        .subcommand(
            Command::new("cp")
                .about("Copies an existing template into a directory")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .help("The name of the template to be copied"),
                )
                .arg(
                    Arg::new("destination")
                        .short('d')
                        .long("dest")
                        .help("The destination where the template will be copied"),
                ),
        )
        .subcommand(Command::new("list").about("Lists all templates"))
}

pub fn match_template_cli_args(matches: &ArgMatches) {
    if let Some(matched) = matches.subcommand_matches("add") {
        handle_create_new_template(&matched);
    }
}

pub fn handle_create_new_template(matches: &ArgMatches) {
    let constants = get_constants();

    print_blocked_text("myra", "Create a new template");

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

    print_success_text("Template created!");
}
