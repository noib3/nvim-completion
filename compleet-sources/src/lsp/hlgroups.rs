use bindings::api;
use mlua::prelude::{Lua, LuaResult};

pub mod kind {
    pub const TEXT: &str = "CompleetLspKindText";
    pub const METHOD: &str = "CompleetLspKindMethod";
    pub const FUNCTION: &str = "CompleetLspKindFunction";
    pub const CONSTRUCTOR: &str = "CompleetLspKindConstructor";
    pub const FIELD: &str = "CompleetLspKindField";
    pub const VARIABLE: &str = "CompleetLspKindVariable";
    pub const CLASS: &str = "CompleetLspKindClass";
    pub const INTERFACE: &str = "CompleetLspKindInterface";
    pub const MODULE: &str = "CompleetLspKindModule";
    pub const PROPERTY: &str = "CompleetLspKindProperty";
    pub const UNIT: &str = "CompleetLspKindUnit";
    pub const VALUE: &str = "CompleetLspKindValue";
    pub const ENUM: &str = "CompleetLspKindEnum";
    pub const KEYWORD: &str = "CompleetLspKindKeyword";
    pub const SNIPPET: &str = "CompleetLspKindSnippet";
    pub const COLOR: &str = "CompleetLspKindColor";
    pub const FILE: &str = "CompleetLspKindFile";
    pub const REFERENCE: &str = "CompleetLspKindReference";
    pub const FOLDER: &str = "CompleetLspKindFolder";
    pub const ENUM_MEMBER: &str = "CompleetLspKindEnumMember";
    pub const CONSTANT: &str = "CompleetLspKindConstant";
    pub const STRUCT: &str = "CompleetLspKindStruct";
    pub const EVENT: &str = "CompleetLspKindEvent";
    pub const OPERATOR: &str = "CompleetLspKindOperator";
    pub const TYPE_PARAMETER: &str = "CompleetLspKindTypeParameter";
}

/// Sets up the highlight groups, linking them to pre-existing defaults. These
/// can be used by colorscheme plugin authors to style the UI.
pub fn setup(lua: &Lua) -> LuaResult<()> {
    let opts = lua.create_table_with_capacity(0, 2)?;

    // Don't override existing definitions.
    opts.set("default", true)?;

    opts.set("link", "TSText")?;
    api::set_hl(lua, 0, kind::TEXT, opts.clone())?;

    opts.set("link", "TSMethod")?;
    api::set_hl(lua, 0, kind::METHOD, opts.clone())?;

    opts.set("link", "TSFunction")?;
    api::set_hl(lua, 0, kind::FUNCTION, opts.clone())?;

    opts.set("link", "TSConstructor")?;
    api::set_hl(lua, 0, kind::CONSTRUCTOR, opts.clone())?;

    opts.set("link", "TSField")?;
    api::set_hl(lua, 0, kind::FIELD, opts.clone())?;

    opts.set("link", "TSVariable")?;
    api::set_hl(lua, 0, kind::VARIABLE, opts.clone())?;

    opts.set("link", "TSType")?;
    api::set_hl(lua, 0, kind::CLASS, opts.clone())?;

    opts.set("link", "TSType")?;
    api::set_hl(lua, 0, kind::INTERFACE, opts.clone())?;

    opts.set("link", "TSNamespace")?;
    api::set_hl(lua, 0, kind::MODULE, opts.clone())?;

    opts.set("link", "TSProperty")?;
    api::set_hl(lua, 0, kind::PROPERTY, opts.clone())?;

    opts.set("link", "TSNone")?;
    api::set_hl(lua, 0, kind::UNIT, opts.clone())?;

    opts.set("link", "TSVariable")?;
    api::set_hl(lua, 0, kind::VALUE, opts.clone())?;

    opts.set("link", "TSType")?;
    api::set_hl(lua, 0, kind::ENUM, opts.clone())?;

    opts.set("link", "TSKeyword")?;
    api::set_hl(lua, 0, kind::KEYWORD, opts.clone())?;

    opts.set("link", "TSStringSpecial")?;
    api::set_hl(lua, 0, kind::SNIPPET, opts.clone())?;

    opts.set("link", "TSConstant")?;
    api::set_hl(lua, 0, kind::COLOR, opts.clone())?;

    opts.set("link", "TSInclude")?;
    api::set_hl(lua, 0, kind::FILE, opts.clone())?;

    opts.set("link", "TSCharacter")?;
    api::set_hl(lua, 0, kind::REFERENCE, opts.clone())?;

    opts.set("link", "TSInclude")?;
    api::set_hl(lua, 0, kind::FOLDER, opts.clone())?;

    opts.set("link", "TSField")?;
    api::set_hl(lua, 0, kind::ENUM_MEMBER, opts.clone())?;

    opts.set("link", "TSContant")?;
    api::set_hl(lua, 0, kind::CONSTANT, opts.clone())?;

    opts.set("link", "TSType")?;
    api::set_hl(lua, 0, kind::STRUCT, opts.clone())?;

    opts.set("link", "TSProperty")?;
    api::set_hl(lua, 0, kind::EVENT, opts.clone())?;

    opts.set("link", "TSOperator")?;
    api::set_hl(lua, 0, kind::OPERATOR, opts.clone())?;

    opts.set("link", "TSType")?;
    api::set_hl(lua, 0, kind::TYPE_PARAMETER, opts.clone())?;

    Ok(())
}
