/// The filename of the compleet server binary.
pub const SERVER_BINARY_NAME: &'static str = "compleet";

/// The name of the augroup used to namespace all the autocommands.
pub const AUGROUP_NAME: &'static str = "Compleet";

/// The tag used as a prefix in all the messages.
pub const MSG_TAG: &'static str = "[nvim-compleet]";

pub mod hlgroups {
    pub mod messages {
        /// Highlights the prefix tag of error messages.
        pub const ERROR_MSG_TAG: &'static str = "CompleetErrorMsgTag";

        /// Highlights the prefix tag of warning messages.
        pub const WARNING_MSG_TAG: &'static str = "CompleetWarningMsgTag";

        /// Highlights the prefix tag of info messages.
        pub const INFO_MSG_TAG: &'static str = "CompleetInfoMsgTag";

        /// Highlights the path of the config option that caused a
        /// deserialization error.
        pub const OPTION_PATH: &'static str = "CompleetOptionPath";

        /// Highlights double quoted strings in the error message.
        pub const MSG_FIELD: &'static str = "CompleetMsgField";
    }

    pub mod ui {
        /// Highlights the completion menu.
        pub const MENU: &'static str = "CompleetMenu";

        /// Highlights the currently selected completion item.
        pub const MENU_SELECTED: &'static str = "CompleetMenuSelected";

        /// Highlights the characters where a completion item matches the
        /// current completion prefix.
        pub const MENU_MATCHING: &'static str = "CompleetMenuMatchingChars";

        /// Highlights the border of the completion menu.
        pub const MENU_BORDER: &'static str = "CompleetMenuBorder";

        /// Highlights the details window.
        pub const DETAILS: &'static str = "CompleetDetails";

        /// Highlights the border of the details menu.
        pub const DETAILS_BORDER: &'static str = "CompleetDetailsBorder";

        /// Highlights the completion hint.
        pub const HINT: &'static str = "CompleetHint";
    }
}
