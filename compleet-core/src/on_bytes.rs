use nvim_oxi::opts::{OnBytesArgs, ShouldDetach};

use crate::{edit::Edit, Client};

/*

- helix
/helix-core/src/movement.rs -> is_word_boundary
/helix-core/src/chars.rs -> categorize_char


:h iskeyword
:lua =vim.api.nvim_buf_get_option(0, "iskeyword")

```lua
-- /runtime/lua/vim/lsp/handlers.lua L291
local line_to_cursor = "pub(crate) fn echo(msg:String"
local textMatch = vim.fn.match(line_to_cursor, '\\k*$')
local prefix = line_to_cursor:sub(textMatch + 1)
print(prefix) -- `String`
```
*/

pub(crate) fn on_bytes(
    client: &Client,
    args: OnBytesArgs,
) -> crate::Result<ShouldDetach> {
    client.apply_edit(&args.1, Edit::try_from(&args)?)?;
    Ok(false)
}
