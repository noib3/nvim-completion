use compleet::api::outgoing::Notification;
use mlua::{prelude::LuaResult, Lua};

use crate::state::State;
use crate::ui;

pub fn handle_notify(
    lua: &Lua,
    state: &mut State,
    ntf: Notification,
) -> LuaResult<()> {
    match ntf {
        Notification::ServeCompletions(completions) => {
            if completions.is_empty() {
                crate::bindings::nvim::print(lua, "Got empty completions!")?;
                return Ok(());
            }

            ui::update(lua, state, completions)
        },
        // _ => Ok(()),
    }
}
