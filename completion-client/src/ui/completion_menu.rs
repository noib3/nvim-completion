use std::cmp;

use completion_types::CompletionItem;
use nvim::api::{self, Buffer, Window};
use nvim::opts::SetExtmarkOpts;
use nvim::types::{WindowBorder, WindowConfig, WindowRelativeTo};
use nvim_oxi as nvim;
use serde::{de, Deserialize};

use super::config::Border;
use super::MenuGeometry;
use crate::CompletionItemExt;

const MENU_NAMESPACE: &str = "completion_menu";

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

#[derive(Debug)]
pub(crate) struct CompletionMenu {
    config: MenuConfig,

    /// TODO: docs
    buf: Buffer,

    /// TODO: docs
    win: Option<Window>,

    /// TODO: docs
    height: u16,

    /// TODO: docs
    width: u16,

    /// TODO: docs
    namespace_id: u32,
}

impl Default for CompletionMenu {
    #[inline]
    fn default() -> Self {
        Self {
            config: MenuConfig::default(),
            buf: 0.into(),
            win: None,
            height: 0,
            width: 0,
            namespace_id: api::create_namespace(MENU_NAMESPACE),
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

    /// TODO: docs
    pub(super) fn display(
        &mut self,
        comps: &[CompletionItem],
        drawable_rows: u16,
        drawable_columns: u16,
    ) -> nvim::Result<()> {
        let desired_height =
            cmp::min(self.config.max_height, comps.len() as _);

        // Computing the "correct" width in terms of grapheme clusters is
        // around 8 times slower than using the number of code points.
        // Tested with 30k completions, using graphemes takes 57ms vs 7ms w/
        // code points.
        //
        // Code points is already a big improvement vs using raw bytes, so the
        // marginal increase in correctness is probably not worth 8x the
        // performance cost.
        let desired_width = comps
            .iter()
            // .map(|c| {
            //     use unicode_segmentation::UnicodeSegmentation;
            //     c.menu_display().graphemes(true).count()
            // })
            .map(|c| c.menu_display().chars().count())
            .max()
            .unwrap();

        let geometry = MenuGeometry::new(
            desired_height as _,
            desired_width as _,
            drawable_rows,
            drawable_columns,
        );

        self.set_contents(comps)?;

        if self.is_open() {
            self.move_window(geometry)?;
        } else {
            self.open_window(geometry)?;
        }

        Ok(())
    }

    /// Fills the menu's buffer with the new completions and highlights each
    /// completion according to its highlight ranges.
    fn set_contents(&mut self, comps: &[CompletionItem]) -> nvim::Result<()> {
        let lines = comps.iter().map(|cmp| cmp.menu_display());
        self.buf.set_lines(0, u32::MAX as _, false, lines)?;

        // Highlight each completion in the menu.
        for (row, comp) in comps.iter().enumerate() {
            for (byte_range, hl_group) in &comp.highlight_ranges() {
                let opts = SetExtmarkOpts::builder()
                    .end_row(row)
                    .end_col(*byte_range.end())
                    .hl_group(hl_group)
                    .priority(100)
                    .build();

                if let Err(err) = self.buf.set_extmark(
                    self.namespace_id,
                    row,
                    *byte_range.start(),
                    &opts,
                ) {
                    nvim::print!("ERR: {:?}, {:?}", comp, byte_range);
                    return Err(err);
                };
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

    /// Closes the completion menu's floating window.
    pub(super) fn close_window(&mut self) -> nvim::Result<()> {
        if let Some(win) = self.win.take() {
            win.hide()?;
        }

        Ok(())
    }
}
