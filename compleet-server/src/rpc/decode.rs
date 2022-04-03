use std::convert::TryInto;
use std::iter::IntoIterator;

use rmpv::Value;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::message::{RpcMessage, RpcNotification, RpcRequest, RpcResponse};
use super::DecodingError;

// A first implementation of an RPC message decoder.
// Mostly taken from `KillTheMule/nvim-rs/src/rpc/model.rs`.

/// TODO: docs
pub async fn decode<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<RpcMessage, DecodingError> {
    let mut buffer = [0u8; 1024];

    let bytes_read = match reader.read(&mut buffer).await {
        Ok(n) => n,
        Err(_) => todo!(),
    };

    RpcMessage::try_from(buffer[..bytes_read].to_vec())
}

impl TryFrom<Vec<u8>> for RpcMessage {
    type Error = DecodingError;

    fn try_from(bytes: Vec<u8>) -> Result<RpcMessage, Self::Error> {
        let array: Vec<Value> =
            rmpv::decode::read_value(&mut bytes.as_slice())?
                .try_into()
                .map_err(|_| "hello there")?;

        let mut iter = array.into_iter();

        let r#type: u64 = iter
            .next()
            .ok_or("expected 3/4 elements, got 0")?
            .try_into()
            .map_err(|_| "expected either 0, 1 or 2 as the first element")?;

        match r#type {
            // A request message.
            0 => {
                let id: u64 = iter
                    .next()
                    .ok_or("requests should have 4 elements, only got 1")?
                    .try_into()
                    .map_err(|_| "request id should be a u32")?;

                let id = id as u32;

                let method = match iter.next() {
                    Some(Value::String(s)) if s.is_str() => s
                        .into_str()
                        .expect("already checked that it's valid utf8"),
                    Some(_) => {
                        return Err(DecodingError::Custom(
                            "expected a string as the request method",
                        ))
                    },
                    None => {
                        return Err(DecodingError::Custom(
                            "requests should have 4 elements only got 2",
                        ))
                    },
                };

                let params: Vec<Value> = iter
                    .next()
                    .ok_or("requests should have 4 elements, only got 3")?
                    .try_into()
                    .map_err(|_| {
                        "expected a vector of values as the request params"
                    })?;

                Ok(RpcMessage::Request(RpcRequest { id, method, params }))
            },

            1 => {
                let id: u64 = iter
                    .next()
                    .ok_or("requests should have 4 elements, only got 1")?
                    .try_into()
                    .map_err(|_| "request id should be a u32")?;

                let id = id as u32;

                let error = iter
                    .next()
                    .ok_or("responses should have 3 elements, only got 1")?;

                let result = iter
                    .next()
                    .ok_or("responses should have 3 elements, only got 2")?;

                Ok(RpcMessage::Response(RpcResponse { id, error, result }))
            },

            2 => {
                let method = match iter.next() {
                    Some(Value::String(s)) if s.is_str() => s
                        .into_str()
                        .expect("already checked that it's valid utf8"),
                    Some(_) => {
                        return Err(DecodingError::Custom(
                            "expected a string as the notification method",
                        ))
                    },
                    None => {
                        return Err(DecodingError::Custom(
                            "notifications should have 3 elements only got 1",
                        ))
                    },
                };

                let params: Vec<Value> = iter
                    .next()
                    .ok_or("notifications should have 3 elements, only got 2")?
                    .try_into()
                    .map_err(|_| {
                        "expected a vector of values as the notification \
                         params"
                    })?;

                Ok(RpcMessage::Notification(RpcNotification {
                    method,
                    params,
                }))
            },

            _ => Err(DecodingError::Custom("a message type can be 0, 1 or 2")),
        }
    }
}
