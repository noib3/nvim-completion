use std::collections::HashMap;

use nvim_oxi::{api::Buffer as NvimBuffer, Function, Object};
use serde::Deserialize;

pub(crate) type SourceConfigs = HashMap<String, SourceConfig>;

#[derive(Deserialize)]
pub(crate) struct SourceConfig {
    pub(crate) enable: SourceEnable,

    #[serde(flatten)]
    pub(crate) rest: Object,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum SourceEnable {
    Final(bool),
    Depends(Function<NvimBuffer, bool>),
}
