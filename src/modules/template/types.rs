use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let option_result = Option::deserialize(deserializer)?;
    Ok(option_result.unwrap_or_default())
}

#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub path: String,
    pub initialise_git: bool,
    pub author: String,
    pub version: String,
    pub description: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct TemplateActionsConfig {
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub install: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct TemplatePipelinesConfig {}

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
    pub workspaces: Vec<String>,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub actions: TemplateActionsConfig,
    #[serde(default, deserialize_with = "deserialize_optional_field")]
    pub pipelines: TemplatePipelinesConfig,
}
