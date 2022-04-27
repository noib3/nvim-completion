use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use bindings::{
    api,
    opinionated::{
        lsp::{
            protocol::{CompletionParams, CompletionResponse},
            LspMethod,
        },
        Neovim,
    },
};
use mlua::prelude::{Lua, LuaResult};

use super::{hlgroups, treesitter, LspConfig};
use crate::prelude::{CompletionSource, Completions, Cursor, Result};

#[derive(Debug, Default)]
pub struct Lsp {
    config: LspConfig,
    bufnr_to_thingy: HashMap<u16, Arc<treesitter::Thingy>>,
    filetype_to_thingy: HashMap<String, Arc<treesitter::Thingy>>,
}

impl From<LspConfig> for Lsp {
    fn from(config: LspConfig) -> Self {
        Self { config, ..Default::default() }
    }
}

#[async_trait]
impl CompletionSource for Lsp {
    fn setup(&mut self, lua: &Lua) -> LuaResult<()> {
        hlgroups::setup(lua)
    }

    fn attach(&mut self, lua: &Lua, bufnr: u16) -> LuaResult<bool> {
        // TODO: check if buffer has any LSPs available.
        // vim.lsp.buf_is_attached

        if !self.config.highlight_completions {
            return Ok(true);
        }

        let filetype = api::buf_get_option::<String>(lua, bufnr, "filetype")?;

        if &filetype == "rust" {
            // Get the `Thingy` for the given filetype if it already exists, or
            // create and cache a new one if it doesn't.
            let thingy = match self.filetype_to_thingy.get(&filetype) {
                Some(thingy) => thingy.clone(),

                None => {
                    let thingy =
                        Arc::new(treesitter::Thingy::new(lua, &filetype)?);
                    self.filetype_to_thingy.insert(filetype, thingy.clone());
                    thingy
                },
            };

            self.bufnr_to_thingy.insert(bufnr, thingy);
        }

        Ok(true)
    }

    async fn complete(
        &self,
        nvim: &Neovim,
        cursor: &Cursor,
        bufnr: u16,
    ) -> Result<Completions> {
        // Get the query
        // :lua query = vim.treesitter.query.get_query("rust",
        // "highlights").query

        // Get the parser
        // :lua parser = vim.treesitter.get_string_parser("&mut Diocane",
        // "rust", {})

        // Get the tree
        // :lua tree = parser:parse()

        // Get the root node
        // :lua root = tree[1]:root()

        // Get the iterator
        // :lua iter = root:_rawquery(query, false, 0, 1)

        // call the iterator

        let client = match nvim.lsp_buf_get_clients(0).await {
            v if v.is_empty() => return Ok(Vec::new()),
            v => v.into_iter().nth(0).unwrap(),
        };

        let method = LspMethod::Completion(CompletionParams::new(
            nvim.api_buf_get_name(0).await,
            cursor.row as u32,
            cursor.bytes as u32,
        ));

        let items = match client.request(method, 0).await? {
            CompletionResponse::CompletionList(list) => list.items,
            CompletionResponse::CompletionItems(items) => items,
        };

        let word_pre = cursor.word_pre();

        if word_pre.is_empty() {
            return Ok(Vec::new());
        }

        let mut completions = items
            .into_iter()
            .filter(|item| {
                item.label.starts_with(word_pre) && item.label != word_pre
            })
            .map(|item| item.into())
            .collect::<Completions>();

        if let Some(thingy) = self.bufnr_to_thingy.get(&bufnr) {
            completions.iter_mut().for_each(|c| {
                let hl = thingy.highlight(&c.text);
                c.highlight_label(hl);
            });
        }

        Ok(completions)
    }
}
