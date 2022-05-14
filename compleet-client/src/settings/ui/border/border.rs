use serde::{Deserialize, Deserializer};

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
struct IncompleteBorder {
    pub enable: Option<bool>,
    pub style: Option<BorderStyle>,
}

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
    default_enable: fn() -> bool,
    default_style: fn() -> BorderStyle,
) -> Result<Border, D::Error> {
    Deserialize::deserialize(deserializer).map(
        |IncompleteBorder { enable, style }| Border {
            enable: enable
                .unwrap_or_else(|| style.is_some() || default_enable()),
            style: style.unwrap_or_else(default_style),
        },
    )
}
