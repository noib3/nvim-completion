use rmpv::Value;
use serde::ser::{Serialize, SerializeSeq, Serializer};
use serde::Deserialize;

/// A valid msgpack-rpc message.
/// See https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md for
/// details.
#[derive(Debug, Deserialize)]
pub enum RpcMessage {
    /// A msgpack-rpc request.
    Request {
        msgid: u32,
        method: String,
        params: Vec<Value>,
    },

    /// A msgpack-rpc response.
    Response {
        msgid: u32,
        error: Value,
        result: Value,
    },

    /// A msgpack-rpc notification.
    Notification { method: String, params: Vec<Value> },
}

impl Serialize for RpcMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use RpcMessage::*;

        match self {
            Request {
                msgid,
                method,
                params,
            } => {
                let mut array = serializer.serialize_seq(Some(4))?;
                array.serialize_element::<u8>(&0)?;
                array.serialize_element(msgid)?;
                array.serialize_element(method)?;
                array.serialize_element(params)?;
                array.end()
            },

            Response {
                msgid,
                error,
                result,
            } => {
                let mut array = serializer.serialize_seq(Some(4))?;
                array.serialize_element::<u8>(&1)?;
                array.serialize_element(msgid)?;
                array.serialize_element(error)?;
                array.serialize_element(result)?;
                array.end()
            },

            Notification { method, params } => {
                let mut array = serializer.serialize_seq(Some(3))?;
                array.serialize_element::<u8>(&2)?;
                array.serialize_element(method)?;
                array.serialize_element(params)?;
                array.end()
            },
        }
    }
}
