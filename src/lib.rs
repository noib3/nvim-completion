use std::cmp;

use futures::future;
use nvim_rs::{compat::tokio::Compat, Neovim};
use tokio::io::Stdout;

pub mod completion;
use completion::CompletionState;

// mod debugging;
// use debugging::nvim_echo;

mod insertion;

pub mod ui;
use ui::UIState;

pub type Nvim = Neovim<Writer>;

pub type Writer = Compat<Stdout>;

pub async fn accept_completion(
    nvim: &Nvim,
    completion_state: &mut CompletionState,
    ui_state: &mut UIState,
) {
    if let Some(selected_index) = ui_state.completion_menu.selected_index {
        let line_after_cursor = &completion_state.current_line
            [completion_state.bytes_before_cursor..];

        let (current_buffer, current_window, (start_col, replacement)) =
            future::join3(
                nvim.get_current_buf(),
                nvim.get_current_win(),
                async {
                    insertion::get_completion(
                        &completion_state.matched_prefix,
                        line_after_cursor,
                        &completion_state.completion_items[selected_index]
                            .text,
                    )
                },
            )
            .await;

        let current_window = &current_window.unwrap();
        let current_row = current_window.get_cursor().await.unwrap().0 - 1;

        let start_col = (completion_state.bytes_before_cursor
            - completion_state.matched_prefix.len()
            + start_col) as i64;

        // The end column (which `nvim_buf_set_text` interprets to be
        // bytes from the beginning of the line, not characters) is
        // always equal to `bytes_before_cursor`, meaning we never
        // mangle the text after the current cursor position.
        let end_col = completion_state.bytes_before_cursor as i64;

        let shift_the_cursor_this_many_bytes =
            completion_state.completion_items[selected_index].text.len()
                - completion_state.matched_prefix.len();

        // nvim_echo(nvim, &format!("Row: {current_row}, Start col: {start_col}, End col: {end_col}, Replacement: '{replacement}', Shift bytes: {shift_the_cursor_this_many_bytes}"), "Normal", true).await;

        // We're assigning just to ignore the `Result` returned by
        // `current_buffer.set_text`.
        let (_, _, _, _, _) = future::join5(
            ui_state.completion_menu.hide(),
            // TODO: why doesn't the window hide if I don't move the cursor?
            ui_state.details_pane.hide(),
            ui_state.virtual_text.erase(),
            // See `:h nvim_buf_set_text` for the docs on how this works.
            current_buffer.unwrap().set_text(
                current_row,
                start_col,
                current_row,
                end_col,
                vec![replacement.to_string()],
            ),
            current_window.set_cursor((
                current_row + 1,
                (completion_state.bytes_before_cursor
                    + shift_the_cursor_this_many_bytes) as i64,
            )),
        )
        .await;

        completion_state.completion_items.clear();
    }
}

pub async fn cursor_moved(ui_state: &mut UIState) {
    future::join3(
        ui_state.completion_menu.hide(),
        ui_state.details_pane.hide(),
        ui_state.virtual_text.erase(),
    )
    .await;
}

pub async fn insert_left(ui_state: &mut UIState) {
    future::join3(
        ui_state.completion_menu.hide(),
        ui_state.details_pane.hide(),
        ui_state.virtual_text.erase(),
    )
    .await;
}

pub fn has_completions(
    completion_state: &mut CompletionState,
    current_line: &str,
    bytes_before_cursor: u64,
) -> bool {
    completion_state.current_line = current_line.to_string();
    completion_state.bytes_before_cursor = bytes_before_cursor as usize;
    completion_state.matched_prefix =
        completion::get_matched_prefix(current_line, bytes_before_cursor);
    completion_state.completion_items =
        completion::complete(&completion_state.matched_prefix);

    !completion_state.completion_items.is_empty()
}

pub async fn select_next_completion(
    ui_state: &mut UIState,
    completion_items_len: usize,
) {
    if !ui_state.completion_menu.is_visible() {
        return;
    }

    let new_selected_index = match ui_state.completion_menu.selected_index {
        Some(index) if index == completion_items_len - 1 => None,
        Some(index) => Some(index + 1),
        None => Some(0),
    };

    ui_state
        .completion_menu
        .update_selected_completion(new_selected_index)
        .await;
}

pub async fn select_prev_completion(
    ui_state: &mut UIState,
    completion_items_len: usize,
) {
    if !ui_state.completion_menu.is_visible() {
        return;
    }

    let new_selected_index = match ui_state.completion_menu.selected_index {
        Some(index) if index == 0 => None,
        Some(index) => Some(index - 1),
        None => Some(completion_items_len - 1),
    };

    ui_state
        .completion_menu
        .update_selected_completion(new_selected_index)
        .await;
}

// TODO: how does this interact w/ virtual text?
pub async fn show_completions(
    nvim: &Nvim,
    completion_state: &mut CompletionState,
    ui_state: &mut UIState,
) {
    if ui_state.completion_menu.is_visible() {
        return;
    }

    // TODO: do I need this? This function should only be called when the
    // completion menu is currently hidden and we want to show the completions
    // (e.g. the user moved the cursor and now wants to get completions at the
    // current cursor position), ideally after checking `has_completions`,
    // which already updates the `completion_items`, so reupdating them again
    // is wasteful.
    //
    // However we actually have no way to control that that's what all users
    // will do, nor is there any obvious way to enforce it. But calling it
    // twice just penalizes the users that use this feature properly, which
    // isn't fair. Hmmmm.
    //
    // completion_state.completion_items =
    //     completion::complete(current_line, bytes_before_cursor);

    if !completion_state.completion_items.is_empty() {
        ui_state
            .completion_menu
            .show_completions(nvim, &completion_state.completion_items)
            .await;
    }
}

pub async fn text_changed(
    nvim: &Nvim,
    completion_state: &mut CompletionState,
    ui_state: &mut UIState,
    current_line: &str,
    bytes_before_cursor: u64,
) {
    completion_state.current_line = current_line.to_string();
    completion_state.bytes_before_cursor = bytes_before_cursor as usize;
    completion_state.matched_prefix =
        completion::get_matched_prefix(current_line, bytes_before_cursor);
    completion_state.completion_items =
        completion::complete(&completion_state.matched_prefix);

    if completion_state.completion_items.is_empty() {
        ui_state.completion_menu.selected_index = None;
        return;
    }

    // TODO: I don't actually need this bc the completion_menu is hidden on
    // every cursor_moved and every completion_menu.hide() already sets the
    // selected index to None. Maybe I do if I decide it's not the
    // responsability of completion_menu.hide() to reset the selected index.
    if let Some(index) = ui_state.completion_menu.selected_index {
        ui_state.completion_menu.selected_index =
            Some(cmp::min(index, completion_state.completion_items.len() - 1))
    }

    ui_state
        .completion_menu
        .show_completions(nvim, &completion_state.completion_items)
        .await;
}
