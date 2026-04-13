use crate::modules::template::types::{TemplateActionsConfig, TemplatePipelinesConfig};

use super::types::{Template, TemplateConfig};
use std::{fs, io::Write, path};

impl Template {
    pub fn new(name: &String, path: &String) -> Self {
        Self {
            name: name.clone(),
            path: path.clone(),
            version: "1.0.0".to_string(),
            author: "".to_string(),
            description: "".to_string(),
            initialise_git: false,
        }
    }

    pub fn create_config(
        &self,
        config_name: String,
        config_path: Option<String>,
    ) -> Result<(), std::io::Error> {
        let file_writer = if let Some(config_path) = config_path {
            fs::File::create(path::Path::new(&config_path).join(config_name)).unwrap()
        } else {
            fs::File::create(path::Path::new(&self.path).join(config_name)).unwrap()
        };

        serde_json::to_writer(
            file_writer,
            &TemplateConfig {
                name: self.name.clone(),
                author: self.author.clone(),
                version: self.version.clone(),
                description: self.description.clone(),
                workspaces: vec![],
                actions: TemplateActionsConfig {
                    install: "".to_string(),
                },
                pipelines: TemplatePipelinesConfig {},
            },
        )
        .unwrap();

        Ok(())
    }

    pub fn from(path: String) -> Template {
        let config_exists = fs::exists(&path).unwrap();

        if !config_exists {
            panic!("Invalid template folder")
        }

        let file = fs::File::open(&path).unwrap();
        let config: TemplateConfig = serde_json::from_reader(file).unwrap();

        Template {
            name: config.name,
            author: config.author,
            description: config.description,
            version: config.version,
            path: path.clone(),
            initialise_git: fs::exists(path::absolute(format!("{}/{}", &path, ".git")).unwrap())
                .unwrap(),
        }
    }

    pub fn get_config(config_path: String, config_name: String) -> TemplateConfig {
        let resolved_path = format!("{}/{}", &config_path, config_name);
        let config_exists = fs::exists(&resolved_path).unwrap();

        if !config_exists {
            let mut file = fs::File::create_new(&resolved_path).unwrap();
            file.write_all(
                format!(
                    r#"{{
"name": "{}",
"version": "1.0.0",
"author": "",
"description": "",
"workspaces": [],
"actions": {{ "install": "" }},
"pipelines": {{}}
}}
        "#,
                    &resolved_path.split("/").last().unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
        }

        let file = fs::File::open(&resolved_path).unwrap();
        let config: TemplateConfig = serde_json::from_reader(file).unwrap();

        config
    }

    pub fn load_config(&self, config_name: String) -> TemplateConfig {
        Template::get_config(self.path.clone(), config_name)
    }
}
