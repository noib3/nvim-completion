use mlua::prelude::LuaValue;

#[derive(Debug)]
pub enum Notification {
    StopTasks,
}

#[derive(Debug)]
pub enum Request {
    StopTasks,
}

impl<'a> Into<(String, Option<Vec<LuaValue<'a>>>)> for Notification {
    fn into(self) -> (String, Option<Vec<LuaValue<'a>>>) {
        match self {
            Notification::StopTasks => ("stop\n".into(), None),
        }
    }
}

impl<'a> Into<(String, Option<Vec<LuaValue<'a>>>)> for Request {
    fn into(self) -> (String, Option<Vec<LuaValue<'a>>>) {
        match self {
            Request::StopTasks => ("stop".into(), None),
        }
    }
}
