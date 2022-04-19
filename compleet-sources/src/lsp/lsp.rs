use std::collections::HashMap;

use async_trait::async_trait;
use bindings::nvim;
use mlua::{
    prelude::{Lua, LuaFunction, LuaResult},
    Table,
};

use super::LspConfig;
use crate::prelude::{CompletionItem, CompletionSource, Completions, Cursor};

#[derive(Debug, Default)]
pub struct Lsp {
    config: LspConfig,
    clients: HashMap<u16, usize>,
}

impl From<LspConfig> for Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lsp {
    fn attach(&mut self, lua: &Lua, bufnr: u16) -> LuaResult<bool> {
        let clients = lua
            .globals()
            .get::<_, Table>("vim")?
            .get::<_, Table>("lsp")?
            .get::<_, LuaFunction>("buf_get_clients")?
            .call::<_, Table>(bufnr)?
            .len()? as usize;

        self.clients.insert(bufnr, clients);
        nvim::print(lua, format!("{:?}", self.clients))?;

        Ok(clients != 0)
    }

    async fn complete(&self, cursor: &Cursor) -> Completions {
        // Simulate a slow source, this shouldn't block.
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        let word_pre = cursor.word_pre();
        if word_pre.is_empty() {
            return Vec::new();
        }

        let test = &self.config.test;
        if test.starts_with(word_pre) && test != word_pre {
            vec![CompletionItem {
                details: None,
                format: format!(" {} ", test),
                matched_bytes: vec![0..word_pre.len()],
                matched_prefix: word_pre.len() as u32,
                source: "Lsp",
                text: test.clone(),
            }]
        } else {
            Vec::new()
        }
    }
}
