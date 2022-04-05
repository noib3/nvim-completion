/// The name of the augroup used to namespace all the autocmds.
pub const AUGROUP_NAME: &'static str = "Compleet";

/// The filename of the compleet server binary.
pub const SERVER_BINARY_NAME: &'static str = "compleet";

/// The prefix tag used in all the messages.
pub const MSG_TAG: &'static str = "[nvim-compleet]";

/// The highlight group used to highlight the `[nvim-compleet]` prefix tag of
/// error messages.
pub const HLGROUP_ERROR_MSG_TAG: &'static str = "CompleetErrorMsgTag";

/// The highlight group used to highlight the `[nvim-compleet]` prefix tag of
/// warning messages.
pub const HLGROUP_WARNING_MSG_TAG: &'static str = "CompleetWarningMsgTag";

/// The highlight group used to highlight the path of the config option that
/// caused a deserialization error.
pub const HLGROUP_OPTION_PATH: &'static str = "CompleetErrorMsgOptionPath";

/// The highlight group used to highlight the path of the config option that
/// caused a deserialization error.
pub const HLGROUP_MSG_FIELD: &'static str = "CompleetErrorMsgField";
