use serde::{Deserialize, Deserializer};

use super::border::{self, Border, BorderItem, BorderStyle};

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
    use BorderItem::{Char, Tuple};
    BorderStyle::Array4([
        Char("".into()),
        Char("".into()),
        Char("".into()),
        // TODO: use constant
        Tuple((" ".into(), Some("CompleetDetails".into()))),
    ])
}

fn deserialize_details_border<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Border, D::Error> {
    border::deserialize(
        deserializer,
        default_border_enable,
        default_border_style,
    )
}

fn default_details_border() -> Border {
    Border { enable: default_border_enable(), style: default_border_style() }
}

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings { border: default_details_border() }
    }
}
