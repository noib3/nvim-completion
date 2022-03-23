use serde::{Deserialize, Serialize};

use super::border;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DetailsSettings {
    // #[serde(default)]
    pub border: border::BorderSettings,
}

impl Default for DetailsSettings {
    fn default() -> Self {
        DetailsSettings {
            border: border::BorderSettings {
                enable: true,
                style: border::BorderStyle::ArrayWithHlgroup8([
                    ["".to_string(), "CompleetDetails".to_string()],
                    ["".to_string(), "CompleetDetails".to_string()],
                    ["".to_string(), "CompleetDetails".to_string()],
                    [" ".to_string(), "CompleetDetails".to_string()],
                    ["".to_string(), "CompleetDetails".to_string()],
                    ["".to_string(), "CompleetDetails".to_string()],
                    ["".to_string(), "CompleetDetails".to_string()],
                    [" ".to_string(), "CompleetDetails".to_string()],
                ]),
            },
        }
    }
}
