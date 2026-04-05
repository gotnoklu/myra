use std::path;

pub struct Constants {
    pub myra_home_dir: String,
    pub myra_templates_dir: String,
    pub myra_config_name: String,
}

pub fn get_constants() -> Constants {
    let env_home_dir: String = if let Some(home) = std::env::home_dir() {
        home.as_path().to_str().unwrap().to_string()
    } else {
        path::absolute("./").unwrap().to_str().unwrap().to_string()
    };

    let myra_home_dir: String = format!("{}/{}", env_home_dir, ".myra");
    let myra_templates_dir: String = format!("{}/{}", myra_home_dir, "templates");
    let myra_config_name: String = "myra.json".to_string();

    Constants {
        myra_home_dir,
        myra_templates_dir,
        myra_config_name,
    }
}

pub mod cli_theme;
