use nvim_oxi::api::types::WindowBorder;
use serde::Deserialize;

use super::{DetailsConfig, HintConfig, MenuConfig};

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UiConfig {
    #[serde(default)]
    pub(super) details: DetailsConfig,

    #[serde(default)]
    pub(super) hint: HintConfig,

    #[serde(default)]
    pub(super) menu: MenuConfig,
}

#[derive(Debug, Deserialize)]
pub(super) struct Border {
    pub(super) enable: bool,
    pub(super) style: WindowBorder,
}
