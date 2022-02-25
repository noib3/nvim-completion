use mlua::{Lua, Result};
use std::{cmp, fmt};

use super::{Buffer, FloatingWindow};

use crate::completion::CompletionItem;
use crate::Nvim;

pub struct CompletionMenu {
    /// TODO: docs
    buffer: Buffer,

    /// TODO: docs
    window: Option<FloatingWindow>,

    /// TODO: docs
    pub selected_index: Option<usize>,
}

impl CompletionMenu {
    /// TODO: docs
    pub fn new(nvim: &Nvim) -> Result<Self> {
        Ok(CompletionMenu {
            buffer: Buffer::new(nvim, false, true)?,
            window: None,
            selected_index: None,
        })
    }
}

impl CompletionMenu {
    pub fn hide(&mut self, nvim: &Nvim) -> Result<()> {
        if let Some(window) = &self.window {
            window.hide(nvim)?;
            self.window = None;
        }
        Ok(())
    }

    pub fn is_visible(&self) -> bool {
        self.window.is_some()
    }

    pub fn select_completion(
        &mut self,
        nvim: &Nvim,
        new_selected_index: Option<usize>,
    ) -> Result<()> {
        match self.selected_index {
            Some(old) => self.buffer.clear_namespace(nvim, old)?,
            None => {},
        };

        match new_selected_index {
            Some(new) => self.buffer.add_highlight(nvim, new)?,
            None => {},
        };

        self.selected_index = new_selected_index;

        Ok(())
    }

    pub fn show_completions(
        &mut self,
        nvim: &Nvim,
        lua: &Lua,
        completion_items: &[CompletionItem],
    ) -> Result<()> {
        let lines = completion_items
            .into_iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>();

        self.buffer.set_lines(nvim, &lines)?;

        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let height = cmp::min(lines.len(), 7);

        self.window = Some(FloatingWindow::new(
            nvim,
            lua,
            self.buffer.bufnr,
            width,
            height,
        )?);

        Ok(())
    }
}

impl fmt::Display for CompletionItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " {} ({}) ", self.text, self.matched_characters.len())
    }
}
