pub mod types;

use std::{fs, io, process};

use crate::modules::template::types::Template;
use types::Registry;

use crate::{
    core::{file_system::copy_fs_objects, git::GitRepo},
    modules::core::get_constants,
};

impl Registry {
    pub fn new(name: String, root_dir: String) -> Self {
        Self {
            name,
            path: root_dir,
        }
    }

    pub fn add_template(&self, template: &Template, source: &String) -> Result<(), io::Error> {
        let template_exists = fs::exists(&template.path).unwrap();
        if template_exists {
            eprintln!("The template \"{}\" already exists!", &template.path);
            process::exit(1);
        }

        if source.is_empty() {
            fs::create_dir(&template.path).unwrap();
        } else {
            let _ = copy_fs_objects(source, &template.path, &vec![]);
        }

        let constants = get_constants();

        let _ = template.create_config(constants.myra_config_name, None);

        if template.initialise_git {
            GitRepo::init(Some(template.path.as_str()), None);
        }

        Ok(())
    }

    pub fn get_templates(&self) -> Vec<Template> {
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
}
