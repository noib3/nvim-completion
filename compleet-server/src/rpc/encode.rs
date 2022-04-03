use rmpv::{encode, Value};

use super::message::*;

impl From<RpcMessage> for Vec<u8> {
    fn from(message: RpcMessage) -> Vec<u8> {
        let value = match message {
            RpcMessage::Request(RpcRequest { id, method, params }) => {
                Value::from(vec![
                    Value::from(id),
                    Value::from(method),
                    Value::from(params),
                ])
            },

            RpcMessage::Response(RpcResponse { id, error, result }) => {
                Value::from(vec![
                    Value::from(id),
                    Value::from(error),
                    Value::from(result),
                ])
            },

            RpcMessage::Notification(RpcNotification { method, params }) => {
                Value::from(vec![Value::from(method), Value::from(params)])
            },
        };

        let mut bytes = Vec::<u8>::new();
        encode::write_value(&mut bytes, &value).expect("When does this fail?");

        bytes
    }
}
