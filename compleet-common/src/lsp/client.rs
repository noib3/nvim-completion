use std::sync::Arc;

use bindings::nvim;
use mlua::{
    prelude::{LuaError, LuaRegistryKey, LuaValue},
    Table,
};
use tokio::sync::oneshot;

use super::{protocol, LspMethod};
use crate::bridge::LuaBridge;

pub type Result<T> = std::result::Result<T, Error>;

/// TODO: docs
#[derive(Debug)]
pub enum Error {
    Lua(LuaError),
    Any(String),
}

impl From<LuaError> for Error {
    fn from(err: LuaError) -> Self {
        Self::Lua(err)
    }
}

/// Binding to `vim.lsp.client`.
#[derive(Debug)]
pub struct LspClient {
    // A bridge acting like a `mlua::Lua`.
    bridge: Arc<LuaBridge>,

    // We have to store all functions through their registry key because
    // `mlua::Function`s are neither 'static nor Send.
    request_key: Arc<LuaRegistryKey>,
}

/// The function signature of an Lsp handler as defined by the Neovim api (see
/// `:h lsp-handler`). Tuple is `(err, result, ctx)`.
type LspHandlerSignature<'lua> = (LuaValue<'lua>, Table<'lua>, Table<'lua>);

impl LspClient {
    /// TODO: docs
    pub fn new(bridge: Arc<LuaBridge>, req_key: LuaRegistryKey) -> Self {
        Self { bridge, request_key: Arc::new(req_key) }
    }

    /// Binding to `vim.lsp.client.request`.
    pub async fn request(
        &self,
        method: LspMethod,
        params: protocol::CompletionParams,
        bufnr: Option<u16>,
    ) -> Result<u32> {
        let (tx, rx) = oneshot::channel::<Result<u32>>();
        let mut tx = Some(tx);

        // let callback =
        //     move |lua, (maybe_err, result, _ctx): LspHandlerSignature| {
        //         let tx = tx.take().expect("this only gets called once");

        //         if maybe_err.type_name() == "table" {
        //             let infos = nvim::inspect(lua, maybe_err)?;
        //             let _ = tx.send(Err(self::Error::Any(infos)));
        //             return Ok(());
        //         }

        //         let num = result.get::<_, Table>("items")?.len()? as u32;
        //         let _ = tx.send(Ok(num));

        //         Ok(())
        //     };

        let _req_id = self
            .bridge
            .call_function::<_, u32>(
                &self.request_key,
                (method.to_string(), true, false, /* callback, */ bufnr),
            )
            .await;

        match rx.await {
            Err(_) => Err(self::Error::Any("Receiver dropped".into())),

            Ok(Err(_)) => Err(self::Error::Any("Something".into())),

            Ok(Ok(bool)) => Ok(bool),
        }
    }
}
