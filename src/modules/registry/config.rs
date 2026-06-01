use std::{fs, io, path::Path, process};
use whoami;

use crate::{
    core::{file_system::copy_fs_objects, git::GitRepo, printer::print_blocked_text},
    modules::{
        constants::{AppConfig, Constants},
        template::config::Template,
    },
};

pub struct Registry {
    pub name: String,
    pub author: String,
    pub description: String,
    pub path: String,
}

impl Registry {
    pub fn new(name: String, path: String) -> Self {
        Self {
            name,
            author: whoami::realname(),
            description: String::new(),
            path,
        }
    }

    pub fn get(name: &str) -> Registry {
        let constants = Constants::get_all();
        let registry_path = Path::join(Path::new(&constants.registries_dir), Path::new(name));

        if !fs::exists(&registry_path).unwrap() {
            panic!(
                "The registry {} does not exist.",
                &registry_path.file_name().unwrap().to_str().unwrap()
            );
        }

        Registry {
            name: registry_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            author: whoami::realname(),
            description: String::new(),
            path: registry_path.to_str().unwrap().to_owned(),
        }
    }

    pub fn get_default() -> Registry {
        let constants = Constants::get_all();
        let app_config_file = fs::File::open(&constants.app_config_path).unwrap();
        let app_config: AppConfig = serde_json::from_reader(app_config_file).unwrap();
        let default_registry = app_config.default_registry;
        Registry::get(default_registry.as_str())
    }

    pub fn get_all() -> Vec<Registry> {
        let constants = Constants::get_all();
        let mut registries = Vec::<Registry>::new();

        for entry in fs::read_dir(constants.registries_dir).unwrap() {
            let entry = entry.unwrap();
            let object_type = entry.file_type().unwrap();

            if object_type.is_dir() {
                registries.push(Registry {
                    name: String::from(entry.file_name().to_str().unwrap()),
                    author: whoami::realname(),
                    description: String::new(),
                    path: String::from(entry.path().to_str().unwrap()),
                });
            }
        }

        registries
    }

    pub fn get_all_templates(&self) -> Vec<Template> {
        let mut templates: Vec<Template> = Vec::new();

        for entry in fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let object_type = entry.file_type().unwrap();
            let object_path = entry.path();

            if object_type.is_dir()
                && let Some(path) = object_path.to_str()
            {
                let template = Template::from(String::from(path));
                templates.push(template);
            }
        }

        templates
    }

    pub fn create(&mut self) -> Result<(), &str> {
        let constants = Constants::get_all();
        let registry_home = Path::new(&constants.registries_dir);
        let registry_path = &Path::join(registry_home, Path::new(&self.path));

        if fs::exists(registry_path).unwrap() {
            return Err("The registry already exists.");
        }

        let _ = fs::create_dir(registry_path).unwrap();
        self.path = registry_path.to_str().unwrap().to_string();
        Ok(())
    }

    pub fn sync(&self) {}

    pub fn sync_all() {}

    pub fn link_remote_registry(&self) {}

    pub fn add_template(&self, template: &Template, source: &String) -> Result<(), io::Error> {
        let template_exists = fs::exists(&template.path).unwrap();
        if template_exists {
            eprintln!("The template \"{}\" already exists!", &template.path);
            process::exit(1);
        }

        if source.is_empty() {
            fs::create_dir(&template.path).unwrap();
        } else {
            let _ = copy_fs_objects(source, &template.path, None);
        }

        let constants = Constants::get_all();

        let _ = template.create_config(constants.config_name, None);

        if template.initialise_git {
            GitRepo::init(Some(template.path.as_str()), None);
        }

        Ok(())
    }
}
