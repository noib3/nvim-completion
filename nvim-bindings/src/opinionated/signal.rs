use std::{io::Write, os::unix::io::IntoRawFd};

use mlua::{
    chunk,
    prelude::{Lua, LuaFunction, LuaResult},
};
use os_pipe::PipeWriter;

#[derive(Debug)]
pub struct Signal(PipeWriter);

impl Signal {
    /// Installs a Lua callback on the Neovim event loop, returning a `Signal`
    /// which can be used to trigger the callback.
    pub fn new(lua: &Lua, callback: LuaFunction) -> LuaResult<Self> {
        // Using pipes as described in
        // https://github.com/khvzak/mlua/issues/143#issuecomment-1094380794
        let (reader, writer) = os_pipe::pipe()?;

        let reader_fd = reader.into_raw_fd();
        lua.load(chunk! {
            local pipe = vim.loop.new_pipe()
            pipe:open($reader_fd)
            pipe:read_start(function(err, chunk)
                assert(not err, err)
                if chunk then vim.schedule($callback) end
            end)
        })
        .exec()?;

        Ok(Self(writer))
    }

    /// Triggers the callback associated with this signal.
    pub fn trigger(&self) {
        // We clone the writer so that we can take a `&self` instead of `&mut
        // self`.
        self.0.try_clone().unwrap().write_all(&[0]).unwrap()
    }
}
