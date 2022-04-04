use mlua::prelude::LuaValue;

#[derive(Debug)]
pub enum Notification {
    StopTasks,
    Completions,
}

#[derive(Debug)]
pub enum Request {
    StopTasks,
}

impl<'a> From<Notification> for (String, Option<Vec<LuaValue<'a>>>) {
    fn from(ntf: Notification) -> Self {
        match ntf {
            Notification::StopTasks => ("stop".into(), None),
            Notification::Completions => ("completions".into(), None),
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
