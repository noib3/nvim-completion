use std::{
    cell::RefCell,
    io::Write,
    os::unix::io::IntoRawFd,
    rc::Rc,
    sync::Arc,
};

use mlua::{
    chunk,
    prelude::{Lua, LuaResult},
};
use os_pipe::PipeWriter;
use parking_lot::Mutex;
use sources::{completion::Completions, cursor::Cursor, sources::Sources};
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{bindings::nvim, state::State, ui};

#[derive(Debug)]
struct Msg {
    completions: Completions,
    changedtick: u32,
    num_sources: u8,
}

#[derive(Debug)]
pub struct Channel {
    /// TODO: docs
    handles: Vec<JoinHandle<()>>,

    /// TODO: docs
    sender: UnboundedSender<Msg>,

    /// TODO: docs
    writer: Arc<Mutex<PipeWriter>>,

    /// TODO: docs
    runtime: Runtime,

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
        let (reader, writer) = os_pipe::pipe()?;

        // TODO: refactor from here --
        let cloned = state.clone();
        let ctick = Arc::new(Mutex::new(0u32));
        let count = Arc::new(Mutex::new(0u8));

        let callback = lua.create_function_mut(move |lua, ()| {
            let state = cloned.clone();

            if let Ok(Msg { completions, changedtick, num_sources }) =
                receiver.try_recv()
            {
                if changedtick != state.borrow().changedtick_last_seen {
                    return Ok(());
                }

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
                            completions.clone(),
                            changedtick,
                            is_last,
                        )
                    })?,
                )?;
            }

            Ok(())
        })?;
        // to here --

        // Setup a pipe on the Neovim side that executes the callback when new
        // data is written to it.
        let reader_fd = reader.into_raw_fd();
        lua.load(chunk! {
            local pipe = vim.loop.new_pipe()
            pipe:open($reader_fd)
            pipe:read_start(function(err, chunk)
                assert(not err, err)
                if chunk then $callback() end
            end)
        })
        .exec()?;

        let runtime = RuntimeBuilder::new_multi_thread()
            .enable_all()
            .build()
            .expect("couldn't create tokio runtime");

        Ok(Channel {
            writer: Arc::new(Mutex::new(writer)),
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
            let sender = self.sender.clone();
            let writer = self.writer.clone();
            self.handles.push(self.runtime.spawn(async move {
                let completions = source.complete(&cursor).await;
                sender
                    .send(Msg { completions, changedtick, num_sources })
                    .expect("the receiver has been closed");
                // Signal Neovim that a source has sent its completions.
                writer.lock().write_all(&[0]).unwrap();
            }));
        }
    }
}
