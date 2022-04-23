use mlua::prelude::{Lua, LuaResult};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot::Receiver,
};

use crate::request::BridgeRequest;
use crate::signal::Signal;

#[derive(Debug)]
pub struct LuaBridge {
    sender: UnboundedSender<BridgeRequest>,
    signal: Signal,
}

impl LuaBridge {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        // I could actually get away with a single-producer, single-consumer
        // channel, but that doesn't seem to be a thing.
        let (sender, mut receiver) =
            mpsc::unbounded_channel::<BridgeRequest>();

        let callback = lua.create_function_mut(move |lua, ()| {
            while let Ok(request) = receiver.try_recv() {
                request.handle(lua)?;
            }
            Ok(())
        })?;

        let signal = Signal::new(lua, callback)?;

        Ok(Self { sender, signal })
    }

    pub async fn send<T>(
        &self,
        request: BridgeRequest,
        receiver: Receiver<T>,
    ) -> T {
        let _ = self.sender.send(request);

        // self.sender
        //     .send(request)
        //     .expect("the Neovim receiver has been closed");

        self.signal.trigger();

        match receiver.await {
            Ok(val) => val,
            Err(_) => todo!("error handling"),
        }
    }
}
