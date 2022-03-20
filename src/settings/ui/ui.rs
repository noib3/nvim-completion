use serde::{Deserialize, Serialize};

use super::{details, hint, menu};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiSettings {
    #[serde(default)]
    pub menu: menu::MenuSettings,

    #[serde(default)]
    pub details: details::DetailsSettings,

    #[serde(default)]
    pub hint: hint::HintSettings,
}

impl Default for UiSettings {
    fn default() -> Self {
        UiSettings {
            menu: menu::MenuSettings::default(),
            details: details::DetailsSettings::default(),
            hint: hint::HintSettings::default(),
        }
    }
}
