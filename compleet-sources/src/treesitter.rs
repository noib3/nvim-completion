use std::{fmt, fs};

use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};
use tree_sitter_highlight::{
    Highlight,
    HighlightConfiguration,
    HighlightEvent,
    Highlighter,
};

use crate::completion_item::HlRange;

// Taken from `/nvim-treesitter/lua/nvim-treesitter/highlight.lua#14`
const HIGHLIGHT_NAMES: &[&str] = &[
    "annotation",
    "attribute",
    "boolean",
    "character",
    "character.special",
    "comment",
    "conditional",
    "constant",
    "constant.builtin",
    "constant.macro",
    "constructor",
    "debug",
    "define",
    "error",
    "exception",
    "field",
    "float",
    "function",
    "function.builtin",
    "function.macro",
    "include",
    "keyword",
    "keyword.function",
    "keyword.operator",
    "keyword.return",
    "label",
    "method",
    "namespace",
    "none",
    "number",
    "operator",
    "parameter",
    "parameter.reference",
    "preproc",
    "property",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "repeat",
    "storageclass",
    "string",
    "string.regex",
    "string.escape",
    "string.special",
    "symbol",
    "tag",
    "tag.attribute",
    "tag.delimiter",
    "text",
    "text.strong",
    "text.emphasis",
    "text.underline",
    "text.strike",
    "text.title",
    "text.literal",
    "text.uri",
    "text.math",
    "text.reference",
    "text.environment",
    "text.environment.name",
    "text.note",
    "text.warning",
    "text.danger",
    "todo",
    "type",
    "type.builtin",
    "type.qualifier",
    "type.definition",
    "variable",
    "variable.builtin",
];

// Taken from `/nvim-treesitter/lua/nvim-treesitter/highlight.lua#14`
const HLGROUP_NAMES: &[&str] = &[
    "TSAnnotation",
    "TSAttribute",
    "TSBoolean",
    "TSCharacter",
    "TSCharacterSpecial",
    "TSComment",
    "TSConditional",
    "TSConstant",
    "TSConstBuiltin",
    "TSConstMacro",
    "TSConstructor",
    "TSDebug",
    "TSDefine",
    "TSError",
    "TSException",
    "TSField",
    "TSFloat",
    "TSFunction",
    "TSFuncBuiltin",
    "TSFuncMacro",
    "TSInclude",
    "TSKeyword",
    "TSKeywordFunction",
    "TSKeywordOperator",
    "TSKeywordReturn",
    "TSLabel",
    "TSMethod",
    "TSNamespace",
    "TSNone",
    "TSNumber",
    "TSOperator",
    "TSParameter",
    "TSParameterReference",
    "TSPreProc",
    "TSProperty",
    "TSPunctDelimiter",
    "TSPunctBracket",
    "TSPunctSpecial",
    "TSRepeat",
    "TSStorageClass",
    "TSString",
    "TSStringRegex",
    "TSStringEscape",
    "TSStringSpecial",
    "TSSymbol",
    "TSTag",
    "TSTagAttribute",
    "TSTagDelimiter",
    "TSText",
    "TSStrong",
    "TSEmphasis",
    "TSUnderline",
    "TSStrike",
    "TSTitle",
    "TSLiteral",
    "TSURI",
    "TSMath",
    "TSTextReference",
    "TSEnvironment",
    "TSEnvironmentName",
    "TSNote",
    "TSWarning",
    "TSDanger",
    "TSTodo",
    "TSType",
    "TSTypeBuiltin",
    "TSTypeQualifier",
    "TSTypeDefinition",
    "TSVariable",
    "TSVariableBuiltin",
    "TSNone",
];

const SUPPORTED_LANGUAGES: &[&str] = &["rust"];

pub fn is_supported(language: &str) -> bool {
    SUPPORTED_LANGUAGES.contains(&language)
}

pub struct TSConfig(HighlightConfiguration);

impl fmt::Debug for TSConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("TSConfig(??)")
    }
}

impl TSConfig {
    pub fn new(lua: &Lua, language: &str) -> LuaResult<Self> {
        let highlight_query = self::get_highlight_query(lua, language)?;

        if highlight_query.is_empty() {
            todo!()
        }

        let mut config = HighlightConfiguration::new(
            // TODO: get language from `rtp/parser`.
            tree_sitter_rust::language(),
            &highlight_query,
            "",
            "",
        )
        .unwrap();

        config.configure(self::HIGHLIGHT_NAMES);

        Ok(Self(config))
    }

    /// TODO: docs
    pub fn highlight(
        &self,
        highlighter: &mut Highlighter,
        string: &str,
    ) -> Vec<HlRange> {
        let events = highlighter
            .highlight(&self.0, string.as_bytes(), None, |_| None)
            .unwrap();

        let size = events.size_hint();
        let mut ranges = Vec::with_capacity(size.1.unwrap_or(size.0));

        let mut group = "";
        let mut start = 0;
        let mut end = 0;

        for event in events {
            match event.unwrap() {
                HighlightEvent::HighlightStart(Highlight(i)) => {
                    group = HLGROUP_NAMES[i];
                },

                HighlightEvent::Source { start: s, end: e } => {
                    start = s;
                    end = e;
                },

                HighlightEvent::HighlightEnd => {
                    ranges.push((start..end, group));
                },
            }
        }

        ranges
    }
}

/// Retrieves the highlight query string for a given language from the Neovim
/// runtime path.
fn get_highlight_query(lua: &Lua, language: &str) -> LuaResult<String> {
    let hl_query_files = lua
        .globals()
        .get::<_, LuaFunction>("require")?
        .call::<_, LuaTable>("vim.treesitter.query")?
        .get::<_, LuaFunction>("get_query_files")?
        .call::<_, Vec<String>>((language, "highlights"))?;

    let queries = hl_query_files
        .into_iter()
        .map(|filepath| {
            fs::read_to_string(filepath).expect("neovim said it exists")
        })
        .collect::<Vec<String>>();

    Ok(queries.join("\n"))
}
