use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub language: String,
}