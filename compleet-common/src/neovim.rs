use mlua::prelude::{Lua, LuaResult};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot::{self, Receiver},
};

use super::{lsp, Request, Signal};

#[derive(Debug)]
pub struct Neovim {
    sender: UnboundedSender<Request>,
    signal: Signal,
}

impl Neovim {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Request>();

        let callback = lua.create_function_mut(move |lua, ()| {
            while let Ok(request) = receiver.try_recv() {
                request.handle(lua)?;
            }
            Ok(())
        })?;

        let signal = Signal::new(lua, callback)?;

        Ok(Self { sender, signal })
    }

    async fn send<T>(&self, request: Request, receiver: Receiver<T>) -> T {
        self.sender
            .send(request)
            .expect("the Neovim receiver has been closed");

        self.signal.trigger();

        match receiver.await {
            Ok(val) => val,
            Err(_) => todo!("error handling"),
        }
    }
}

impl Neovim {
    pub async fn api_get_current_buf(&self) -> u16 {
        let (responder, receiver) = oneshot::channel();
        let request = Request::ApiGetCurrentBuf(responder);
        self.send(request, receiver).await
    }

    pub async fn lsp_buf_get_clients(
        &self,
        bufnr: u16,
    ) -> Vec<lsp::LspClient> {
        let (responder, receiver) = oneshot::channel();
        let request = Request::LspBufGetClients(bufnr, responder);
        self.send(request, receiver).await
    }
}
