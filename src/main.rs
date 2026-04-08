mod core;
mod modules;

use crate::core::cli::config::{match_cli_args, register_cli_args};

fn main() {
    let arg_matches = register_cli_args();

    match_cli_args(&arg_matches);
}
