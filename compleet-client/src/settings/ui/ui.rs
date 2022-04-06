use serde::Deserialize;

use super::{
    details::DetailsSettings,
    hint::HintSettings,
    menu::MenuSettings,
};

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiSettings {
    #[serde(default)]
    pub menu: MenuSettings,

    #[serde(default)]
    pub details: DetailsSettings,

    #[serde(default)]
    pub hint: HintSettings,
}
