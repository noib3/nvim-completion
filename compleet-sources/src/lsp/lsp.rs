use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bindings::opinionated::{
    lsp::protocol::CompletionResponse,
    Buffer,
    Neovim,
};
use mlua::prelude::{Lua, LuaResult};
use tree_sitter_highlight::Highlighter as OGHighlighter;
use treesitter_highlighter::Highlighter;

use super::{setup, LspConfig};
use crate::prelude::{
    CompletionItem,
    CompletionSource,
    Completions,
    Cursor,
    Result,
};
// use crate::treesitter::{self, TSConfig};

#[derive(Debug, Default)]
pub struct Lsp {
    config: LspConfig,
    buf_to_highlighter: HashMap<u16, Arc<Highlighter>>,
    ft_to_highlighter: HashMap<String, Arc<Highlighter>>,
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

    fn attach(&mut self, _lua: &Lua, buffer: &Buffer) -> LuaResult<bool> {
        // TODO: check if buffer has any LSPs available.
        // vim.lsp.buf_is_attached

        if !self.config.highlight_completions {
            return Ok(true);
        }

        let ft = &buffer.filetype;
        if let Some(highlighter) = self.ft_to_highlighter.get(ft) {
            self.buf_to_highlighter.insert(buffer.bufnr, highlighter.clone());
        } else if let Some(hl) = Highlighter::from_filetype(ft) {
            let highlighter = Arc::new(hl);
            self.buf_to_highlighter.insert(buffer.bufnr, highlighter.clone());
            self.ft_to_highlighter.insert(ft.to_owned(), highlighter);
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

        let params = super::make_completion_params(
            buffer,
            cursor,
            client.offset_encoding,
        );

        let items = match client.request_completions(params, 0).await? {
            CompletionResponse::CompletionList(list) => list.items,
            CompletionResponse::CompletionItems(items) => items,
        };

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        let mut hl = OGHighlighter::new();

        let completions = items
            .into_iter()
            .filter(|lsp_item| {
                lsp_item.label.starts_with(word_pre)
                    && lsp_item.label != word_pre
            })
            .map(|lsp_item| {
                let mut comp =
                    CompletionItem::from_lsp(lsp_item, &buffer.filetype);

                if let Some(h) = self.buf_to_highlighter.get(&buffer.bufnr) {
                    comp.highlight_text(h.highlight(&mut hl, &comp.text));
                }

                comp
            })
            .collect::<Completions>();

        Ok(completions)
    }
}
