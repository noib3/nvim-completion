use nvim_oxi::{api::Buffer, Function};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum SourceEnable {
    Bool(bool),
    Function(Function<Buffer, bool>),
}
