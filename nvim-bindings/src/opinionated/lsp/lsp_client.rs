use std::sync::Arc;

use mlua::prelude::{
    Lua,
    LuaFunction,
    LuaRegistryKey,
    LuaResult,
    LuaSerdeExt,
    LuaTable,
    LuaValue,
};
use tokio::sync::oneshot;

use super::protocol::{
    CompletionItem,
    CompletionList,
    CompletionParams,
    CompletionResponse,
    PositionEncodingKind,
    ResponseError,
};
use super::LspResult;
use crate::opinionated::{BridgeRequest, LspHandler, LuaBridge};

/// Acts as an abstraction over a Neovim Lsp client (see `:h vim.lsp.client`).
#[derive(Clone, Debug)]
pub struct LspClient {
    bridge: Arc<LuaBridge>,

    // We have to store all functions through their registry key because
    // `mlua::Function`s are neither 'static nor Send.
    /// The `mlua::RegistryKey` of the client's `request` function.
    request_key: Arc<LuaRegistryKey>,

    /// The id allocated to the client.
    pub id: u16,

    /// Name of the client if specified, client id otherwise.
    pub name: String,

    // TODO: make this into an enum
    /// The encoding used to communicate w/ the server.
    pub offset_encoding: PositionEncodingKind,
}

/// The function signature of an Lsp handler as defined by the Neovim API (see
/// `:h lsp-handler`). The tuple is `(err, result, ctx)`.
pub type LspHandlerSignature<'lua> =
    (Option<LuaTable<'lua>>, Option<LuaTable<'lua>>, LuaTable<'lua>);

impl LspClient {
    /// TODO: docs
    pub fn new(
        lua: &Lua,
        bridge: Arc<LuaBridge>,
        table: LuaTable,
    ) -> LuaResult<Self> {
        Ok(Self {
            bridge,
            request_key: Arc::new(lua.create_registry_value(
                table.get::<_, LuaFunction>("request")?,
            )?),
            id: table.get("id")?,
            name: table.get("name")?,
            offset_encoding: lua.from_value(table.get("offset_encoding")?)?,
        })
    }

    /// Binding to `vim.lsp.client.request` specialized for completions.
    pub async fn request_completions(
        &self,
        params: CompletionParams,
        bufnr: u32,
    ) -> LspResult<CompletionResponse> {
        let (tx, rx) = oneshot::channel::<LspResult<CompletionResponse>>();
        let mut tx = Some(tx);

        // This gets executed by Neovim when the response message arrives from
        // the server.
        let handler: LspHandler = Box::new(move |lua, (err, result, _ctx)| {
            let result = match result {
                // If the `isIncomplete` key is present deserialize the result
                // table as a `CompletionList`.
                Some(t) if t.contains_key("isIncomplete")? => {
                    let val = LuaValue::Table(t);
                    let list = lua.from_value::<CompletionList>(val)?;
                    Ok(CompletionResponse::List(list))
                },

                // If it isn't deserialize it as a `Vec<CompletionItem>`.
                Some(t) => {
                    let val = LuaValue::Table(t);
                    let items = lua.from_value::<Vec<CompletionItem>>(val)?;
                    Ok(CompletionResponse::Array(items))
                },

                // If there's no result then the `err` table *should* be
                // populated..
                None if err.is_some() => {
                    let val = LuaValue::Table(err.unwrap());
                    let error = lua.from_value::<ResponseError>(val)?;
                    Err(error.into())
                },

                // ..but apparently sometimes you get neither a `result` nor an
                // `err`.
                _ => todo!(),
            };

            let _ =
                tx.take().expect("this only gets called once").send(result);

            Ok(())
        });

        let (responder, receiver) = oneshot::channel();

        let request = BridgeRequest::LspClientRequestCompletions {
            req_key: self.request_key.clone(),
            handler,
            params,
            bufnr,
            responder,
        };

        let _request_id = self.bridge.send(request, receiver).await?;

        rx.await?
    }
}
