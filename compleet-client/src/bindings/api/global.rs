use mlua::prelude::{Lua, LuaFunction, LuaResult};

pub fn echo(
    lua: &Lua,
    chunks: Vec<(&str, Option<&str>)>,
    history: bool,
) -> LuaResult<()> {
    let chunks = chunks
        .iter()
        .map(|(text, hlgroup)| match hlgroup {
            Some(group) => vec![*text, *group],
            None => vec![*text],
        })
        .collect::<Vec<Vec<&str>>>();

    super::api(&lua)?
        .get::<&str, LuaFunction>("nvim_echo")?
        .call((chunks, history, Vec::<u8>::new()))
}

pub fn get_runtime_file(
    lua: &Lua,
    name: &str,
    all: bool,
) -> LuaResult<Vec<String>> {
    super::api(&lua)?
        .get::<_, LuaFunction>("nvim_get_runtime_file")?
        .call((name, all))
}
