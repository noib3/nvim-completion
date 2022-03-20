use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BorderSettings {
    #[serde(default)]
    enable: bool,

    #[serde(default)]
    style: BorderType,
}

impl Default for BorderSettings {
    fn default() -> Self {
        BorderSettings {
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
