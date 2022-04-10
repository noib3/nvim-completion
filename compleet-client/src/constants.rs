/// The name of the augroup used to namespace all the autocommands.
pub const AUGROUP_NAME: &str = "Compleet";

/// The tag used as a prefix in all the messages.
pub const MSG_TAG: &str = "[nvim-compleet]";

pub mod hlgroups {
    pub mod messages {
        /// Highlights the prefix tag of error messages.
        pub const ERROR_MSG_TAG: &str = "CompleetErrorMsgTag";

        /// Highlights the prefix tag of warning messages.
        pub const WARNING_MSG_TAG: &str = "CompleetWarningMsgTag";

        /// Highlights the prefix tag of info messages.
        pub const INFO_MSG_TAG: &str = "CompleetInfoMsgTag";

        /// Highlights the path of the config option that caused a
        /// deserialization error.
        pub const OPTION_PATH: &str = "CompleetOptionPath";

        /// Highlights double quoted strings in the error message.
        pub const MSG_FIELD: &str = "CompleetMsgField";
    }

    pub mod ui {
        /// Highlights the completion menu.
        pub const MENU: &str = "CompleetMenu";

        /// Highlights the currently selected completion item.
        pub const MENU_SELECTED: &str = "CompleetMenuSelected";

        /// Highlights the characters where a completion item matches the
        /// current completion prefix.
        pub const MENU_MATCHING: &str = "CompleetMenuMatchingChars";

        /// Highlights the border of the completion menu.
        pub const MENU_BORDER: &str = "CompleetMenuBorder";

        /// Highlights the details window.
        pub const DETAILS: &str = "CompleetDetails";

        /// Highlights the border of the details menu.
        pub const DETAILS_BORDER: &str = "CompleetDetailsBorder";

        /// Highlights the completion hint.
        pub const HINT: &str = "CompleetHint";
    }
}
