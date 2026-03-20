use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::languages::BaseSettings;

mod schemes;

#[derive(Debug, Deserialize)]
pub struct JavaSettings {

    #[serde(flatten)]
    pub base: BaseSettings,
}

impl JavaSettings {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let settings = serde_json::from_str(&contents)?;
        Ok(settings)
    }
}