use serde::Deserialize;

use super::{details, hint, menu};

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiSettings {
    #[serde(default)]
    pub menu: menu::MenuSettings,

    #[serde(default)]
    pub details: details::DetailsSettings,

    #[serde(default)]
    pub hint: hint::HintSettings,
}
