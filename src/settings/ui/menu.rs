use serde::{Deserialize, Deserializer};
use std::num::NonZeroU32;

use super::border::{Border, BorderString, BorderStyle, IncompleteBorder};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MenuSettings {
    #[serde(default)]
    pub anchor: MenuAnchor,

    #[serde(default = "default_autoshow")]
    pub autoshow: bool,

    #[serde(default)]
    pub max_height: Option<NonZeroU32>,

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

fn default_menu_border() -> Border {
    Border {
        enable: default_border_enable(),
        style: default_border_style(),
    }
}

fn deserialize_menu_border<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Border, D::Error> {
    Deserialize::deserialize(deserializer).map(
        |IncompleteBorder { enable, style }| {
            let enable = match (enable, &style) {
                (Some(b), _) => b,
                // If the `enable` field is missing but `style` is set the
                // border is enabled automatically.
                (None, Some(_)) => true,
                (None, None) => default_border_enable(),
            };

            Border {
                enable,
                style: style.unwrap_or(default_border_style()),
            }
        },
    )
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
