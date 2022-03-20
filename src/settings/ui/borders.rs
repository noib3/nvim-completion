use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BordersSettings {
    #[serde(default)]
    enable: bool,

    #[serde(default)]
    style: BorderType,
}

impl Default for BordersSettings {
    fn default() -> Self {
        BordersSettings {
            enable: bool::default(),
            style: BorderType::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BorderType {
    None,
    Single,
    Double,
    Rounded,
    Solid,
    Shadow,
}

impl Default for BorderType {
    fn default() -> Self {
        BorderType::None
    }
}
