use async_trait::async_trait;
use mlua::prelude::{Lua, LuaResult};

use super::{
    lorems::{LOREMS, LOREM_IPSUM},
    LipsumConfig,
};
use crate::prelude::{CompletionItem, CompletionSource, Completions, Cursor};

#[derive(Debug, Default)]
pub struct Lipsum {
    _config: LipsumConfig,
}

impl From<LipsumConfig> for Lipsum {
    fn from(_config: LipsumConfig) -> Self {
        Self { _config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lipsum {
    fn attach(&mut self, _lua: &Lua, _bufnr: u16) -> LuaResult<bool> {
        Ok(true)
    }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Vec::new();
        }

        LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|word| CompletionItem {
                details: Some(
                    LOREM_IPSUM.iter().map(|word| word.to_string()).collect(),
                ),
                format: format!(" {word} "),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u32,
                source: "Lipsum",
                text: word.to_string(),
            })
            .collect()
    }
}
