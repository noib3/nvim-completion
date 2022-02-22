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

        // TODO
        // self.selected_index = None;
    }

    pub fn is_visible(&self) -> bool {
        self.window.is_some()
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
            // This is only executed the first time the completion menu is
            // shown. After that we'll already have a buffer available.
            self.buffer = Some(Buffer::new(nvim).await);
        }

        let lines = completion_items
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = cmp::min(lines.len(), 7);

        if let Some(buffer) = &self.buffer {
            let (_, window) = future::join(
                buffer.set_lines(&lines),
                FloatingWindow::new(nvim, &buffer, width, height),
            )
            .await;

            self.window = Some(window);
        }
    }

    pub async fn update_selected_completion(
        &mut self,
        new_selected_index: Option<usize>,
    ) {
        // TODO: if both the old and new indexes are `Some` we don't have to
        // wait for the old one to be cleared before adding the new one => run
        // them at the same time asyncy.
        if let Some(buffer) = &self.buffer {
            match self.selected_index {
                Some(old) => buffer.clear_highlight(old as i64).await,
                None => {},
            }

            match new_selected_index {
                Some(new) => buffer.add_highlight(new as i64).await,
                None => {},
            }
        }

        self.selected_index = new_selected_index;
    }
}

impl fmt::Display for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} ({}) ", self.text, self.matched_characters.len())
    }
}
