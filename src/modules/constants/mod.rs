use std::process::exit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub version: i8,
    pub default_registry: String,
}

pub struct Constants {
    pub app_dir: String,
    pub app_config_path: String,
    pub registries_dir: String,
    pub config_dir_name: String,
    pub config_name: String,
}

impl Constants {
    pub fn get_all() -> Constants {
        let env_home_dir: String = if let Some(home) = std::env::home_dir() {
            home.as_path().to_str().unwrap().to_string()
        } else {
            eprintln!("The user's home directory was not found. Exiting...");
            exit(1)
        };

        let app_dir = format!("{}/{}", env_home_dir, "myra");
        let app_config_path = format!("{}/{}", app_dir, "myra.json");

        let config_dir_name: String = ".myra".to_string();
        let config_name: String = "myra.json".to_string();

        let registries_dir: String = format!("{}/{}", app_dir, "registries");

        Constants {
            app_dir,
            app_config_path,
            registries_dir,
            config_dir_name,
            config_name,
        }
    }
}
