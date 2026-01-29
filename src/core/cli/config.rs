use clap::ArgMatches;

use crate::core::registry::config::{Registry, RegistryManager};

pub struct CliMetadata<'a> {
    pub registry: &'a Registry,
    pub registry_manager: &'a RegistryManager,
}

pub struct CliParserOptions<'a> {
    pub metadata: &'a CliMetadata<'a>,
    pub matches: &'a ArgMatches,
}
