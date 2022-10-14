use std::cmp;
use std::ops::{Bound, Range, RangeBounds, RangeInclusive};
use std::slice::SliceIndex;

use completion_types::{CompletionItem, ScoredCompletion};
use nvim::api::{
    self,
    opts::SetExtmarkOpts,
    types::{WindowBorder, WindowConfig, WindowRelativeTo},
    Buffer,
    Window,
};
use nvim_oxi as nvim;
use serde::{de, Deserialize};

use super::config::Border;
use super::MenuGeometry;
use crate::hlgroups;
use crate::CompletionItemExt;

const MENU_NAMESPACE: &str = "completion_menu";

#[derive(Debug)]
pub(crate) struct CompletionMenu {
    /// The Neovim buffer used to display the rendered completion items.
    buf: Buffer,

    /// The current completion items.
    completions: Vec<ScoredCompletion>,

    /// Config set by the user.
    config: MenuConfig,

    /// The height of the completion menu in terminal rows.
    height: u16,

    /// The currently rendered range of completions.
    ///
    /// When new completion items arrive they have to be "rendered" to be
    /// displayed in this menu.
    ///
    /// By "rendering" I mean going from a [`CompletionItem`] to its single
    /// line string representation, highlighting the resulting string and
    /// highlighting the matched characters.
    ///
    /// If there are lots of completions rendering all of them at once right
    /// when they arrive can be quite expensive, and also unnecessary since the
    /// total number of completions can be much larger than the height of this
    /// menu.
    ///
    /// We instead only render the first `n` completions, where `n` is some
    /// small multiple of the menu height, and render the rest lazily when the
    /// user scrolls down.
    ///
    /// This range contains the indices of the completion items that have been
    /// rendered.
    ///
    /// # Invariants:
    ///
    /// - the end of the range is always `<= completions.len() - 1`;
    ///
    /// - the index in `selected_completion`, if set, is always contained in
    ///   this range.
    rendered_range: RangeInclusive<usize>,

    /// The id of the Neovim namespace used to ...
    namespace_id: u32,

    /// The index of the currently selected completion item, if any.
    ///
    /// # Invariants
    ///
    /// - it's always a valid index into `completions`, i.e. always between `0`
    ///   and `completions.len() - 1`.
    selected_completion: Option<usize>,

    /// The width of the completion menu in terminal cells.
    width: u16,

    /// The Neovim floating window used to hold the buffer, or `None` if the
    /// completion menu is currently closed.
    win: Option<Window>,
}

impl Default for CompletionMenu {
    #[inline]
    fn default() -> Self {
        Self {
            buf: 0.into(),
            completions: Vec::new(),
            config: MenuConfig::default(),
            height: 0,
            namespace_id: api::create_namespace(MENU_NAMESPACE),
            rendered_range: RangeInclusive::new(0, 0),
            selected_completion: None,
            width: 0,
            win: None,
        }
    }
}

impl CompletionMenu {
    #[inline]
    pub(super) fn init(&mut self, config: MenuConfig) -> nvim::Result<()> {
        self.config = config;
        self.buf = api::create_buf(false, true)?;

        Ok(())
    }

    #[inline]
    fn is_open(&self) -> bool {
        self.win.is_some()
    }

    pub(crate) fn set_completions(
        &mut self,
        completions: Vec<ScoredCompletion>,
        drawable_rows: u16,
        drawable_columns: u16,
    ) -> nvim::Result<()> {
        let desired_height =
            cmp::min(self.config.max_height as usize, completions.len());

        let desired_width = {
            let compute_width =
                width_compute_strategy(completions.iter().map(|c| &*c.item));

            completions
                .iter()
                .map(|c| c.item.menu_display())
                .map(|s| compute_width(&s))
                .max()
                .unwrap()
        };

        self.completions = completions;
        self.render(0..desired_height)?;

        let positioning = MenuGeometry::new(
            desired_height as u16,
            desired_width as u16,
            drawable_rows,
            drawable_columns,
        );

        if self.is_open() {
            self.move_window(positioning)?;
        } else {
            self.open_window(positioning)?;
        }

        Ok(())
    }

    fn render(&mut self, range: Range<usize>) -> nvim::Result<()> {
        let start = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Excluded(&n) => n + 1,
            Bound::Included(&n) => n,
        };

        let end = match range.end_bound() {
            Bound::Unbounded => self.completions.len().saturating_sub(1),
            Bound::Excluded(&n) => n.saturating_sub(1),
            Bound::Included(&n) => n,
        };

        assert!(start <= end);
        assert!(end < self.completions.len());

        let lines = self.completions[range.clone()]
            .iter()
            .map(|c| c.item.menu_display());

        self.buf.set_lines(start, end, false, lines)?;

