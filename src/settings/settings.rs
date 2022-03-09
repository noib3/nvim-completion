use mlua::{Table, Value};

#[derive(Debug)]
pub struct Settings {
    /// Whether to automatically show the completion menu every time there are
    /// completion items available.
    pub autoshow_menu: bool,

    /// Enable insert mode mappings for `<Tab>`, `<S-Tab>` and `<CR>`.
    pub enable_default_mappings: bool,

    /// The maximum number of rows the completion menu can take up.
    pub max_menu_height: Option<usize>,

    /// Whether to show completion hints.
    pub show_hints: bool,
}

impl Settings {
    // TODO: this is annoying to maintain.
    fn field_names() -> &'static [&'static str] {
        &[
            "autoshow_menu",
            "enable_default_mappings",
            "max_menu_height",
            "show_hints",
        ]
    }
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            autoshow_menu: true,
            enable_default_mappings: false,
            max_menu_height: None,
            show_hints: false,
        }
    }
}

pub enum Error {
    // TODO: add `found` field with the value of the value that was passed in.
    FailedConversion {
        option: &'static str,
        expected: &'static str,
    },

    InvalidValue {
        option: &'static str,
        reason: &'static str,
    },

    OptionDoesntExist {
        option: String,
    },

    Lua(mlua::Error),
}

impl From<mlua::Error> for Error {
    fn from(err: mlua::Error) -> Error {
        Error::Lua(err)
    }
}

impl<'a> TryFrom<Option<Table<'a>>> for Settings {
    type Error = Error;

    fn try_from(preferences: Option<Table>) -> Result<Self, Error> {
        let mut config = Settings::default();

        let preferences = match preferences {
            Some(table) => table,
            None => return Ok(config),
        };

        for pair in preferences.clone().pairs::<String, Value>() {
            let (key, _) = pair?;
            if !Settings::field_names().contains(&key.as_str()) {
                return Err(Error::OptionDoesntExist { option: key });
            }
        }

        // TODO: use a proc macro to derive some trait in `Config` to help
        // avoiding some of this boilerplate. Might need help for that.

        match preferences.get("autoshow_menu")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.autoshow_menu = bool,
            _ => {
                return Err(Error::FailedConversion {
                    option: "autoshow_menu",
                    expected: "boolean",
                })
            },
        }

        match preferences.get("enable_default_mappings")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.enable_default_mappings = bool,
            _ => {
                return Err(Error::FailedConversion {
                    option: "enable_default_mappings",
                    expected: "boolean",
                })
            },
        }

        match preferences.get("max_menu_height")? {
            Value::Nil => {},
            Value::Integer(height) => {
                if height < 1 {
                    return Err(Error::InvalidValue {
                        option: "max_menu_height",
                        reason: "the maximum menu height should be at least 1",
                    });
                }
                config.max_menu_height = Some(height as usize);
            },
            _ => {
                return Err(Error::FailedConversion {
                    option: "max_menu_height",
                    expected: "positive integer",
                })
            },
        }

        match preferences.get("show_hints")? {
            Value::Nil => {},
            Value::Boolean(bool) => config.show_hints = bool,
            _ => {
                return Err(Error::FailedConversion {
                    option: "show_hints",
                    expected: "boolean",
                })
            },
        }

        Ok(config)
    }
}
