use std::io::Write;
use std::os::unix::io::IntoRawFd;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc, thread};

use mlua::{
    chunk,
    prelude::{Lua, LuaResult},
};
// use nix::{
//     sys::signal::{self, Signal},
//     unistd::Pid,
// };
// use os_pipe::PipeWriter;
use parking_lot::Mutex;
use sources::{completion::Completions, cursor::Cursor, sources::Sources};
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{bindings::nvim, state::State, ui};

/// TODO: REFACTOR THE WHOLE FILE, IT'S A MESS!!

#[derive(Debug)]
pub struct Channel {
    /// TODO: docs
    handles: Vec<JoinHandle<()>>,

    /// TODO: docs
    sender: UnboundedSender<(Completions, u32, u32)>,

    /// TODO: docs
    runtime: Runtime,

    /// TODO: docs
    sources: Sources,
    // /// TODO: docs
    // writer: Arc<PipeWriter>,
}

impl Channel {
    /// Creates a new unbounded channel to communicate with the main thread and
    /// spawns a tokio runtime.
    pub fn new(
        lua: &Lua,
        state: &Rc<RefCell<State>>,
        sources: Sources,
    ) -> LuaResult<Channel> {
        let this = Arc::new(Mutex::new((Vec::new(), 0u32, 0u32)));
        // This is the callback that's executed when there are new completions
        // to be displayed.
        let cloned = state.clone();
        let that = this.clone();

        let ctick = Arc::new(Mutex::new(0u32));
        let count = Arc::new(Mutex::new(0u32));

        let callback = lua.create_function(move |lua, ()| {
            let state = cloned.clone();
            let (new, changedtick, num_sources) = {
                let this = &mut that.lock();
                (this.0.drain(..).collect::<Completions>(), this.1, this.2)
            };

            // TODO: omg refactor what even is this.
            let is_last = {
                let tick = &mut *ctick.lock();
                let num = &mut *count.lock();

                if changedtick != *tick {
                    *tick = changedtick;
                    *num = 1;
                    num_sources == 1
                } else {
                    *num += 1;
                    if *num == num_sources {
                        *num = 1;
                        true
                    } else {
                        false
                    }
                }
            };

            // Schedule a UI update.
            nvim::schedule(
                lua,
                lua.create_function(move |lua, ()| {
                    ui::update(
                        lua,
                        &mut state.borrow_mut(),
                        new.clone(),
                        changedtick,
                        is_last,
                    )
                })?,
            )?;
            Ok(())
        })?;

        // r#"
        // function(callback)
        //     local signal = vim.loop.new_signal()
        //     signal:start("sigusr2", callback)
        // end
        // "#,
        // )
        // .eval::<mlua::Function>()?
        // .call(callback)?;

        let (sender, mut receiver) =
            mpsc::unbounded_channel::<(Completions, u32, u32)>();

        let (reader, mut writer) = os_pipe::pipe()?;

        // Add an event listener on the SIGUSR2 signal along with the
        // associated callback.
        let read_fd = reader.into_raw_fd();
        lua.load(chunk! {
            local read_pipe = vim.loop.new_pipe()
            read_pipe:open($read_fd)
            read_pipe:read_start(function(err, chunk)
                assert(not err, err)
                if chunk then
                    for _ = 1,string.len(chunk) do
                        $callback()
                    end
                end
            end)
        })
        .exec()?;

        let _ = thread::spawn(move || {
            let rt = RuntimeBuilder::new_current_thread()
                .enable_time()
                .build()
                .expect("Couldn't create async runtime");

            rt.block_on(async {
                // let pid = i32::try_from(std::process::id()).unwrap();
                loop {
                    if let Some((completions, changedtick, num_sources)) =
                        receiver.recv().await
                    {
                        {
                            let that = &mut this.lock();
                            // Clear any completions that haven't been
                            // consumed?
                            // TODO: test this w/ multiple sources arriving at
                            // the same moment.
                            // that.0.extend(completions);
                            that.0 = completions;
                            that.1 = changedtick;
                            that.2 = num_sources;
                        }
                        // Signal Neovim that a source has sent its
                        // completions.
                        writer.write_all(&[b'1']).unwrap();

                        // // Notify the main thread that new completions are
                        // // available by sending a SIGUSR2 signal.
                        // signal::kill(Pid::from_raw(pid), Signal::SIGUSR2)
                        //     .unwrap();
                    }
                }
            })
        });

        let runtime = RuntimeBuilder::new_multi_thread()
            .enable_all()
            .build()
            .expect("couldn't create async runtime");

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
    pub fn fetch_completions(
        &mut self,
        cursor: &Cursor,
        changedtick: u32,
    ) -> LuaResult<()> {
        let cursor = Arc::new(cursor.clone());
        let num_sources = u32::try_from(self.sources.len()).unwrap();
        for source in self.sources.iter() {
            let sender = self.sender.clone();
            let cur = cursor.clone();
            let source = source.clone();
            // let writer = self.writer.clone();
            self.handles.push(self.runtime.spawn(async move {
                let completions = source.complete(&cur).await;
                if let Err(_) =
                    sender.send((completions, changedtick, num_sources))
                {
                    // TODO: error handling
                }
            }));
        }
        Ok(())
    }
}
