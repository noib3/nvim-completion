use mlua::Lua;
use neovim::Api;

/// TODO: docs
#[derive(Debug)]
pub enum DetailsPosition {
    /// TODO: docs
    After { width: usize },

    /// TODO: docs
    Before { width: usize, height: usize },
}

/// TODO: docs
pub fn create_window(
    lua: &Lua,
    api: &Api,
    bufnr: usize,
    width: usize,
    height: usize,
) -> super::Result<(usize, DetailsPosition)> {
    todo!()
}
