use rmpv::{encode, Value};

use super::message::*;

impl From<RpcMessage> for Vec<u8> {
    fn from(message: RpcMessage) -> Vec<u8> {
        match message {
            RpcMessage::Request(req) => req.into(),
            RpcMessage::Response(rsp) => rsp.into(),
            RpcMessage::Notification(ntf) => ntf.into(),
        }
    }
}

impl From<RpcRequest> for Vec<u8> {
    fn from(request: RpcRequest) -> Vec<u8> {
        let value = Value::from(vec![
            Value::from(0),
            Value::from(request.id),
            Value::from(request.method),
            Value::from(request.params),
        ]);

        let mut bytes = Vec::<u8>::new();
        encode::write_value(&mut bytes, &value).expect("When does this fail?");

        bytes
    }
}

impl From<RpcResponse> for Vec<u8> {
    fn from(response: RpcResponse) -> Vec<u8> {
        let value = Value::from(vec![
            Value::from(1),
            Value::from(response.id),
            Value::from(response.error),
            Value::from(response.result),
        ]);

        let mut bytes = Vec::<u8>::new();
        encode::write_value(&mut bytes, &value).expect("When does this fail?");

        bytes
    }
}

impl From<RpcNotification> for Vec<u8> {
    fn from(notification: RpcNotification) -> Vec<u8> {
        let value = Value::from(vec![
            Value::from(2),
            Value::from(notification.method),
            Value::from(notification.params),
        ]);

        let mut bytes = Vec::<u8>::new();
        encode::write_value(&mut bytes, &value).expect("When does this fail?");

        bytes
    }
}
