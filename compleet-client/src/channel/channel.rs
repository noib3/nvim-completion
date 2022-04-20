use std::{cell::RefCell, rc::Rc, sync::Arc};

use common::{Completions, Cursor, Neovim, Signal, Sources};
use mlua::prelude::{Lua, LuaResult};
use parking_lot::Mutex;
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{state::State, ui};

/// TODO: docs
#[derive(Debug)]
struct Msg {
    completions: Completions,
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
        state: &Rc<RefCell<State>>,
        sources: Sources,
    ) -> LuaResult<Channel> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<Msg>();

        let state = state.clone();

        // TODO: do we need this?
        let count = Arc::new(Mutex::new(0u8));

        // This is the callback that's executed when new bytes are written to
        // the pipe spawned w/ `vim.loop.new_pipe`.
        let callback = lua.create_function_mut(move |lua, ()| {
            let changedtick = state.borrow().changedtick_last_seen;

            // Pull all the messages sent to the receiver.
            let mut messages = Vec::<Msg>::new();
            while let Ok(msg) = receiver.try_recv() {
                // Only add the ones whose changedtick matches the last one set
                // in `super::on_bytes`.
                if msg.changedtick == changedtick {
                    messages.push(msg);
                }
            }

            // If all the messages are old we can return early.
            if messages.is_empty() {
                return Ok(());
            }

            // TODO: do we need this?
            let arrived = messages.len() as u8;
            let num_sources = messages[0].num_sources;

            let completions = messages
                .into_iter()
                .flat_map(|msg| msg.completions)
                .collect::<Completions>();

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
    ) {
        let num_sources = u8::try_from(self.sources.len()).unwrap();
        for source in &self.sources {
            let source = source.clone();
            let cursor = cursor.clone();
            let nvim = self.nvim.clone();
            let sender = self.sender.clone();
            let signal = self.signal.clone();
            self.handles.push(self.runtime.spawn(async move {
                let completions =
                    source.lock().await.complete(&nvim, &cursor).await;

                sender
                    .send(Msg { completions, changedtick, num_sources })
                    .expect("the receiver has been closed");

                // Signal Neovim that a source has sent its completions.
                signal.trigger();
            }));
        }
    }

    /// TODO: docs
    pub fn should_attach(&mut self, bufnr: u16) -> bool {
        let handles = self
            .sources
            .iter()
            .map(|source| {
                let source = source.clone();
                let nvim = self.nvim.clone();
                self.runtime.spawn(async move {
                    source.lock().await.attach(&nvim, bufnr).await
                })
            })
            .collect::<Vec<JoinHandle<bool>>>();

        let mut results = Vec::<bool>::with_capacity(self.sources.len());

        self.runtime.block_on(async {
            for handle in handles {
                if let Ok(has_attached) = handle.await {
                    results.push(has_attached);
                }
            }
        });

        results.into_iter().any(|has_attached| has_attached)
    }
}
