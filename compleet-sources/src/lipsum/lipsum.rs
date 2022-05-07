use async_trait::async_trait;
use bindings::opinionated::{Buffer, Neovim};
use mlua::Lua;

use super::lorems::{LOREMS, LOREM_IPSUM};
use super::LipsumConfig;
use crate::completion_item::{CompletionItemBuilder, Completions};
use crate::completion_source::{CompletionSource, ShouldAttach};
use crate::cursor::Cursor;

#[derive(Debug, Default, Clone)]
pub struct Lipsum {
    pub _config: LipsumConfig,
}

#[async_trait]
impl CompletionSource for Lipsum {
    fn on_buf_enter(
        &mut self,
        _: &Lua,
        _: &Buffer,
    ) -> crate::Result<ShouldAttach> {
        Ok(true)
    }

    async fn complete(
        &mut self,
        _: &Neovim,
        cursor: &Cursor,
        _: &Buffer,
    ) -> crate::Result<Completions> {
        // // Simulate a slow source, this shouldn't block.
        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        Ok(LOREMS
            .iter()
            .filter(|&&word| word.starts_with(word_pre) && word != word_pre)
            .map(|&lorem| {
                CompletionItemBuilder::new(lorem)
                    .details_text(LOREM_IPSUM)
                    .build()
            })
            .collect())
    }
}
