use futures::future;
use std::{cmp, fmt};

use super::{Buffer, FloatingWindow};
use crate::{completion::CompletionItem, Nvim};

pub struct CompletionMenu {
    // TODO: docs
    buffer: Option<Buffer>,

    // TODO: docs
    window: Option<FloatingWindow>,

    // TODO: docs
    pub selected_index: Option<usize>,
}

impl CompletionMenu {
    pub async fn hide(&mut self) {
        if let Some(window) = &self.window {
            window.hide().await;
            self.window = None;
        }

        self.selected_index = None
    }

    pub fn is_visible(&self) -> bool {
        self.window.is_none()
    }

    pub fn new() -> Self {
        CompletionMenu {
            buffer: None,
            window: None,
            selected_index: None,
        }
    }

    pub async fn show_completions(
        &mut self,
        nvim: &Nvim,
        completion_items: &[CompletionItem],
    ) {
        if self.buffer.is_none() {
            self.buffer = Some(Buffer::new(&nvim).await);
        }

        let lines = completion_items
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let max_window_height = 7;

        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = cmp::min(lines.len(), max_window_height);

        if let Some(buffer) = &self.buffer {
            let (_, window) = future::join(
                buffer.set_lines(&lines),
                FloatingWindow::new(nvim, &buffer, width, height),
            )
            .await;

            self.window = Some(window);
        }
    }
}

impl fmt::Display for CompletionItem {
    // Look into `:h nvim_buf_add_highlight` for highlighting matching chars
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} ({}) ", self.text, self.matched_characters.len())
    }
}
