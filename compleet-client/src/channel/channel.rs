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
use sources::prelude::{Completions, Cursor, Sources};
use tokio::{
    runtime::{Builder as RuntimeBuilder, Runtime},
    sync::mpsc::{self, UnboundedSender},
    task::JoinHandle,
};

use crate::{bindings::nvim, state::State, ui};

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

        // TODO: do we need this?
        let count = Arc::new(Mutex::new(0u8));

        let state = state.clone();

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
            let is_last = {
                let count = &mut *count.lock();
                if *count + arrived == num_sources {
                    *count = 0;
                    true
                } else {
                    *count += arrived;
                    false
                }
            };

            let state = state.clone();
            let mut completions = Some(completions);
            let update_ui = lua.create_function_mut(move |lua, ()| {
                ui::update(
                    lua,
                    &mut state.borrow_mut(),
                    completions.take().expect("this only gets called once"),
                    changedtick,
                    is_last,
                )
            })?;

            // Schedule a UI update.
            nvim::schedule(lua, update_ui)
        })?;

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

    /// TODO: docs
    pub fn should_attach(
        &mut self,
        _lua: &Lua,
        _bufnr: u16,
    ) -> LuaResult<bool> {
        Ok(true)
    }
}
