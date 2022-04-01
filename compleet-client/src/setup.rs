use std::sync::Arc;

use mlua::prelude::{Lua, LuaResult, LuaValue};
use parking_lot::Mutex;

use crate::channel::Channel;
use crate::state::State;

/// Executed by the `require("compleet").setup` Lua function.
pub fn setup(
    lua: &Lua,
    state: &Arc<Mutex<State>>,
    _preferences: LuaValue,
) -> LuaResult<()> {
    {
        let state = &mut state.lock();
        state.channel = Some(Channel::new(lua)?);
    }

    let print = lua.globals().get::<_, mlua::Function>("print")?;
    print.call::<_, ()>("Setup complete!")
}
