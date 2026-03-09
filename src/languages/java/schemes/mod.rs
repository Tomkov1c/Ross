use serde::Deserialize;
mod v1;

#[derive(Debug, Deserialize)]
#[serde(tag = "schemeVersion")]
pub enum VersionedSettings {

    #[serde(rename = "1")]
    V1(v1::Settings),
}