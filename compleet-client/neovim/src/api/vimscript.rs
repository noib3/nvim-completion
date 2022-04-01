use mlua::{FromLuaMulti, Function, Result, ToLua};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_call_function`.
    ///
    /// Calls a VimL function with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `fun`    Name of the function to call.
    /// * `args`   Function arguments packed in a slice.
    pub fn call_function<A: ToLua<'a>, R: FromLuaMulti<'a>>(
        &self,
        fun: &str,
        args: Vec<A>,
    ) -> Result<R> {
        self.0
            .get::<&str, Function>("nvim_call_function")?
            .call::<(_, Vec<A>), R>((fun, args))
    }

    /// Binding to `vim.api.nvim_command`.
    ///
    /// Executes an ex-command.
    ///
    /// # Arguments
    ///
    /// * `cmd`  The command to execute.
    pub fn command(&self, cmd: &str) -> Result<()> {
        self.0
            .get::<&str, Function>("nvim_command")?
            .call::<_, ()>(cmd)
    }
}
