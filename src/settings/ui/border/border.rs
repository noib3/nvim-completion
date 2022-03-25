use serde::Deserialize;

use super::BorderStyle;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Border {
    /// Whether to enable the border.
    pub enable: bool,

    /// The style of the border. Can be any of the values listed in `:h
    /// nvim_open_win`.
    pub style: BorderStyle,
}

/// Helper struct used to deserialize the borders in the completion menu and in
/// the details window with different defaults.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IncompleteBorder {
    pub enable: Option<bool>,
    pub style: Option<BorderStyle>,
}

impl Border {
    pub fn has_top_edge(&self) -> bool {
        self.enable && self.style.has_top_edge()
    }

    pub fn has_bottom_edge(&self) -> bool {
        self.enable && self.style.has_bottom_edge()
    }

    pub fn has_left_edge(&self) -> bool {
        self.enable && self.style.has_left_edge()
    }

    pub fn has_right_edge(&self) -> bool {
        self.enable && self.style.has_right_edge()
    }
}
