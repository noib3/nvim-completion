use rmpv::Value;
use serde::{Deserialize, Serialize};

use crate::cursor::Cursor;
use crate::rpc::RpcMessage;

/// A valid incoming notification, all other RPC notifications are not
/// recognized.
#[derive(Debug, Serialize, Deserialize)]
pub enum Notification {
    /// Stop all running tasks.
    StopTasks,

    /// Start computing completions.
    SendCompletions(u32, Cursor),
}

impl From<Notification> for RpcMessage {
    fn from(notification: Notification) -> RpcMessage {
        let (method, params) = match notification {
            Notification::StopTasks => ("stop".into(), vec![]),

            Notification::SendCompletions(bufnr, cursor) => (
                "completions".into(),
                vec![
                    Value::from(bufnr),
                    Value::from(vec![
                        Value::from(cursor.bytes),
                        Value::from(cursor.line),
                        Value::from(cursor.row),
                    ]),
                ],
            ),
        };

        RpcMessage::Notification { method, params }
    }
}

/// Try to decode a `Notification` from a `(method, params)` tuple.
impl TryFrom<(String, Vec<Value>)> for Notification {
    type Error = &'static str;

    fn try_from(
        (method, params): (String, Vec<Value>),
    ) -> Result<Notification, Self::Error> {
        match (method.as_ref(), params) {
            ("stop", vec) if vec.is_empty() => Ok(Self::StopTasks),

            ("completions", vec) if vec.len() == 2 => {
                let mut iter = vec.into_iter();

                let bufnr = match iter.next() {
                    Some(Value::Integer(n)) if n.is_u64() => {
                        n.as_u64().expect("already checked that it's a u64")
                            as u32
                    },

                    _ => return Err("bufnr isn't valid"),
                };

                let cursor = match iter.next() {
                    Some(Value::Array(vec)) => Cursor::try_from(vec)?,
                    _ => return Err("cursor should be an array"),
                };

                Ok(Self::SendCompletions(bufnr, cursor))
            },

            _ => Err("incoming notification doesn't match the API"),
        }
    }
}