        for (row, completion) in self.completions[range].iter().enumerate() {
            for (byte_range, hl_group) in completion.item.highlight_ranges() {
                let opts = SetExtmarkOpts::builder()
                    .end_row(row)
                    .end_col(*byte_range.end())
                    .hl_group(hl_group)
                    .priority(100)
                    .build();

                self.buf.set_extmark(
                    self.namespace_id,
                    row,
                    *byte_range.start(),
                    &opts,
                )?;
            }

            for byte_range in completion.matched_ranges() {
                let offset = completion.item.text_offset();

                let opts = SetExtmarkOpts::builder()
                    .end_row(row)
                    .end_col(offset + *byte_range.end())
                    .hl_group(hlgroups::MENU_MATCHING)
                    .priority(200)
                    .build();

                self.buf.set_extmark(
                    self.namespace_id,
                    row,
                    offset + *byte_range.start(),
                    &opts,
                )?;
            }
        }

        Ok(())
    }

    /// Opens the completion menu's floating window used to display the
    /// completion results.
    ///
    /// Should only be called if the menu is currently closed. If the menu is
    /// already open consider using [`move_window`] instead.
    fn open_window(&mut self, geometry: MenuGeometry) -> nvim::Result<()> {
        debug_assert!(!self.is_open());

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .height(geometry.height as _)
            .width(geometry.width as _)
            .row(geometry.row)
            .col(geometry.col)
            .noautocmd(true)
            .zindex(200)
            .build();

        self.win = Some(api::open_win(&self.buf, false, &config)?);

        Ok(())
    }

    /// Moves the completion menu's floating window to a new position.
    ///
    /// Should only be called if the menu is already open. If the menu is
    /// currently closed consider using [`open_window`] instead.
    fn move_window(&mut self, geometry: MenuGeometry) -> nvim::Result<()> {
        debug_assert!(self.is_open());

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Cursor)
            .height(geometry.height as _)
            .width(geometry.width as _)
            .row(geometry.row)
            .col(geometry.col)
            .build();

        self.win.as_mut().unwrap().set_config(&config)?;

        Ok(())
    }

    pub(crate) fn selected_completion(&self) -> Option<&CompletionItem> {
        self.selected_completion.map(|idx| &*self.completions[idx].item)
    }

    pub(crate) fn nth_completion(
        &self,
        idx: usize,
    ) -> Option<&CompletionItem> {
        self.completions.get(idx).map(|c| &*c.item)
    }

    pub(crate) fn select_next(&mut self) {
        todo!()
    }

    pub(crate) fn select_prev(&mut self) {
        todo!()
    }

    pub(crate) fn select_completion(&mut self, idx: usize) {
        todo!()
    }

    pub(crate) fn close(&mut self) -> nvim::Result<()> {
        if let Some(win) = self.win.take() {
            win.hide()?;
            self.completions.clear();
            self.selected_completion = None;
        }

        Ok(())
    }
}

/// TODO: docs
fn width_compute_strategy<'item, I>(
    completions: I,
) -> impl for<'a> Fn(&'a str) -> usize
where
    I: ExactSizeIterator<Item = &'item CompletionItem>,
{
    match completions.len() {
        n if n <= 50 => nvim_strwidth,
        n if n <= 1000 => grapheme_width,
        n if n <= 5000 => code_point_width,
        _ => bytes_width,
    }
}

fn nvim_strwidth(line: &str) -> usize {
    api::call_function::<_, usize>("strwidth", (line,)).unwrap()
}

fn grapheme_width(line: &str) -> usize {
    use unicode_segmentation::UnicodeSegmentation;
    line.graphemes(true).count()
}

fn code_point_width(line: &str) -> usize {
    line.chars().count()
}

fn bytes_width(line: &str) -> usize {
    line.bytes().count()
}

#[derive(Debug, Deserialize)]
pub(crate) struct MenuConfig {
    /// Whether to automatically display the completion menu when completion
    /// results are available. If `false` the menu won't be shown until asked
    /// explicitely via .. TODO.
    #[serde(default = "yes")]
    autoshow: bool,

    #[serde(default = "default_menu_border")]
    border: Border,

    #[serde(default = "seven", deserialize_with = "deser_max_height")]
    pub max_height: u16,
}

impl Default for MenuConfig {
    #[inline]
    fn default() -> Self {
        Self {
            autoshow: yes(),
            border: default_menu_border(),
            max_height: seven(),
        }
    }
}

fn default_menu_border() -> Border {
    Border {
        enable: true,
        style: WindowBorder::from((None, None, None, (' ', "CompletionMenu"))),
    }
}

fn yes() -> bool {
    true
}

fn seven() -> u16 {
    7
}

fn deser_max_height<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: de::Deserializer<'de>,
{
    match <u16>::deserialize(deserializer)? {
        0 => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(0),
            &"a positive number",
        )),

        height => Ok(height),
    }
}
