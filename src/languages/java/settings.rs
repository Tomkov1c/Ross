use serde::Deserialize;
use crate::settings::BaseSettings;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavaSettings {
    #[serde(flatten)]
    pub base: BaseSettings,
}

impl Settings {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let settings = serde_json::from_str(&contents)?;
        Ok(settings)
    }
}