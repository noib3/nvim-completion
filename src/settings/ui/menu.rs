use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use super::borders;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MenuSettings {
    #[serde(default)]
    pub anchor: MenuAnchor,

    #[serde(default = "default_menu_autoshow")]
    pub autoshow: bool,

    #[serde(default)]
    pub max_height: Option<NonZeroU32>,

    #[serde(default)]
    pub borders: super::borders::BordersSettings,
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            anchor: MenuAnchor::default(),
            autoshow: default_menu_autoshow(),
            max_height: Option::default(),
            borders: borders::BordersSettings::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MenuAnchor {
    Cursor,
    Match,
}

impl Default for MenuAnchor {
    fn default() -> Self {
        MenuAnchor::Cursor
    }
}

fn default_menu_autoshow() -> bool {
    true
}
