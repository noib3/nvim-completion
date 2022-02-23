use mlua::{Lua, Result, Table};
use std::sync::{Arc, Mutex};

mod ui;
use ui::UIState;

type Nvim<'lua> = Table<'lua>;

struct State {
    ui: Arc<Mutex<UIState>>,
}

impl State {
    fn new() -> Self {
        State {
            ui: Arc::new(Mutex::new(UIState::new())),
        }
    }
}

fn plus_one(ui_state: &mut UIState) {
    ui_state.foo += 1;
}

fn minus_one(ui_state: &mut UIState) {
    ui_state.foo -= 1;
}

#[mlua::lua_module]
fn compleet(lua: &Lua) -> Result<Table> {
    let _nvim = lua.globals().get::<&str, Nvim>("vim")?;
    let state = State::new();

    let ui_state_1 = Arc::clone(&state.ui);
    let plus_one = lua.create_function(move |_, ()| {
        Ok(plus_one(&mut ui_state_1.lock().unwrap()))
    })?;

    let ui_state_2 = Arc::clone(&state.ui);
    let minus_one = lua.create_function(move |_, ()| {
        Ok(minus_one(&mut ui_state_2.lock().unwrap()))
    })?;

    let ui_state_3 = Arc::clone(&state.ui);
    let print = lua.create_function(move |_, ()| {
        Ok(println!("{:?}", &ui_state_3.lock().unwrap().foo))
    })?;

    let exports = lua.create_table()?;
    exports.set("plus_one", plus_one)?;
    exports.set("minus_one", minus_one)?;
    exports.set("print", print)?;
    Ok(exports)
}
