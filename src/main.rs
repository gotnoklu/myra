mod core;

use core::cli::{
    config::{CliMetadata, CliParserOptions},
    core::{parse_cli_args, register_cli_args},
};

use std::path;

use crate::core::registry::config::{
    AddedRegistry, Registry, RegistryConfig, RegistryManager, RegistryManagerConfig,
};

fn main() {
    let home_dir = if let Some(home) = std::env::home_dir() {
        home.as_path().to_str().unwrap().to_string()
    } else {
        path::absolute("./").unwrap().to_str().unwrap().to_string()
    };

    let edna_home_dir = format!("{}/{}", home_dir, ".edna");
    let edna_registries_dir = format!("{}/{}", edna_home_dir, "registries");

    let mut registry_manager = RegistryManager::new(
        edna_registries_dir.clone(),
        String::from("edna.manager.json"),
    );

    let _ = registry_manager.load_config();

    let registry = Registry::new(
        format!("{}/{}", edna_registries_dir.clone(), "templates"),
        String::from("edna.registry.json"),
    );

    let _ = registry.create_config(&RegistryConfig {
        templates: Vec::new(),
    });

    let added_registry = AddedRegistry {
        name: String::from("templates"),
        path: registry.root_dir.clone(),
    };

    let _ = registry_manager.add_registry(&added_registry);

    let arg_matches = register_cli_args();

    parse_cli_args(CliParserOptions {
        metadata: &CliMetadata {
            registry: &registry,
            registry_manager: &registry_manager,
        },
        matches: &arg_matches,
    });
}
