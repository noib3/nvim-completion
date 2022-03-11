use mlua::{Function, Result};

use super::Api;

impl<'a> Api<'a> {
    /// Binding to `vim.api.nvim_command`.
    ///
    /// Executes an ex-command.
    ///
    /// # Arguments
    ///
    /// * `cmd`  The command to execute
    pub fn command(&self, cmd: &str) -> Result<()> {
        Ok(self
            .0
            .get::<&str, Function>("nvim_command")?
            .call::<_, ()>(cmd)?)
    }
}
