use std::{cell::RefCell, rc::Rc, sync::Arc};

use bindings::opinionated::{Buffer, Neovim, Signal};
use mlua::prelude::{Lua, LuaResult};
use parking_lot::Mutex;
use sources::prelude::{Completions, Cursor, Result, Sources};
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{client::Client, messages, ui};

/// TODO: docs
struct Msg {
    completions: Result<Completions>,
    changedtick: u32,
    num_sources: u8,
}

/// TODO: docs
#[derive(Debug)]
pub struct Channel {
    /// TODO: docs
    handles: Vec<JoinHandle<()>>,

    /// TODO: docs
    nvim: Arc<Neovim>,

    /// TODO: docs
    runtime: Runtime,

    /// TODO: docs
    sender: UnboundedSender<Msg>,

    /// TODO: docs
    signal: Arc<Signal>,

    /// TODO: docs
    sources: Sources,
}

impl Channel {
    /// TODO: docs
    pub fn new(
        lua: &Lua,
        state: &Rc<RefCell<Client>>,
        sources: Sources,
    ) -> LuaResult<Channel> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Msg>();

        let state = state.clone();

        // TODO: do we need this?
        let count = Arc::new(Mutex::new(0u8));

        let callback = lua.create_function_mut(move |lua, ()| {
            let changedtick = state.borrow().changedtick_last_seen;

            // TODO: do we need this?
            let mut arrived = 0;
            let mut num_sources = 0;

            let mut completions = Vec::new();

            // Go over all the messages sent to the receiver.
            while let Ok(Msg {
                completions: maybe_cmp,
                changedtick: ct,
                num_sources: num,
            }) = receiver.try_recv()
            {
                // Only add the completions whose changedtick matches the last
                // one set in `super::on_bytes`.
                if ct == changedtick {
                    // TODO: do we need this?
                    arrived += 1;
                    num_sources = num;

                    match maybe_cmp {
                        Ok(compl) => completions.extend(compl),
                        Err(err) => messages::echowarn!(lua, "{err}")?,
                    }
                }
            }

            // If all the messages are old we can return early.
            if arrived == 0 {
                return Ok(());
            }

            // TODO: do we need this?
            let has_last = {
                let count = &mut *count.lock();
                if *count + arrived == num_sources {
                    *count = 0;
                    true
                } else {
                    *count += arrived;
                    false
                }
            };

            ui::update(
                lua,
                &mut state.borrow_mut(),
                completions,
                changedtick,
                has_last,
            )
        })?;

        let runtime = RuntimeBuilder::new_multi_thread()
            .enable_all()
            .build()
            .expect("couldn't create tokio runtime");

        runtime.block_on(async {
            for source in &sources {
                if let Err(_) = source.lock().await.setup(lua) {
                    todo!("error handling");
                }
            }
        });

        Ok(Channel {
            signal: Arc::new(Signal::new(lua, callback)?),
            nvim: Arc::new(Neovim::new(lua)?),
            handles: Vec::new(),
            sender,
            runtime,
            sources,
        })
    }

    /// TODO: docs
    pub fn stop_tasks(&mut self) {
        self.handles.drain(..).for_each(|handle| handle.abort())
    }

    /// TODO: docs
    pub fn fetch_completions(
        &mut self,
        cursor: Arc<Cursor>,
        changedtick: u32,
        buffer: Arc<Buffer>,
    ) {
        let num_sources = u8::try_from(self.sources.len()).unwrap();
        for source in &self.sources {
            let source = source.clone();
            let cursor = cursor.clone();
            let buffer = buffer.clone();
            let nvim = self.nvim.clone();
            let sender = self.sender.clone();
            let signal = self.signal.clone();
            self.handles.push(self.runtime.spawn(async move {
                let completions = source
                    .lock()
                    .await
                    .complete(&nvim, &cursor, &buffer)
                    .await;

                let _ =
                    sender.send(Msg { completions, changedtick, num_sources });

                // sender
                //     .send(Msg { completions, changedtick, num_sources })
                //     .expect("the receiver has been closed");

                // Signal Neovim that a source has sent its completions.
                signal.trigger();
            }));
        }
    }

    /// TODO: docs
    // pub fn should_attach(&mut self, bufnr: u32) -> bool {
    pub fn should_attach(
        &mut self,
        lua: &Lua,
        buffer: &Buffer,
    ) -> LuaResult<bool> {
        // let handles = self
        //     .sources
        //     .iter()
        //     .map(|source| {
        //         let source = source.clone();
        //         let nvim = self.nvim.clone();
        //         self.runtime.spawn(async move {
        //             source.lock().await.attach(&nvim, bufnr).await
        //         })
        //     })
        //     .collect::<Vec<JoinHandle<bool>>>();

        // let mut results = Vec::<bool>::with_capacity(self.sources.len());

        // self.runtime.block_on(async {
        //     for handle in handles {
        //         if let Ok(has_attached) = handle.await {
        //             results.push(has_attached);
        //         }
        //     }
        // });

        // results.into_iter().any(|has_attached| has_attached)

        let results = self
            .sources
            .iter()
            .flat_map(|source| {
                let source = source.clone();
                self.runtime.block_on(async {
                    source.lock().await.on_buf_enter(lua, buffer)
                })
            })
            .collect::<Vec<bool>>();

        Ok(results.into_iter().any(|has_attached| has_attached))
    }
}
