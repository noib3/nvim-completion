use nvim_oxi::types::WindowBorder;
use serde::{de, Deserialize};

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct UiConfig {
    #[serde(default)]
    pub(super) details: DetailsConfig,

    #[serde(default)]
    pub(super) hint: HintConfig,

    #[serde(default)]
    pub(super) menu: MenuConfig,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DetailsConfig {
    #[serde(default = "default_details_border")]
    border: Border,
}

impl Default for DetailsConfig {
    #[inline]
    fn default() -> Self {
        Self { border: default_details_border() }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct HintConfig {
    #[serde(default)]
    pub(super) enable: bool,
}

#[derive(Debug, Deserialize)]
pub(super) struct MenuConfig {
    /// Where to anchor the completion menu.
    #[serde(default)]
    anchor: MenuAnchor,

    /// Whether to automatically display the completion menu when completion
    /// results are available. If `false` the menu won't be shown until asked
    /// explicitely via .. TODO.
    #[serde(default = "yes")]
    autoshow: bool,

    #[serde(default = "default_menu_border")]
    border: Border,

    #[serde(default = "seven", deserialize_with = "deser_max_height")]
    max_height: u32,
}

impl Default for MenuConfig {
    #[inline]
    fn default() -> Self {
        Self {
            anchor: MenuAnchor::default(),
            autoshow: yes(),
            border: default_menu_border(),
            max_height: seven(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub(super) struct Border {
    enable: bool,
    style: WindowBorder,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum MenuAnchor {
    /// TODO: docs.
    #[default]
    Cursor,

    /// TODO: docs.
    Match,
}

fn default_details_border() -> Border {
    Border {
        enable: true,
        style: WindowBorder::from((
            None,
            None,
            None,
            (' ', "CompletionDetails"),
        )),
    }
}

fn default_menu_border() -> Border {
    Border {
        enable: true,
        style: WindowBorder::from((None, None, None, (' ', "CompletionMenu"))),
    }
}

fn seven() -> u32 {
    7
}

fn yes() -> bool {
    true
}

fn deser_max_height<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: de::Deserializer<'de>,
{
    match <u32>::deserialize(deserializer)? {
        0 => Err(de::Error::invalid_value(
            de::Unexpected::Unsigned(0),
            &"a positive number",
        )),

        height => Ok(height),
    }
}
