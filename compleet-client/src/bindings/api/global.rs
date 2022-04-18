use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    FromLua,
    Table,
};

// TODO: make `command` accept strings.
/// Binding to `vim.api.nvim_create_user_command`.
pub fn create_user_command(
    lua: &Lua,
    name: &str,
    command: LuaFunction,
    opts: Table,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_create_user_command")?
        .call((name, command, opts))
}

/// Binding to `vim.api.nvim_create_buf`.
pub fn create_buf(lua: &Lua, listed: bool, scratch: bool) -> LuaResult<u16> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_create_buf")?
        .call((listed, scratch))
}

/// Binding to `vim.api.nvim_echo`.
pub fn echo(
    lua: &Lua,
    chunks: Vec<(String, Option<&str>)>,
    history: bool,
) -> LuaResult<()> {
    let chunks = chunks
        .into_iter()
        .map(|(text, hlgroup)| match hlgroup {
            Some(group) => vec![text, group.to_string()],
            None => vec![text],
        })
        .collect::<Vec<Vec<String>>>();

    super::api(lua)?.get::<&str, LuaFunction>("nvim_echo")?.call((
        chunks,
        history,
        Vec::<u8>::new(),
    ))
}

/// Binding to `vim.api.nvim_get_current_buf`
pub fn get_current_buf(lua: &Lua) -> LuaResult<u16> {
    super::api(lua)?.get::<&str, LuaFunction>("nvim_get_current_buf")?.call(())
}

/// Binding to `vim.api.nvim_get_current_line`
pub fn get_current_line(lua: &Lua) -> LuaResult<String> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_get_current_line")?
        .call(())
}

/// Binding to `vim.api.nvim_get_mode`
pub fn get_mode(lua: &Lua) -> LuaResult<(String, bool)> {
    let t = super::api(lua)?
        .get::<&str, LuaFunction>("nvim_get_mode")?
        .call::<_, Table>(())?;

    Ok((t.get("mode")?, t.get("blocking")?))
}

/// Binding to `vim.api.nvim_get_option`
pub fn get_option<'lua, V: FromLua<'lua>>(
    lua: &'lua Lua,
    name: &str,
) -> LuaResult<V> {
    super::api(lua)?.get::<&str, LuaFunction>("nvim_get_option")?.call(name)
}

/// Binding to `vim.api.nvim_get_runtime_file`.
pub fn get_runtime_file(
    lua: &Lua,
    name: &str,
    all: bool,
) -> LuaResult<Vec<String>> {
    super::api(lua)?
        .get::<_, LuaFunction>("nvim_get_runtime_file")?
        .call((name, all))
}

/// Binding to `vim.api.nvim_notify`.
pub fn notify<S: AsRef<str>>(
    lua: &Lua,
    msg: S,
    level: super::LogLevel,
) -> LuaResult<()> {
    super::api(lua)?.get::<&str, LuaFunction>("nvim_notify")?.call((
        msg.as_ref(),
        level as u8,
        Vec::<u8>::new(),
    ))
}

#[allow(dead_code)]
/// Binding to `vim.api.nvim_replace_termcodes`
pub fn replace_termcodes(
    lua: &Lua,
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> LuaResult<std::ffi::CString> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_replace_termcodes")?
        .call((str, from_part, do_lt, special))
}

/// Binding to `vim.api.nvim_set_hl`.
pub fn set_hl(
    lua: &Lua,
    ns_id: u16,
    name: &str,
    opts: Table,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_set_hl")?
        .call((ns_id, name, opts))
}

/// Binding to `vim.api.nvim_set_keymap`
pub fn set_keymap(
    lua: &Lua,
    mode: &str,
    lhs: &str,
    rhs: &str,
    opts: Table,
) -> LuaResult<()> {
    super::api(lua)?
        .get::<&str, LuaFunction>("nvim_set_keymap")?
        .call((mode, lhs, rhs, opts))
}
