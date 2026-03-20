use serde::Deserialize;

mod v1;
// mod v2;

#[derive(Debug, Deserialize)]
#[serde(tag = "schemeVersion")]
pub enum VersionedSettings {

    #[serde(rename = "1")]
    V1(v1::Settings),

    // #[serde(rename = "2")]
    // V2(v2::Settings),
}