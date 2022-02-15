use rmpv::Value;

// `:h nvim_open_win`
pub struct FloatingWindowConfig {
    relative: FloatingWindowRelativePosition,
    win: Option<usize>,
    anchor: Option<FloatingWindowAnchor>,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
}

// Floating window is placed at (row, col) coordinates relative to the:
enum FloatingWindowRelativePosition {
    Editor, // Global editor grid.
    Window, // Window given by the `win` field, or current window.
    Cursor, // Cursor position in current window.
}

enum FloatingWindowAnchor {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl FloatingWindowConfig {
    pub fn new() -> FloatingWindowConfig {
        unimplemented!()
    }
}

impl From<FloatingWindowConfig> for Vec<(Value, Value)> {
    fn from(config: FloatingWindowConfig) -> Vec<(Value, Value)> {
        vec![
            (Value::from("relative"), Value::from(config.relative)),
            (Value::from("row"), Value::from(config.row)),
            (Value::from("col"), Value::from(config.col)),
            (Value::from("height"), Value::from(config.height)),
            (Value::from("width"), Value::from(config.width)),
        ]
    }
}

impl From<FloatingWindowRelativePosition> for Value {
    fn from(position: FloatingWindowRelativePosition) -> Value {
        Value::from(match position {
            FloatingWindowRelativePosition::Editor => "editor",
            FloatingWindowRelativePosition::Window => "win",
            FloatingWindowRelativePosition::Cursor => "cursor",
        })
    }
}

impl From<FloatingWindowAnchor> for Value {
    fn from(anchor: FloatingWindowAnchor) -> Value {
        Value::from(match anchor {
            FloatingWindowAnchor::NorthWest => "NW",
            FloatingWindowAnchor::NorthEast => "NE",
            FloatingWindowAnchor::SouthWest => "SW",
            FloatingWindowAnchor::SouthEast => "SE",
        })
    }
}
