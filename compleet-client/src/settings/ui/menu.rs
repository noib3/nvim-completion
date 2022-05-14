use std::num::NonZeroUsize;

use serde::{Deserialize, Deserializer};

use super::border::{self, Border, BorderString, BorderStyle};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MenuSettings {
    #[serde(default)]
    pub anchor: MenuAnchor,

    #[serde(default = "default_autoshow")]
    pub autoshow: bool,

    #[serde(default)]
    pub max_height: Option<NonZeroUsize>,

    #[serde(deserialize_with = "deserialize_menu_border")]
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

fn default_autoshow() -> bool {
    true
}

fn default_border_enable() -> bool {
    false
}

fn default_border_style() -> BorderStyle {
    BorderStyle::String(BorderString::Single)
}

fn deserialize_menu_border<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Border, D::Error> {
    border::deserialize(
        deserializer,
        default_border_enable,
        default_border_style,
    )
}

fn default_menu_border() -> Border {
    Border { enable: default_border_enable(), style: default_border_style() }
}

impl Default for MenuSettings {
    fn default() -> Self {
        MenuSettings {
            anchor: MenuAnchor::default(),
            autoshow: default_autoshow(),
            max_height: Option::default(),
            border: default_menu_border(),
        }
    }
}
