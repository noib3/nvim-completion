use nvim::api::{self, Buffer, Window};
use nvim_oxi as nvim;

use super::ui_config::MenuConfig;

const MENU_NAMESPACE: &str = "completion_menu";

#[derive(Debug)]
pub(crate) struct CompletionMenu {
    config: MenuConfig,

    /// TODO: docs
    buf: Buffer,

    /// TODO: docs
    win: Option<Window>,

    /// TODO: docs
    height: u16,

    /// TODO: docs
    width: u16,

    /// TODO: docs
    namespace_id: u32,
}

impl Default for CompletionMenu {
    #[inline]
    fn default() -> Self {
        Self {
            config: MenuConfig::default(),
            buf: 0.into(),
            win: None,
            height: 0,
            width: 0,
            namespace_id: api::create_namespace(MENU_NAMESPACE),
        }
    }
}

impl CompletionMenu {
    #[inline]
    pub(super) fn init(&mut self, config: MenuConfig) -> nvim::Result<()> {
        self.config = config;
        self.buf = api::create_buf(false, true)?;

        Ok(())
    }
}
