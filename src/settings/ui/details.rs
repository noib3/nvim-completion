use serde::Deserialize;

// use super::border::{BorderItem, BorderSettings, BorderStyle};
use super::border::{self, BorderSettings, BorderStyle};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailsSettings {
    pub border: BorderSettings,
}

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings {
            border: BorderSettings {
                enable: true,
                // style: BorderStyle::Array4([
                //     BorderItem::String("".into()),
                //     BorderItem::String("".into()),
                //     BorderItem::String("".into()),
                //     BorderItem::Tuple((
                //         " ".into(),
                //         Some("CompleetDetails".into()),
                //     )),
                // ]),
                style: BorderStyle::Array4WithHlgroup([
                    (
                        border::OnecharOrEmpty("".to_string()),
                        "CompleetDetails".to_string(),
                    ),
                    (
                        border::OnecharOrEmpty("".to_string()),
                        "CompleetDetails".to_string(),
                    ),
                    (
                        border::OnecharOrEmpty("".to_string()),
                        "CompleetDetails".to_string(),
                    ),
                    (
                        border::OnecharOrEmpty(" ".to_string()),
                        "CompleetDetails".to_string(),
                    ),
                ]),
            },
        }
    }
}
