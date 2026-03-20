use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

mod java;

pub enum SupportedLanguages {
    Java,
}

impl SupportedLanguages {
    pub fn as_str(&self) -> &str {
        match self {
            SupportedLanguages::Java => "java"
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BaseSettings {
    #[serde(rename = "$schemeVersion")]
    pub scheme_version: u8,

    #[serde(rename = "$language")]
    pub language: String,
}

pub fn load<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&contents)?)
}