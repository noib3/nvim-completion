use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bindings::opinionated::{
    lsp::protocol::{CompletionParams, CompletionResponse},
    Buffer,
    Neovim,
};
use mlua::prelude::{Lua, LuaResult};
use tree_sitter_highlight::Highlighter;

use super::{setup, LspConfig};
use crate::prelude::{
    CompletionItem,
    CompletionSource,
    Completions,
    Cursor,
    Result,
};
use crate::treesitter::{self, TSConfig};

#[derive(Debug, Default)]
pub struct Lsp {
    config: LspConfig,
    bufnr_to_ts_config: HashMap<u16, Arc<TSConfig>>,
    filetype_to_ts_config: HashMap<String, Arc<TSConfig>>,
}

impl From<LspConfig> for Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lsp {
    fn setup(&mut self, lua: &Lua) -> LuaResult<()> {
        setup::hlgroups(lua)
    }

    fn attach(&mut self, lua: &Lua, buffer: &Buffer) -> LuaResult<bool> {
        // TODO: check if buffer has any LSPs available.
        // vim.lsp.buf_is_attached

        if !self.config.highlight_completions {
            return Ok(true);
        }

        let filetype = &buffer.filetype;
        if treesitter::is_supported(filetype) {
            // Get the `TSConfig` for the given filetype if it already exists,
            // or create and cache a new one if it doesn't.
            let config = if let Some(config) =
                self.filetype_to_ts_config.get(filetype)
            {
                config.clone()
            } else {
                let config = Arc::new(TSConfig::new(lua, filetype)?);
                self.filetype_to_ts_config
                    .insert(filetype.clone(), config.clone());
                config
            };

            self.bufnr_to_ts_config.insert(buffer.bufnr, config);
        }

        Ok(true)
    }

    async fn complete(
        &self,
        nvim: &Neovim,
        cursor: &Cursor,
        buffer: &Buffer,
    ) -> Result<Completions> {
        let client = match nvim.lsp_buf_get_clients(0).await {
            v if v.is_empty() => return Ok(Vec::new()),
            v => v.into_iter().nth(0).unwrap(),
        };

        let params = CompletionParams::new(
            buffer.filepath.clone(),
            cursor.row as u32,
            cursor.bytes as u32,
        );

        let items = match client.request_completions(params, 0).await? {
            CompletionResponse::CompletionList(list) => list.items,
            CompletionResponse::CompletionItems(items) => items,
        };

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        let mut highlighter = Highlighter::new();

        let completions = items
            .into_iter()
            .filter(|lsp_item| {
                lsp_item.label.starts_with(word_pre)
                    && lsp_item.label != word_pre
            })
            .map(|lsp_item| {
                let mut comp =
                    CompletionItem::from_lsp(lsp_item, &buffer.filetype);

                // Highlight the text of the completion w/ treesitter if the
                // buffer has a `TSConfig` object for it.
                if let Some(c) = self.bufnr_to_ts_config.get(&buffer.bufnr) {
                    comp.ts_highlight_text(&mut highlighter, c);
                }

                comp
            })
            .collect::<Completions>();

        // ,

        Ok(completions)
    }
}
