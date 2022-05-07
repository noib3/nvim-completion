use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bindings::opinionated::lsp::protocol::{
    CompletionItem as LspCompletionItem,
    CompletionResponse,
};
use bindings::opinionated::{lsp::LspClient, Buffer, Neovim};
use futures::future;
use mlua::Lua;
use treesitter_highlighter::Highlighter;

use super::{setup, LspConfig};
use crate::completion_source::{CompletionSource, ShouldAttach};
use crate::prelude::{CompletionItem, Completions, Cursor};

#[derive(Debug, Default, Clone)]
pub struct Lsp {
    pub config: LspConfig,

    /// The Lsp clients attached to a buffer.
    pub _clients: HashMap<u16, Vec<LspClient>>,

    /// Maps buffer numbers to tree-sitter highlighters.
    pub buf_to_highlighter: HashMap<u16, Highlighter>,
    // /// Maps Neovim filetypes to tree-sitter highlighters.
    // pub ft_to_highlighter: HashMap<String, Highlighter>,
}

#[async_trait]
impl CompletionSource for Lsp {
    fn setup(&mut self, lua: &Lua) -> crate::Result<()> {
        Ok(setup::hlgroups(lua)?)
    }

    fn on_buf_enter(
        &mut self,
        _lua: &Lua,
        buffer: &Buffer,
    ) -> crate::Result<ShouldAttach> {
        // TODO: check if buffer has any LSPs available.

        if !self.config.highlight_completions {
            return Ok(true);
        }

        let ft = &buffer.filetype;

        if let Some(hl) = Highlighter::from_filetype(ft) {
            self.buf_to_highlighter.insert(buffer.bufnr, hl);
        }

        // // If we already have a highlighter cached for this filetype we can
        // // just clone it.
        // if let Some(highlighter) = self.ft_to_highlighter.get(ft) {
        //     self.buf_to_highlighter.insert(buffer.bufnr, highlighter.clone());
        // }
        // // If we don't we check if this filetype can be highlighted. If it can
        // // we also cache the returned highlighter for this filetype.
        // else if let Some(hl) = Highlighter::from_filetype(ft) {
        //     let highlighter = Arc::new(hl);

        //     self.buf_to_highlighter.insert(buffer.bufnr, highlighter.clone());
        //     self.ft_to_highlighter.insert(ft.to_owned(), highlighter);
        // }

        Ok(true)
    }

    async fn complete(
        &mut self,
        nvim: &Neovim,
        cursor: &Cursor,
        buffer: &Buffer,
    ) -> crate::Result<Completions> {
        // let clients = self.clients.get(&buffer.bufnr).unwrap();
        let clients = nvim.lsp_buf_get_clients(buffer.bufnr).await;

        if clients.is_empty() {
            return Ok(Vec::new());
        }

        let requests = clients.iter().map(|client| {
            let params = super::make_completion_params(
                buffer,
                cursor,
                client.offset_encoding,
            );
            client.request_completions(params, buffer.bufnr)
        });

        let items = future::join_all(requests)
            .await
            .into_iter()
            .filter_map(Result::ok)
            .flat_map(|response| match response {
                CompletionResponse::CompletionList(list) => list.items,
                CompletionResponse::CompletionItems(items) => items,
            })
            .collect::<Vec<LspCompletionItem>>();

        if items.is_empty() {
            return Ok(Vec::new());
        }

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        let completions = items
            .into_iter()
            .filter(|item| {
                item.label.starts_with(word_pre) && item.label != word_pre
            })
            .map(|item| {
                CompletionItem::from_lsp_item(
                    item,
                    &buffer.filetype,
                    self.buf_to_highlighter
                        .get_mut(&buffer.bufnr)
                        // .map(|hl| Arc::make_mut(hl)),
                )
            })
            .collect::<Completions>();

        Ok(completions)
    }
}
