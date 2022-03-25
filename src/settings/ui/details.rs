use serde::Deserialize;

use super::border::{Border, BorderItem, BorderStyle};
// use super::border::{self, Border, BorderStyle};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailsSettings {
    #[serde(default = "default_details_border")]
    pub border: Border,
}

fn default_details_border() -> Border {
    Border {
        enable: true,
        style: BorderStyle::Array4([
            BorderItem::Char("".into()),
            BorderItem::Char("".into()),
            BorderItem::Char("".into()),
            BorderItem::Tuple((" ".into(), Some("CompleetDetails".into()))),
        ]),
    }
}

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings {
            border: default_details_border(),
        }
    }
}
