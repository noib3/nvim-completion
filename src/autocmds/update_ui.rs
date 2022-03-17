use mlua::{Lua, Result};
use neovim::{Api, Neovim};

use crate::autocmds;
use crate::state::State;
use crate::ui::{completion_menu, positioning::WindowPosition};

// refactor
/// Executed on every `CursorMovedI` event.
pub fn update_ui(lua: &Lua, state: &mut State) -> Result<()> {
    let ui = &mut state.ui;

    // TODO: refactor, menu_position = None != no drawing instructions.
    //
    // If there aren't any drawing instructions there's not much to do. We just
    // cleanup the UI and return.
    if ui.draw_instructions.menu_position.is_none() {
        autocmds::cleanup_ui(lua, ui)?;
        return Ok(());
    }

    let menu_position = ui
        .draw_instructions
        .menu_position
        .as_ref()
        .expect("Already handled `None` case.");

    let api = &Neovim::new(lua)?.api;
    let menu = &mut ui.completion_menu;
    let completions = &state.completions;

    // If the completion menu's floating window is already visible we just move
    // it to the new position. If not we create it.
    if menu.is_visible() {
        move_floatwin(
            lua,
            &api,
            menu.winid
                .expect("The completion menu is visible so it has a winid."),
            menu_position,
        )?;
    } else {
        menu.winid = Some(completion_menu::create_floatwin(
            lua,
            &api,
            menu.bufnr,
            menu_position,
        )?);
    }

    // Next, we fill the completion menu's buffer with the completion results
    // and highlight the matched characters of every completion result.
    completion_menu::fill_buffer(
        lua,
        &api,
        menu.bufnr,
        menu.matched_chars_nsid,
        completions,
    )?;

    // Check if a completion hint should be displayed.
    if let Some(index) = ui.draw_instructions.hinted_index {
        let completion = &completions[index];
        let text = &completion.text[completion.matched_prefix_len..];
        ui.completion_hint.set(
            lua,
            &api,
            index,
            state.buffer.row,
            state.buffer.at_bytes,
            text,
        )?;
    }

    // After we've consumed all the instructions we reset them.
    ui.draw_instructions.reset();

    Ok(())
}

// refactor: move to ui module
/// Moves a floating window to a new position.
fn move_floatwin(
    lua: &Lua,
    api: &Api,
    winid: usize,
    position: &WindowPosition,
) -> Result<()> {
    let opts = lua.create_table_with_capacity(0, 5)?;
    opts.set("relative", "cursor")?;
    opts.set("width", position.width)?;
    opts.set("height", position.height)?;
    opts.set("row", position.row)?;
    opts.set("col", position.col)?;

    api.win_set_config(winid, opts)?;

    Ok(())
}
