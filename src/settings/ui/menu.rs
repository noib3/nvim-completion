use serde::Deserialize;
use std::num::NonZeroU32;

use super::border::{Border, BorderString, BorderStyle};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MenuSettings {
    #[serde(default)]
    pub anchor: MenuAnchor,

    #[serde(default = "default_menu_autoshow")]
    pub autoshow: bool,

    #[serde(default)]
    pub max_height: Option<NonZeroU32>,

    #[serde(default = "default_menu_border")]
    pub border: Border,
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

fn default_menu_border() -> Border {
    Border {
        enable: false,
        style: BorderStyle::String(BorderString::Single),
    }
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            anchor: MenuAnchor::default(),
            autoshow: default_menu_autoshow(),
            max_height: Option::default(),
            border: default_menu_border(),
        }
    }
}
