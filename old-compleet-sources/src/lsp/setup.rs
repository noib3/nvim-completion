use bindings::api;
use mlua::prelude::{Lua, LuaResult};

use super::constants::hlgroup;

/// Sets up the highlight groups for the Lsp icons, linking them to
/// pre-existing defaults.
pub fn hlgroups(lua: &Lua) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::TEXT, opts.clone())?;

    opts.set("link", "TSFunction")?;
    api::set_hl(lua, 0, hlgroup::METHOD, opts.clone())?;

    opts.set("link", "TSFunction")?;
    api::set_hl(lua, 0, hlgroup::FUNCTION, opts.clone())?;

    opts.set("link", "TSFunction")?;
    api::set_hl(lua, 0, hlgroup::CONSTRUCTOR, opts.clone())?;

    opts.set("link", "TSConstructor")?;
    api::set_hl(lua, 0, hlgroup::FIELD, opts.clone())?;

    opts.set("link", "TSConstructor")?;
    api::set_hl(lua, 0, hlgroup::VARIABLE, opts.clone())?;

    opts.set("link", "TSParameter")?;
    api::set_hl(lua, 0, hlgroup::CLASS, opts.clone())?;

    opts.set("link", "TSConstructor")?;
    api::set_hl(lua, 0, hlgroup::INTERFACE, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::MODULE, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::PROPERTY, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::UNIT, opts.clone())?;

    opts.set("link", "TSParameter")?;
    api::set_hl(lua, 0, hlgroup::VALUE, opts.clone())?;

    opts.set("link", "TSParameter")?;
    api::set_hl(lua, 0, hlgroup::ENUM, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::KEYWORD, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::SNIPPET, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::COLOR, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::FILE, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::REFERENCE, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::FOLDER, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::ENUM_MEMBER, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::CONSTANT, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::STRUCT, opts.clone())?;

    opts.set("link", "TSParameter")?;
    api::set_hl(lua, 0, hlgroup::EVENT, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::OPERATOR, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, hlgroup::TYPE_PARAMETER, opts.clone())?;

    Ok(())
}
