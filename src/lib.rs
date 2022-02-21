use std::cmp;

use futures::future;
use nvim_rs::{compat::tokio::Compat, Neovim};
use tokio::io::Stdout;

pub mod completion;
use completion::CompletionItem;

// mod debugging;
// use debugging::nvim_echo;

mod insertion;

pub mod ui;
use ui::UIState;

pub type Nvim = Neovim<Writer>;

pub type Writer = Compat<Stdout>;

pub async fn accept_completion(
    nvim: &Nvim,
    completion_items: &mut Vec<CompletionItem>,
    ui_state: &mut UIState,
    current_line: &str,
    bytes_before_cursor: u64,
) {
    if let Some(selected_index) = ui_state.completion_menu.selected_index {
        let line_after_cursor = &current_line[bytes_before_cursor as usize..];

        let (current_buffer, current_window, (start_col, replacement)) =
            future::join3(
                nvim.get_current_buf(),
                nvim.get_current_win(),
                async {
                    insertion::get_completion(
                        // TODO: get matched_prefix here
                        "hi",
                        line_after_cursor,
                        &completion_items[selected_index].text,
                    )
                },
            )
            .await;

        // Do I event need to get the current row? What if I use 0?
        let current_row =
            current_window.unwrap().get_position().await.unwrap().0 - 1;

        // We're assigning just to ignore the `Result` returned by
        // `current_buffer.set_text`.
        let (_, _, _, _) = future::join4(
            ui_state.completion_menu.hide(),
            ui_state.details_pane.hide(),
            ui_state.virtual_text.erase(),
            // See `:h nvim_buf_set_text` for the docs on how this works.
            current_buffer.unwrap().set_text(
                current_row,
                start_col as i64,
                current_row,
                // The end column (which `nvim_buf_set_text` interprets to be
                // bytes from the beginning of the line, not characters) is
                // always equal to `bytes_before_cursor`, meaning we never
                // mangle the text after the current cursor position.
                bytes_before_cursor as i64,
                vec![replacement.to_string()],
            ),
        )
        .await;

        completion_items.clear();
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
    completion_items: &mut Vec<CompletionItem>,
    current_line: &str,
    bytes_before_cursor: u64,
) -> bool {
    *completion_items =
        completion::complete(current_line, bytes_before_cursor);
    !completion_items.is_empty()
}

pub async fn select_next_completion(
    ui_state: &mut UIState,
    completion_items_len: usize,
) {
    if !ui_state.completion_menu.is_visible() {
        return;
    }

    ui_state.completion_menu.selected_index =
        match ui_state.completion_menu.selected_index {
            Some(index) if index == completion_items_len - 1 => None,
            Some(index) => Some(index + 1),
            None => Some(0),
        };
}

pub async fn select_prev_completion(
    ui_state: &mut UIState,
    completion_items_len: usize,
) {
    if !ui_state.completion_menu.is_visible() {
        return;
    }

    ui_state.completion_menu.selected_index =
        match ui_state.completion_menu.selected_index {
            Some(index) if index == 0 => None,
            Some(index) => Some(index - 1),
            None => Some(completion_items_len - 1),
        };
}

// TODO: how does this interact w/ virtual text?
pub async fn show_completions(
    nvim: &Nvim,
    completion_items: &mut Vec<CompletionItem>,
    ui_state: &mut UIState,
    _current_line: &str,
    _bytes_before_cursor: u64,
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
    // *completion_items =
    //     completion::complete(current_line, bytes_before_cursor);

    if completion_items.is_empty() {
        return;
    }

    ui_state
        .completion_menu
        .show_completions(nvim, completion_items)
        .await;
}

pub async fn text_changed(
    nvim: &Nvim,
    completion_items: &mut Vec<CompletionItem>,
    ui_state: &mut UIState,
    current_line: &str,
    bytes_before_cursor: u64,
) {
    *completion_items =
        completion::complete(current_line, bytes_before_cursor);

    if completion_items.is_empty() {
        ui_state.completion_menu.selected_index = None;
        return;
    }

    // TODO: I don't actually need this bc the completion_menu is hidden on
    // every cursor_moved and every completion_menu.hide() already sets the
    // selected index to None. Maybe I do if I decide it's not the
    // responsability of completion_menu.hide() to reset the selected index.
    if let Some(index) = ui_state.completion_menu.selected_index {
        ui_state.completion_menu.selected_index =
            Some(cmp::min(index, completion_items.len() - 1))
    }

    ui_state
        .completion_menu
        .show_completions(nvim, completion_items)
        .await;
}
