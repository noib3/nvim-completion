use std::fmt::{self, Display, Formatter};

/// A vector of bytes couldn't be decoded into a valid msgpack-rpc message.
pub enum DecodingError {
    /// A custom error message.
    Custom(&'static str),

    /// `rmpv::decode::read_value` failed.
    Rmpv(rmpv::decode::Error),
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let message = match self {
            Self::Custom(str) => str.to_string(),
            Self::Rmpv(err) => err.to_string(),
        };

        f.write_str(&message)
    }
}

impl From<&'static str> for DecodingError {
    fn from(str: &'static str) -> DecodingError { DecodingError::Custom(str) }
}

impl From<rmpv::decode::Error> for DecodingError {
    fn from(err: rmpv::decode::Error) -> DecodingError {
        DecodingError::Rmpv(err)
    }
}
