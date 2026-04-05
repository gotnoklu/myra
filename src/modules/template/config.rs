use super::types::Template;
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

    pub fn create_config(&self, config_name: String) -> Result<(), std::io::Error> {
        let file_writer = fs::File::create(path::Path::new(&self.path).join(config_name)).unwrap();

        let _ = serde_json::to_writer(file_writer, self).unwrap();

        Ok(())
    }

    pub fn from(path: String) -> Template {
        let config_exists = fs::exists(&path).unwrap();

        if !config_exists {
            panic!("Invalid template folder")
        }

        let file = fs::File::open(&path).unwrap();
        let config: Template = serde_json::from_reader(file).unwrap();

        Template { ..config }
    }

    pub fn load_config(&self, config_name: String) -> Template {
        let resolved_path = format!("{}/{}", &self.path, config_name);
        let config_exists = fs::exists(&resolved_path).unwrap();

        if !config_exists {
            println!("{}, {}", &resolved_path, &self.path);
            let mut file = fs::File::create_new(&resolved_path).unwrap();
            file.write_all(
                format!(
                    r#"{{
"name": "{}",
"version": "1.0.0",
"author": "",
"description": ""
}}
        "#,
                    &self.path.split("/").last().unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
        }

        let file = fs::File::open(&resolved_path).unwrap();
        let config: Template = serde_json::from_reader(file).unwrap();

        config
    }
}
