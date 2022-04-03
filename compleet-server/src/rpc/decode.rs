use std::convert::TryInto;
use std::io::{Cursor, ErrorKind, Read};
use std::iter::IntoIterator;

use rmpv::Value;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::message::{RpcMessage, RpcNotification, RpcRequest, RpcResponse};

pub enum Error {
    Msg(&'static str),
    Dec(rmpv::decode::Error),
    Io(std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

impl From<&'static str> for Error {
    fn from(str: &'static str) -> Error { Error::Msg(str) }
}

impl From<rmpv::decode::Error> for Error {
    fn from(err: rmpv::decode::Error) -> Error { Error::Dec(err) }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error { Error::Io(err) }
}

// A first implementation of an RPC message decoder.
// Mostly taken from `KillTheMule/nvim-rs/src/rpc/model.rs`.

/// TODO: docs
pub async fn decode<R: AsyncRead + Unpin>(
    reader: &mut R,
    queue: &mut Vec<u8>,
) -> Result<RpcMessage> {
    let mut buf = Box::new([0_u8; 80 * 1024]);
    let mut bytes_read;

    loop {
        let mut c = Cursor::new(&queue);

        match decode_sync(&mut c) {
            Ok(msg) => {
                let pos = c.position();
                // TODO: more efficiency
                *queue = queue.split_off(pos as usize);
                return Ok(msg);
            },
            Err(Error::Dec(e)) if e.kind() == ErrorKind::UnexpectedEof => {
                bytes_read = reader.read(&mut *buf).await;
            },
            Err(err) => return Err(err.into()),
        }

        match bytes_read {
            Ok(n) if n == 0 => {
                return Err(std::io::Error::new(
                    ErrorKind::UnexpectedEof,
                    "EOF",
                )
                .into());
            },
            Ok(n) => {
                queue.extend_from_slice(&buf[..n]);
            },
            Err(err) => return Err(err.into()),
        }
    }
}

/// TODO: docs
pub fn decode_sync<R: Read>(reader: &mut R) -> Result<RpcMessage> {
    let array: Vec<Value> = rmpv::decode::read_value(reader)?
        .try_into()
        .map_err(|_| "not an array!")?;

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
                Some(Value::String(s)) if s.is_str() => {
                    s.into_str().expect("already checked that it's valid utf8")
                },
                Some(_) => {
                    return Err(Error::Msg(
                        "expected a string as the request method",
                    ))
                },
                None => {
                    return Err(Error::Msg(
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
                Some(Value::String(s)) if s.is_str() => {
                    s.into_str().expect("already checked that it's valid utf8")
                },
                Some(_) => {
                    return Err(Error::Msg(
                        "expected a string as the notification method",
                    ))
                },
                None => {
                    return Err(Error::Msg(
                        "notifications should have 3 elements only got 1",
                    ))
                },
            };

            let params: Vec<Value> = iter
                .next()
                .ok_or("notifications should have 3 elements, only got 2")?
                .try_into()
                .map_err(|_| {
                    "expected a vector of values as the notification params"
                })?;

            Ok(RpcMessage::Notification(RpcNotification { method, params }))
        },

        _ => Err(Error::Msg("a message type can be 0, 1 or 2")),
    }
}
