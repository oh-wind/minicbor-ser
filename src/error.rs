use minicbor;
use minicbor::data::Type;
use core::fmt::{self, Debug, Display, Formatter};
use crate::lib::*;

use ser::StdError;

use serde::{de, ser};
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct Error{
    err: Box<ErrorKind>
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Write(Box<str>),
    Message(Box<str>),
    /// Decoding has (unexpectedly) reached the end of the input slice.
    EndOfInput,
    /// Data item to decode is not a valid `char`.
    InvalidChar(u32),
    /// Decoding a string failed because it is invalid UTF-8.
    Utf8(str::Utf8Error),
    /// A numeric value exceeds its value range.
    Overflow(u64, Box<str>),
    /// An unexpected type was encountered.
    TypeMismatch(Type, Box<str>),
    /// An unknown enum variant was encountered.
    UnknownVariant(u32),
    /// A value was missing at the specified index.
    MissingValue(u32, Box<str>),
    /// 128-bit integers are not supported at this time
    Unsupported128BitInteger,
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &*self.err {
            ErrorKind::Write(w) => write!(f, "write error: {}", w),
            ErrorKind::EndOfInput         => f.write_str("end of input bytes"),
            ErrorKind::InvalidChar(n)     => write!(f, "invalid char: {:#x?}", n),
            ErrorKind::Utf8(e)            => write!(f, "invalid utf-8: {}", e),
            ErrorKind::Overflow(n, m)     => write!(f, "{}: {} overflows target type", m, n),
            ErrorKind::TypeMismatch(t, m) => write!(f, "unexpected type: {}, {}", t, m),
            ErrorKind::UnknownVariant(n)  => write!(f, "unknown enum variant {}", n),
            ErrorKind::MissingValue(n, s) => write!(f, "missing value at index {} for {}", n, s),
            ErrorKind::Message(m)         => f.write_str(m),
            _                                   => f.write_str("unknow")

        }
    }
}

impl StdError for Error{
    #[cfg(feature="std")]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &*self.err {
            ErrorKind::Utf8(e) => Some(e),
            _                       => None,
        }
    }
}


impl ser::Error for Error{
    fn custom<T> (msg: T) -> Self where T: Display {
        make_msg_err(msg)
    }
}

impl de::Error for Error{
    fn custom<T>(msg:T)->Self where T:Display {
        make_msg_err(msg)
    }
}

fn make_msg_err<T: Display>(msg: T) -> Error{
    Error{
        err: Box::new(
            ErrorKind::Message(msg.to_string().into_boxed_str())
        )
    }
}

impl <T: Display> From<minicbor::encode::Error<T>> for Error{
    fn from(e: minicbor::encode::Error<T>) -> Self {
        match e {
            minicbor::encode::Error::Write(ref w) => make_msg_err(w),
            minicbor::encode::Error::Message(ref m) => make_msg_err(m),
            _ => panic!(),
        }
    }
}

impl From<minicbor::decode::Error> for Error{
    fn from(e: minicbor::decode::Error) -> Self {
        match e {
            minicbor::decode::Error::EndOfInput => make_kind_err(ErrorKind::EndOfInput),
            minicbor::decode::Error::InvalidChar(n) => make_kind_err(ErrorKind::InvalidChar(n)),
            minicbor::decode::Error::Utf8(e) => make_kind_err(ErrorKind::Utf8(e)),
            minicbor::decode::Error::Overflow(n, s) => make_kind_err(ErrorKind::Overflow(n, s.to_string().into_boxed_str())),
            minicbor::decode::Error::TypeMismatch(t, s) => make_kind_err(ErrorKind::TypeMismatch(t, s.to_string().into_boxed_str())),
            minicbor::decode::Error::UnknownVariant(v) => make_kind_err(ErrorKind::UnknownVariant(v)),
            minicbor::decode::Error::MissingValue(m, s) => make_kind_err(ErrorKind::MissingValue(m, s.to_string().into_boxed_str())),
            minicbor::decode::Error::Message(m) => make_kind_err(ErrorKind::Message(m.to_string().into_boxed_str())),
            _ => make_msg_err("unknow error"),
        }
    }
}
#[inline]
pub(crate) fn make_kind_err(e: ErrorKind) -> Error{
    Error { err: Box::new(e) }
}
#[inline]
pub(crate) fn type_mismatch(t: Type, s: &str) -> Error{
    make_kind_err(ErrorKind::TypeMismatch(t, s.to_string().into_boxed_str()))
}