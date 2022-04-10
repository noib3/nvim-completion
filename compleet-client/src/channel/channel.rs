use std::sync::Arc;
use std::{cell::RefCell, rc::Rc, thread};

use mlua::prelude::{Lua, LuaResult};
use nix::{
    sys::signal::{self, Signal},
    unistd::Pid,
};
use parking_lot::Mutex;
use sources::{completion::Completions, cursor::Cursor, sources::Sources};
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{bindings::nvim, state::State, ui};

#[derive(Debug)]
pub struct Channel {
    /// TODO: docs
    handles: Vec<JoinHandle<()>>,

    /// TODO: docs
    sender: UnboundedSender<Completions>,

    /// TODO: docs
    runtime: Runtime,

    /// TODO: docs
    sources: Sources,
}

impl Channel {
    /// Creates a new unbounded channel to communicate with the main thread and
    /// spawns a tokio runtime.
    pub fn new(
        lua: &Lua,
        state: &Rc<RefCell<State>>,
        sources: Sources,
    ) -> LuaResult<Channel> {
        let this = Arc::new(Mutex::new(Vec::new()));

        // This is the callback that's executed when there are new completions
        // to be displayed.
        let cloned = state.clone();
        let that = this.clone();
        let callback = lua.create_function(move |lua, ()| {
            let new = {
                let this = &mut that.lock();
                this.drain(..).collect::<Completions>()
            };
            if !new.is_empty() {
                let state = cloned.clone();
                // Schedule a UI update.
                nvim::schedule(
                    lua,
                    lua.create_function(move |lua, ()| {
                        ui::update(lua, &mut state.borrow_mut(), new.clone())
                    })?,
                )?;
            }
            Ok(())
        })?;

        // Add an event listener on the SIGUSR2 signal along with the
        // associated callback.
        lua.load(
            r#"
            function(callback)
                local signal = vim.loop.new_signal()
                signal:start("sigusr2", callback)
            end
            "#,
        )
        .eval::<mlua::Function>()?
        .call(callback)?;

        let (sender, mut receiver) = mpsc::unbounded_channel::<Completions>();

        let _ = thread::spawn(move || {
            let rt = RuntimeBuilder::new_current_thread()
                .enable_time()
                .build()
                .expect("Couldn't create async runtime");

            rt.block_on(async {
                let pid = i32::try_from(std::process::id()).unwrap();
                loop {
                    if let Some(completions) = receiver.recv().await {
                        // TODO: place the completions inside the state
                        {
                            let that = &mut this.lock();
                            that.extend(completions);
                        }
                        // Notify the main thread that new completions are
                        // available by sending a SIGUSR2 signal.
                        signal::kill(Pid::from_raw(pid), Signal::SIGUSR2)
                            .unwrap();
                    }
                }
            })
        });

        let runtime = RuntimeBuilder::new_multi_thread()
            .enable_io()
            .enable_time()
            .build()
            .expect("Couldn't create async runtime");

        Ok(Channel {
            handles: Vec::new(),
            sender,
            runtime,
            sources,
        })
    }

    /// TODO: docs
    pub fn stop_tasks(&mut self) {
        self.handles.iter().for_each(|handle| handle.abort());
        self.handles.clear();
    }

    /// TODO: docs
    pub fn fetch_completions(&mut self, cursor: &Cursor) -> LuaResult<()> {
        let cursor = Arc::new(cursor.clone());
        for source in self.sources.iter() {
            let sender = self.sender.clone();
            let cur = cursor.clone();
            let source = source.clone();
            self.handles.push(self.runtime.spawn(async move {
                let completions = source.complete(&cur).await;
                if let Err(_) = sender.send(completions) {
                    // TODO: error handling
                }
            }));
        }
        Ok(())
    }
}
