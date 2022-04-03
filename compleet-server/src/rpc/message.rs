use rmpv::Value;
use serde::{Deserialize, Serialize};

/// A valid msgpack-rpc message.
/// See `https://github.com/msgpack-rpc/msgpack-rpc/blob/master/spec.md` for
/// details.
#[derive(Debug /* , Deserialize, Serialize */)]
pub enum RpcMessage {
    Request(RpcRequest),
    Response(RpcResponse),
    Notification(RpcNotification),
}

/// A msgpack-rpc request message.
#[derive(Debug /* , Deserialize, Serialize */)]
pub struct RpcRequest {
    pub id: u32,
    pub method: String,
    pub params: Vec<Value>,
}

/// A msgpack-rpc response message.
#[derive(Debug /* , Deserialize, Serialize */)]
pub struct RpcResponse {
    pub id: u32,
    pub error: Value,
    pub result: Value,
}

/// A msgpack-rpc notification message.
#[derive(Debug /* , Deserialize, Serialize */)]
pub struct RpcNotification {
    pub method: String,
    pub params: Vec<Value>,
}
