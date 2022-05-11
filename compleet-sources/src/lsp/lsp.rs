use std::collections::HashMap;

use async_trait::async_trait;
use bindings::opinionated::lsp::protocol::CompletionResponse;
use bindings::opinionated::{lsp::LspClient, Buffer, Neovim};
use futures::future;
use mlua::Lua;
use treesitter_highlighter::Highlighter;

use super::{setup, LspConfig};
use crate::completion_source::{
    CompletionSource,
    ShouldAttach,
    ShouldRecompute,
};
use crate::prelude::{CompletionItem, Completions, Cursor};

#[derive(Debug, Default, Clone)]
pub struct Lsp {
    pub config: LspConfig,

    /// The Lsp clients attached to a buffer.
    pub _clients: HashMap<u16, Vec<LspClient>>,

    /// Maps buffer numbers to tree-sitter highlighters.
    pub buf_to_highlighter: HashMap<u16, Highlighter>,
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

        Ok(true)
    }

    fn on_edit(
        &mut self,
        _lua: &Lua,
        _buffer: &Buffer,
    ) -> crate::Result<ShouldRecompute> {
        Ok(true)
    }

    async fn complete(
        &mut self,
        nvim: &Neovim,
        cursor: &Cursor,
        buffer: &Buffer,
    ) -> crate::Result<Completions> {
        // let clients = &self._clients[&buffer.bufnr];
        let clients = nvim.lsp_buf_get_clients(buffer.bufnr).await;

        let requests = clients.iter().map(|client| {
            let params = super::make_completion_params(
                buffer,
                cursor,
                client.offset_encoding,
            );

            client.request_completions(params, buffer.bufnr)
        });

        Ok(future::join_all(requests)
            .await
            .into_iter()
            .filter_map(Result::ok)
            .flat_map(|response| match response {
                CompletionResponse::List(list) => list.items,
                CompletionResponse::Array(items) => items,
            })
            .map(|lsp_item| {
                CompletionItem::from_lsp_item(
                    lsp_item,
                    &buffer.filetype,
                    self.buf_to_highlighter.get_mut(&buffer.bufnr),
                )
            })
            .collect())
    }
}
