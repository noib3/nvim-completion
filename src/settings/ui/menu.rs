use serde::Deserialize;
use std::num::NonZeroU32;

use super::border;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MenuSettings {
    #[serde(default)]
    pub anchor: MenuAnchor,

    #[serde(default = "default_menu_autoshow")]
    pub autoshow: bool,

    #[serde(default)]
    pub max_height: Option<NonZeroU32>,

    pub border: super::border::BorderSettings,
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            anchor: MenuAnchor::default(),
            autoshow: default_menu_autoshow(),
            max_height: Option::default(),
            border: border::BorderSettings {
                enable: false,
                style: border::BorderStyle::String(
                    border::BorderString::Single,
                ),
            },
        }
    }
}

#[derive(Debug, Deserialize)]
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
