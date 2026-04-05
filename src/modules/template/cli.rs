use super::types::Template;
use crate::core::printer::print_success_text;
use crate::modules::core::{cli_theme::CliTheme, get_constants};
use crate::modules::registry::types::Registry;
use clap::ArgMatches;
use dialoguer::{Input, Select};
use std::path;

pub fn handle_create_new_template(matches: &ArgMatches) {
    let template_cmd = matches.subcommand_matches("template").unwrap();
    let constants = get_constants();

    let template_name = if let Some(name) = template_cmd.get_one::<String>("name") {
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

    let template_author = if let Some(author) = template_cmd.get_one::<String>("author") {
        author
    } else {
        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Enter the template's author")
            .with_post_completion_text("Template Author")
            .allow_empty(true)
            .show_default(true)
            .default(String::from(whoami::realname()))
            .interact()
            .unwrap();

        &input.to_string()
    };

    let template_version = if let Some(version) = template_cmd.get_one::<String>("version") {
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

    let template_description =
        if let Some(description) = template_cmd.get_one::<String>("description") {
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

    println!("Git init: {:?}", template_cmd.get_one::<bool>("init_git"));

    let mut initialise_git: bool = false;
    if let Some(value) = template_cmd.get_one::<bool>("init_git") {
        if value == &true {
            initialise_git = value.clone();
        } else {
            let input: usize = Select::with_theme(&CliTheme::default())
                .with_prompt("Initialise Git when creating project")
                .default(0)
                .items(&vec![String::from("Yes"), String::from("No")])
                .interact()
                .unwrap();

            initialise_git = input == 0;
        }
    }

    let template_output = if let Some(output) = template_cmd.get_one::<String>("output") {
        &path::absolute(format!("{}/{}", constants.myra_templates_dir, output))
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    } else {
        let default_path = String::from(format!(
            "{}/{}",
            constants.myra_templates_dir, template_name
        ));

        let input: String = Input::with_theme(&CliTheme::default())
            .with_prompt("Where in the templates directory to create it")
            .with_post_completion_text("Template Output")
            .show_default(true)
            .default(default_path.clone())
            .interact()
            .unwrap();

        if input == default_path {
            &input.to_string()
        } else {
            &path::absolute(format!("{}/{}", constants.myra_templates_dir, input))
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        }
    };

    let template_source = if let Some(source) = template_cmd.get_one::<String>("source") {
        if source == "" {
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
            .with_prompt("The folder to be used when creating the template")
            .with_post_completion_text("Template Source")
            .allow_empty(true)
            .interact()
            .unwrap();

        if input == "" {
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
        initialise_git: initialise_git,
    };

    let registry = Registry::new("templates".to_string(), constants.myra_templates_dir);

    let _ = registry.add_template(&template, &template_source);

    print_success_text("Template created!");
}
