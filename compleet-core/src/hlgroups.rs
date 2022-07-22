use nvim_oxi::{self as nvim, api, opts::SetHighlightOpts};

pub(crate) fn setup() -> nvim::Result<()> {
    let mut opts = SetHighlightOpts::builder();
    opts.default(true);

    api::set_hl(0, BAD_OPTION_PATH, Some(&opts.link("Statement").build()))?;
    api::set_hl(0, ERROR_MSG_TAG, Some(&opts.link("ErrorMsg").build()))?;
    api::set_hl(0, INFO_MSG_TAG, Some(&opts.link("Question").build()))?;
    api::set_hl(0, MSG_DQUOTED, Some(&opts.link("Special").build()))?;
    api::set_hl(0, WARNING_MSG_TAG, Some(&opts.link("WarningMsg").build()))?;

    api::set_hl(0, DETAILS, Some(&opts.link("NormalFloat").build()))?;
    api::set_hl(0, DETAILS_BORDER, Some(&opts.link("FloatBorder").build()))?;
    api::set_hl(0, HINT, Some(&opts.link("Comment").build()))?;
    api::set_hl(0, MENU, Some(&opts.link("NormalFloat").build()))?;
    api::set_hl(0, MENU_BORDER, Some(&opts.link("FloatBorder").build()))?;
    api::set_hl(0, MENU_MATCHING, Some(&opts.bold(true).build()))?;
    api::set_hl(0, MENU_SELECTED, Some(&opts.link("PmenuSel").build()))?;

    Ok(())
}

pub(crate) use consts::*;

mod consts {
    pub use messages::*;
    pub use ui::*;

    mod messages {
        /// Highlights the path of the config option that caused a
        /// deserialization error.
        pub const BAD_OPTION_PATH: &str = "CompleetBadOptionPath";

        /// Highlights the prefix tag of error messages.
        pub const ERROR_MSG_TAG: &str = "CompleetErrorMsgTag";

        /// Highlights the prefix tag of info messages.
        pub const INFO_MSG_TAG: &str = "CompleetInfoMsgTag";

        /// Highlights double quoted strings in the error message.
        pub const MSG_DQUOTED: &str = "CompleetMsgField";

        /// Highlights the prefix tag of warning messages.
        pub const WARNING_MSG_TAG: &str = "CompleetWarningMsgTag";
    }

    mod ui {
        /// Highlights the details window.
        pub const DETAILS: &str = "CompleetDetails";

        /// Highlights the border of the details window.
        pub const DETAILS_BORDER: &str = "CompleetDetailsBorder";

        /// Highlights the completion hint.
        pub const HINT: &str = "CompleetHint";

        /// Highlights the completion menu.
        pub const MENU: &str = "CompleetMenu";

        /// Highlights the border of the completion menu.
        pub const MENU_BORDER: &str = "CompleetMenuBorder";

        /// Highlights the characters where a completion item matches the
        /// current completion prefix.
        pub const MENU_MATCHING: &str = "CompleetMenuMatchingChars";

        /// Highlights the currently selected completion item.
        pub const MENU_SELECTED: &str = "CompleetMenuSelected";
    }
}
