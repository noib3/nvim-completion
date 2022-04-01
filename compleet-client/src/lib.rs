use std::sync::Arc;

use mlua::{prelude::LuaResult, Lua, Table};
use parking_lot::Mutex;

mod channel;
mod constants;
mod setup;
mod state;
mod ui;

use state::State;

#[mlua::lua_module]
fn compleet(lua: &Lua) -> LuaResult<Table> {
    let on_exit = lua.create_function(
        |lua, (channel_id, exit_code, _exit): (u32, u32, String)| {
            let print = lua.globals().get::<_, mlua::Function>("print")?;

            print.call::<_, ()>(format!(
                "process on channel {channel_id} exited with code {exit_code}"
            ))
        },
    )?;

    let on_stderr = lua.create_function(|_lua, ()| Ok(()))?;

    let _ = Arc::new(Mutex::new(State::new(lua, on_exit, on_stderr)?));

    // let s = state.clone();
    // let has_completions = lua.create_function(move |_lua, ()| {
    //     let _ = &mut s.lock();
    //     Ok(())
    // })?;

    // let s = state.clone();
    // let is_completion_selected = lua.create_function(move |_lua, ()| {
    //     let _ = &mut s.lock();
    //     Ok(())
    // })?;

    // let s = state.clone();
    // let is_hint_visible = lua.create_function(move |_lua, ()| {
    //     let _ = &mut s.lock();
    //     Ok(())
    // })?;

    // let s = state.clone();
    // let is_menu_visible = lua.create_function(move |_lua, ()| {
    //     let _ = &mut s.lock();
    //     Ok(())
    // })?;

    let setup = lua.create_function(move |lua, preferences| {
        setup::setup(lua, preferences)
    })?;

    // let _callback = lua.create_function(move |lua, message: String| {
    //     let print = lua
    //         .globals()
    //         .get::<_, Table>("vim")?
    //         .get::<_, mlua::Function>("print")?;

    //     print.call::<_, ()>(message)
    // })?;

    // let _update_ui = lua.create_function(move |lua, ()| {
    //     ui::update(lua, &mut state.lock())?;
    //     Ok(())
    // })?;

    // let _ = std::thread::spawn(move || {
    //     update_ui.call::<_, ()>(()).unwrap();
    // });

    lua.create_table_from([
        // ("has_completions", has_completions),
        // ("is_completion_selected", is_completion_selected),
        // ("is_hint_visible", is_hint_visible),
        // ("is_menu_visible", is_menu_visible),
        ("setup", setup),
    ])
}
