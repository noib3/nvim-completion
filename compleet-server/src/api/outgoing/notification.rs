use rmp_serde::{Deserializer, Serializer};
use rmpv::Value;
use serde::{Deserialize, Serialize};

use crate::completion::{Completion, Completions};
use crate::rpc::RpcMessage;

const SERVE_COMPLETIONS_METHOD_NAME: &'static str = "serve_completions";

/// An outgoing notification.
#[derive(Debug, Serialize, Deserialize)]
pub enum Notification {
    ServeCompletions(Completions),
}

/// Convert `self::Notification` to an RPC notification.
impl From<Notification> for RpcMessage {
    fn from(ntf: Notification) -> RpcMessage {
        let (method, params) = match ntf {
            Notification::ServeCompletions(completions) => (
                SERVE_COMPLETIONS_METHOD_NAME.into(),
                completions
                    .into_iter()
                    .map(|c| Value::from(c))
                    .collect::<Vec<Value>>(),
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
            (SERVE_COMPLETIONS_METHOD_NAME, vec) => {
                let completions = vec
                    .into_iter()
                    .flat_map(|v| Completion::try_from(v))
                    .collect::<Completions>();

                Ok(Notification::ServeCompletions(completions))
            },

            _ => Err("outgoing notification doesn't match the API"),
        }
    }
}
