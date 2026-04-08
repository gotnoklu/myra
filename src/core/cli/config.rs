use crate::modules::project::cli::{match_project_cli_args, register_project_cli_args};
use crate::modules::template::cli::{match_template_cli_args, register_template_cli_args};

use clap::{ArgMatches, command};

pub fn register_cli_args() -> ArgMatches {
    command!()
        .subcommand(register_template_cli_args())
        .subcommand(register_project_cli_args())
        .get_matches()
}

pub fn match_cli_args(matches: &ArgMatches) {
    if let Some(matched) = matches.subcommand_matches("templates") {
        match_template_cli_args(&matched)
    } else if let Some(matched) = matches.subcommand_matches("projects") {
        match_project_cli_args(&matched)
    }
}
