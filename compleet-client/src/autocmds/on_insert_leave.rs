use mlua::Lua;

use crate::client::Client;

/// Called every time the user exits insert mode (only in attached buffers).
pub fn on_insert_leave(lua: &Lua, client: &mut Client) -> mlua::Result<()> {
    client.stop_fetching_completions();
    client.clear_completions();
    client.cleanup_ui(lua)
}
