use async_trait::async_trait;
use futures::lock::Mutex;
use rmpv::Value;
use std::sync::Arc;

use compleet::{completion::CompletionItem, ui::UIState};

#[derive(Clone)]
pub struct NeovimHandler {
    completion_items: Arc<Mutex<Vec<CompletionItem>>>,
    ui_state: Arc<Mutex<UIState>>,
}

impl NeovimHandler {
    fn new() -> Self {
        NeovimHandler {
            completion_items: Arc::new(Mutex::new(Vec::new())),
            ui_state: Arc::new(Mutex::new(UIState::new())),
        }
    }
}

#[async_trait]
impl nvim_rs::Handler for NeovimHandler {
    type Writer = compleet::Writer;

    async fn handle_notify(
        &self,
        method: String,
        args: Vec<Value>,
        nvim: compleet::Nvim,
    ) {
        let completion_items = &mut *self.completion_items.lock().await;
        let ui_state = &mut *self.ui_state.lock().await;
        match method.as_str() {
            "accept_completion" => {
                // TODO: I shouldn't need any args here. All that I need should
                // be saved in a `CompletionState` struct.
                let current_line = args[0].as_str().unwrap_or("");
                let bytes_before_cursor = args[1].as_u64().unwrap_or(0);
                compleet::accept_completion(
                    &nvim,
                    completion_items,
                    ui_state,
                    current_line,
                    bytes_before_cursor,
                )
                .await
            },

            "cursor_moved" => compleet::cursor_moved(ui_state).await,

            "insert_left" => compleet::insert_left(ui_state).await,

            "select_next_completion" => {
                compleet::select_next_completion(
                    ui_state,
                    completion_items.len(),
                )
                .await
            },

            "select_prev_completion" => {
                compleet::select_prev_completion(
                    ui_state,
                    completion_items.len(),
                )
                .await
            },

            "show_completions" => {
                let current_line = args[0].as_str().unwrap_or("");
                let bytes_before_cursor = args[1].as_u64().unwrap_or(0);
                compleet::show_completions(
                    &nvim,
                    completion_items,
                    ui_state,
                    current_line,
                    bytes_before_cursor,
                )
                .await
            },

            "text_changed" => {
                let current_line = args[0].as_str().unwrap_or("");
                let bytes_before_cursor = args[1].as_u64().unwrap_or(0);
                compleet::text_changed(
                    &nvim,
                    completion_items,
                    ui_state,
                    current_line,
                    bytes_before_cursor,
                )
                .await
            },

            _ => {},
        }
    }

    async fn handle_request(
        &self,
        method: String,
        args: Vec<Value>,
        _nvim: compleet::Nvim,
    ) -> Result<Value, Value> {
        let ui_state = &mut *self.ui_state.lock().await;
        match method.as_str() {
            "has_completions" => {
                let completion_items =
                    &mut *self.completion_items.lock().await;
                let current_line = args[0].as_str().unwrap_or("");
                let bytes_before_cursor = args[1].as_u64().unwrap_or(0);
                Ok(Value::from(compleet::has_completions(
                    completion_items,
                    current_line,
                    bytes_before_cursor,
                )))
            },

            "is_completion_item_selected" => Ok(Value::from(
                ui_state.completion_menu.selected_index.is_some(),
            )),

            "is_completion_menu_visible" => {
                Ok(Value::from(ui_state.completion_menu.is_visible()))
            },

            "ping" => match args[0].as_str().unwrap_or("") {
                "Neovim says ping!" => Ok(Value::from("Rust says pong!")),
                _ => Err(Value::Nil),
            },

            _ => Err(Value::Nil),
        }
    }
}

#[tokio::main]
async fn main() {
    let handler = NeovimHandler::new();
    let (_nvim, io_handler) =
        nvim_rs::create::tokio::new_parent(handler).await;

    match io_handler.await {
        Ok(_) => {},
        Err(_) => {},
    }
}
