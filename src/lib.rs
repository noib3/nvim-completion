use futures::future;
use nvim_rs::{compat::tokio::Compat, Neovim};
use tokio::io::Stdout;

pub mod completion;
use completion::CompletionItem;

// mod debugging;
// use debugging::nvim_echo::nvim_echo;

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
    if let Some(index) = ui_state.completion_menu.selected_index {
        future::join4(
            ui_state.completion_menu.hide(),
            ui_state.details_pane.hide(),
            ui_state.virtual_text.erase(),
            insertion::insert_completion(
                nvim,
                current_line,
                bytes_before_cursor,
                &completion_items[index],
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
        }
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
        }
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

    if let Some(index) = ui_state.completion_menu.selected_index {
        ui_state.completion_menu.selected_index =
            Some(std::cmp::min(index, completion_items.len() - 1))
    }

    ui_state
        .completion_menu
        .show_completions(nvim, completion_items)
        .await;
}
