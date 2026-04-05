mod core;
mod modules;

use crate::core::cli::config::{parse_cli_args, register_cli_args};

fn main() {
    let arg_matches = register_cli_args();

    parse_cli_args(&arg_matches);
}
