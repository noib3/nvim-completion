use compleet::completion::Completion;
use compleet::rpc::message::RpcNotification;
use mlua::{prelude::LuaResult, Lua};

use crate::bindings::nvim;
use crate::state::State;
use crate::ui;

pub fn handle_notify(
    lua: &Lua,
    state: &mut State,
    notification: RpcNotification,
) -> LuaResult<()> {
    nvim::print(
        lua,
        format!("Got a notification with method: {}", notification.method),
    )?;

    let cmp = Completion {
        details: None,
        format: notification.method,
        text: "Hi".into(),
        hl_ranges: Vec::new(),
        source: "lsp",
        matched_bytes: 1,
    };

    ui::update(lua, state, vec![cmp])
}
