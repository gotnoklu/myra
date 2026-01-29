use std::{
    fs,
    io::{self, Write},
    path, process,
};

use serde::{Deserialize, Deserializer, Serialize};

use crate::core::file_system::copy_fs_objects;

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let option_result = Option::deserialize(deserializer)?;
    Ok(option_result.unwrap_or_default())
}

fn get_abs_config_path(dir: &String, config_file: &String) -> String {
    let path_buffer = path::absolute(&dir).unwrap().join(&config_file);
    path_buffer.to_str().unwrap().to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub name: String,
    pub path: String,
}

impl Template {
    pub fn new(name: &String, path: &String) -> Self {
        Self {
            name: name.clone(),
            path: path.clone(),
        }
    }

    pub fn create_config(&self, config: &TemplateConfig) -> Result<(), io::Error> {
        let file_writer = fs::File::create(&self.path).unwrap();
        let _ = serde_json::to_writer(file_writer, config).unwrap();
        Ok(())
    }

    pub fn load_config(&self) -> TemplateConfig {
        let resolved_path = format!("{}/{}", &self.path, "edna.config.json");
        let config_exists = fs::exists(&resolved_path).unwrap();

        if !config_exists {
            println!("{}, {}", &resolved_path, &self.path);
            let mut file = fs::File::create_new(&resolved_path).unwrap();
            file.write_all(
                format!(
                    r#"{{
"target": "project",
"author": "",
"name": "{}",
"version": "1.0.0",
"exclude_config": true,
"exclude_paths": [],
"scripts": []
}}
        "#,
                    &self.path.split("/").last().unwrap()
                )
                .as_bytes(),
            )
            .unwrap();
        }

        let file = fs::File::open(&resolved_path).unwrap();
        let mut config: TemplateConfig = serde_json::from_reader(file).unwrap();

        // if config.target != "project" {
        //     eprintln!(
        //         "Invalid template config for {}. The variant must be `project`",
        //         &resolved_path
        //     );
        //     process::exit(1);
        // }

        if config.exclude_config {
            config.exclude_paths.push(resolved_path);
        }

        config
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryConfig {
    pub templates: Vec<Template>,
}

pub struct Registry {
    pub root_dir: String,
    pub config_file: String,
}

impl Registry {
    pub fn new(root_dir: String, config_file: String) -> Self {
        Self {
            root_dir,
            config_file,
        }
    }

    pub fn create_config(&self, config: &RegistryConfig) -> Result<(), io::Error> {
        let config_path = get_abs_config_path(&self.root_dir, &self.config_file);

        if !fs::exists(&self.root_dir).unwrap() {
            let _ = fs::create_dir_all(&self.root_dir);
        }

        if !fs::exists(&config_path).unwrap() {
            let file_writer = fs::File::create(config_path).unwrap();
            let _ = serde_json::to_writer(file_writer, config).unwrap();
        }

        Ok(())
    }

    pub fn load_config(&self) -> RegistryConfig {
        let file = fs::File::open(get_abs_config_path(&self.root_dir, &self.config_file)).unwrap();
        let registry: RegistryConfig = serde_json::from_reader(file).unwrap();
        registry
    }

    pub fn add_template(&self, template: &Template) -> Result<(), io::Error> {
        let mut config = self.load_config();
        config.templates.push(template.clone());
        let file_writer =
            fs::File::create(get_abs_config_path(&self.root_dir, &self.config_file)).unwrap();
        let _ = serde_json::to_writer(file_writer, &config).unwrap();

        Ok(())
    }

    pub fn create_template(
        &self,
        template: &Template,
        source: &String,
        config: &TemplateConfig,
    ) -> Result<(), io::Error> {
        let template_exists = fs::exists(&template.path).unwrap();
        if template_exists {
            eprintln!("The template \"{}\" already exists!", &template.path);
            process::exit(1);
        }

        if source == "" {
            let _ = fs::create_dir(&template.path).unwrap();
        } else {
            let _ = copy_fs_objects(source, &template.path, &config.exclude_paths);
        }

        let _ = template.create_config(config);

        let _ = self.add_template(&Template {
            name: config.name.clone(),
            path: template.path.to_string(),
        });

        Ok(())
    }

    pub fn get_templates(&self) -> Vec<Template> {
        let config = self.load_config();

        let mut registered_templates: Vec<Template> = Vec::new();
        registered_templates.push(Template {
            name: String::from("(No template)"),
            path: String::from(""),
        });

        for entry in config.templates {
            let metadata = fs::metadata(&entry.path).unwrap();
            if metadata.is_dir() {
                registered_templates.push(entry);
            }
        }

        registered_templates
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddedRegistry {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryManagerConfig {
    pub registries: Vec<AddedRegistry>,
}

pub struct RegistryManager {
    pub root_dir: String,
    pub config_file: String,
    pub registries: Vec<AddedRegistry>,
}

impl RegistryManager {
    pub fn new(root_dir: String, config_file: String) -> Self {
        Self {
            root_dir,
            config_file,
            registries: Vec::new(),
        }
    }

    pub fn create_config(&self, config: &RegistryManagerConfig) -> Result<(), io::Error> {
        let config_path = get_abs_config_path(&self.root_dir, &self.config_file);

        if !fs::exists(&self.root_dir).unwrap() {
            let _ = fs::create_dir_all(&self.root_dir);
        }

        if !fs::exists(&config_path).unwrap() {
            let file_writer = fs::File::create(config_path).unwrap();
            let _ = serde_json::to_writer(file_writer, config).unwrap();
        }

        Ok(())
    }

    pub fn load_config(&mut self) -> RegistryManagerConfig {
        let config_path = get_abs_config_path(&self.root_dir, &self.config_file);

        if !fs::exists(&config_path).unwrap() {
            let _ = self.create_config(&RegistryManagerConfig {
                registries: Vec::new(),
            });
        }

        let file = fs::File::open(&config_path).unwrap();
        let config: RegistryManagerConfig = serde_json::from_reader(file).unwrap();
        self.registries = config.registries.clone();
        config
    }

    pub fn registry_added(&mut self, registry: &AddedRegistry) -> bool {
        let config = self.load_config();
        config
            .registries
            .iter()
            .any(|r| r.path == registry.path && r.name == registry.name)
    }

    pub fn registry_exists(&mut self, registry: &AddedRegistry) -> bool {
        fs::exists(&registry.path).unwrap()
    }

    pub fn add_registry(&mut self, registry: &AddedRegistry) -> Result<(), io::Error> {
        if !self.registry_added(registry) {
            let mut config = self.load_config();
            config.registries.push(registry.clone());
            let file_writer =
                fs::File::create(get_abs_config_path(&self.root_dir, &self.config_file)).unwrap();
            let _ = serde_json::to_writer(file_writer, &config).unwrap();
        }

        if !self.registry_exists(&registry) {
            let registry = Registry::new(registry.path.clone(), String::from("edna.registry.json"));
            registry.load_config();
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateConfig {
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub author: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub version: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub description: String,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub exclude_paths: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub scripts: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub exclude_config: bool,
}
