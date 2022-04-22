use mlua::prelude::{
    FromLuaMulti,
    Lua,
    LuaRegistryKey,
    LuaResult,
    ToLuaMulti,
};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot::{self, Receiver},
};

use super::Signal;

// type Func = bool;
// Box<dyn 'static + MaybeSend + FnMut(&'static Lua, ()) -> LuaResult<()>>;

pub type Responder<T> = oneshot::Sender<T>;

// pub enum Msg {
//     MakeFunc(Func, Responder<mlua::Function<'static>>),
// }

#[derive(Debug)]
pub struct LuaBridge {
    // sender: UnboundedSender<Msg>,
    sender: UnboundedSender<u32>,
    signal: Signal,
}

impl LuaBridge {
    pub fn new(lua: &Lua) -> LuaResult<Self> {
        todo!()

        // let (sender, mut receiver) = mpsc::unbounded_channel::<Msg>();

        // let callback = lua.create_function_mut(move |lua, ()| {
        //     while let Ok(msg) = receiver.try_recv() {
        //         match msg {
        //             Msg::MakeFunc(f, resp) => {
        //                 let lua: &'static Lua =
        //                     unsafe { std::mem::transmute(lua) };
        //                 let a = lua.create_function_mut(f)?;
        //                 let _ = resp.send(a);
        //                 Ok(())
        //             },
        //         }
        //     }
        //     Ok(())
        // })?;

        // let signal = Signal::new(lua, callback)?;

        // Ok(Self { sender, signal })
    }

    pub async fn call_function<'lua, A, R>(
        &self,
        func_key: &LuaRegistryKey,
        args: A,
    ) -> R
    where
        A: ToLuaMulti<'lua>,
        R: FromLuaMulti<'lua>,
    {
        let (tx, mut rx) = oneshot::channel::<R>();

        todo!()
    }
}
