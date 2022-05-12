use mlua::Lua;

use crate::client::Client;

/// Called every time the cursor is moved while in insert mode (only in
/// attached buffers).
pub fn on_cursor_moved_i(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    if client.should_skip_next_cursor_moved_i() {
        return Ok(());
    }

    client.stop_fetching_completions();
    client.clear_completions();
    client.cleanup_ui(lua)
}
