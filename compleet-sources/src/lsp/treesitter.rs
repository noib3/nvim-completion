use std::{fmt, fs, ops::Range};

use mlua::prelude::{Lua, LuaFunction, LuaResult, LuaTable};
use tree_sitter_highlight::{
    Highlight,
    HighlightConfiguration,
    HighlightEvent,
    Highlighter,
};

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

pub struct Thingy {
    // highlighter: Highlighter,
    config: HighlightConfiguration,
}

impl fmt::Debug for Thingy {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Thingy")
    }
}

impl Thingy {
    pub fn new(lua: &Lua, language: &str) -> LuaResult<Self> {
        let highlight_query = self::get_highlight_query(lua, language)?;

        if highlight_query.is_empty() {
            todo!()
        }

        // let highlighter = Highlighter::new();

        let mut config = HighlightConfiguration::new(
            // TODO: get language from `rtp/parser`.
            tree_sitter_rust::language(),
            &highlight_query,
            "",
            "",
        )
        .unwrap();

        config.configure(self::HIGHLIGHT_NAMES);

        Ok(Self { config })

        // Ok(Self { highlighter, config })
    }

    pub fn highlight(
        &self,
        string: &str,
    ) -> Vec<(&'static str, Range<usize>)> {
        let mut highlighter = Highlighter::new();
        let iter = highlighter
            .highlight(&self.config, string.as_bytes(), None, |_| None)
            .unwrap();

        let hint = iter.size_hint();
        let mut ranges = Vec::with_capacity(hint.1.unwrap_or(hint.0));

        let mut group = "";
        let mut sstart = 0;
        let mut send = 0;

        use HighlightEvent::*;

        for event in iter {
            match event.unwrap() {
                HighlightStart(Highlight(i)) => {
                    group = hlname_to_hlgroup(HIGHLIGHT_NAMES[i]);
                },

                Source { start, end } => {
                    sstart = start;
                    send = end;
                },

                HighlightEnd => {
                    ranges.push((group, sstart..send));
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

/// Maps treesitter highlight names to Neovim's highlight groups created by the
/// `nvim-treesitter/nvim-treesitter` plugin.
fn hlname_to_hlgroup(name: &str) -> &'static str {
    // Taken from `/nvim-treesitter/lua/nvim-treesitter/highlight.lua#14`
    match name {
        "annotation" => "TSAnnotation",
        "attribute" => "TSAttribute",
        "boolean" => "TSBoolean",
        "character" => "TSCharacter",
        "character.special" => "TSCharacterSpecial",
        "comment" => "TSComment",
        "conditional" => "TSConditional",
        "constant" => "TSConstant",
        "constant.builtin" => "TSConstBuiltin",
        "constant.macro" => "TSConstMacro",
        "constructor" => "TSConstructor",
        "debug" => "TSDebug",
        "define" => "TSDefine",
        "error" => "TSError",
        "exception" => "TSException",
        "field" => "TSField",
        "float" => "TSFloat",
        "function" => "TSFunction",
        "function.builtin" => "TSFuncBuiltin",
        "function.macro" => "TSFuncMacro",
        "include" => "TSInclude",
        "keyword" => "TSKeyword",
        "keyword.function" => "TSKeywordFunction",
        "keyword.operator" => "TSKeywordOperator",
        "keyword.return" => "TSKeywordReturn",
        "label" => "TSLabel",
        "method" => "TSMethod",
        "namespace" => "TSNamespace",
        "none" => "TSNone",
        "number" => "TSNumber",
        "operator" => "TSOperator",
        "parameter" => "TSParameter",
        "parameter.reference" => "TSParameterReference",
        "preproc" => "TSPreProc",
        "property" => "TSProperty",
        "punctuation.delimiter" => "TSPunctDelimiter",
        "punctuation.bracket" => "TSPunctBracket",
        "punctuation.special" => "TSPunctSpecial",
        "repeat" => "TSRepeat",
        "storageclass" => "TSStorageClass",
        "string" => "TSString",
        "string.regex" => "TSStringRegex",
        "string.escape" => "TSStringEscape",
        "string.special" => "TSStringSpecial",
        "symbol" => "TSSymbol",
        "tag" => "TSTag",
        "tag.attribute" => "TSTagAttribute",
        "tag.delimiter" => "TSTagDelimiter",
        "text" => "TSText",
        "text.strong" => "TSStrong",
        "text.emphasis" => "TSEmphasis",
        "text.underline" => "TSUnderline",
        "text.strike" => "TSStrike",
        "text.title" => "TSTitle",
        "text.literal" => "TSLiteral",
        "text.uri" => "TSURI",
        "text.math" => "TSMath",
        "text.reference" => "TSTextReference",
        "text.environment" => "TSEnvironment",
        "text.environment.name" => "TSEnvironmentName",
        "text.note" => "TSNote",
        "text.warning" => "TSWarning",
        "text.danger" => "TSDanger",
        "todo" => "TSTodo",
        "type" => "TSType",
        "type.builtin" => "TSTypeBuiltin",
        "type.qualifier" => "TSTypeQualifier",
        "type.definition" => "TSTypeDefinition",
        "variable" => "TSVariable",
        "variable.builtin" => "TSVariableBuiltin",
        _ => "TSNone",
    }
}
