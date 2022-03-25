use serde::{Deserialize, Deserializer};

use super::border::{Border, BorderItem, BorderStyle, IncompleteBorder};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailsSettings {
    #[serde(deserialize_with = "deserialize_details_border")]
    #[serde(default = "default_details_border")]
    pub border: Border,
}

fn default_border_enable() -> bool {
    true
}

fn default_border_style() -> BorderStyle {
    BorderStyle::Array4([
        BorderItem::Char("".into()),
        BorderItem::Char("".into()),
        BorderItem::Char("".into()),
        BorderItem::Tuple((" ".into(), Some("CompleetDetails".into()))),
    ])
}

fn default_details_border() -> Border {
    Border {
        enable: default_border_enable(),
        style: default_border_style(),
    }
}

fn deserialize_details_border<'de, D: Deserializer<'de>>(
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

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings {
            border: default_details_border(),
        }
    }
}
