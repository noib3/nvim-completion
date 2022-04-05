use rmpv::{encode, Value};

use super::message::RpcMessage;

impl From<RpcMessage> for Vec<u8> {
    fn from(message: RpcMessage) -> Vec<u8> {
        let value = match message {
            RpcMessage::Request {
                msgid,
                method,
                params,
            } => Value::from(vec![
                Value::from(0),
                Value::from(msgid),
                Value::from(method),
                Value::from(params),
            ]),

            RpcMessage::Response {
                msgid,
                error,
                result,
            } => Value::from(vec![
                Value::from(1),
                Value::from(msgid),
                error,
                result,
            ]),

            RpcMessage::Notification { method, params } => Value::from(vec![
                Value::from(2),
                Value::from(method),
                Value::from(params),
            ]),
        };

        let mut bytes = Vec::<u8>::new();
        encode::write_value(&mut bytes, &value).expect("When does this fail?");

        bytes
    }
}
