use std::collections::HashMap;
use std::process::{Child, Command};

use async_trait::async_trait;
use bindings::{api, nvim};
use mlua::prelude::{Lua, LuaResult};

use super::LspConfig;
use crate::prelude::{CompletionItem, CompletionSource, Completions, Cursor};

#[derive(Debug, Default)]
pub struct Lsp {
    config: LspConfig,
    clients: HashMap<u16, Vec<Child>>,
}

impl From<LspConfig> for Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lsp {
    fn attach(&mut self, lua: &Lua, bufnr: u16) -> LuaResult<bool> {
        if self.clients.get(&bufnr).is_some() {
            return Ok(true);
        }

        let filetype = api::buf_get_option::<String>(lua, bufnr, "filetype")?;
        if filetype != "rust" {
            return Ok(false);
        }

        let anal = match Command::new("rust-analyzer").spawn() {
            Ok(child) => child,
            Err(_) => return Ok(false),
        };

        nvim::print(lua, anal.id())?;
        self.clients.insert(bufnr, vec![anal]);

        Ok(true)
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
