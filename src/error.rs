#![allow(unused)]

pub mod en {

    use super::make_msg;
    use crate::lib::*;
    
    use serde::ser;

    pub struct Error {
        #[cfg(feature = "alloc")]
        source: Option<Box<dyn Display>>,

        pub kind: ErrorKind,
        #[cfg(not(feature = "alloc"))]
        msg: &'static str,
        #[cfg(feature = "alloc")]
        msg: String,
    }

    impl Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Error {{ ")?;
            #[cfg(feature = "alloc")]
            {
                write!(f, "source: ")?;
                if let Some(s) = self.source.as_ref() {
                    s.as_ref().fmt(f)
                } else {
                    write!(f, "None")
                }
            }?;
            write!(f, " kind: {}, msg: {} }}", self.kind, self.msg)
        }
    }

    #[derive(Debug, Clone)]
    pub enum ErrorKind {
        Write,
        Message,
        Custom,
        Unknow,
        Unsupported128BitInteger,
    }
    impl Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ErrorKind::Write => write!(f, "Write"),
                ErrorKind::Message => write!(f, "Message"),
                ErrorKind::Custom => write!(f, "Custom"),
                ErrorKind::Unknow => write!(f, "Unknow"),
                ErrorKind::Unsupported128BitInteger => write!(f, "Unsupported128BitIntege"),
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> fmt::Result {
            #[cfg(feature = "alloc")]
            {
                if let Some(e) = self.source.as_ref() {
                    fmt::Display::fmt(e, f)
                } else {
                    write!(f, "error type: {}, error msg: {}", self.kind, self.msg)
                }
            }
            #[cfg(not(feature = "alloc"))]
            {
                write!(f, "error type: {}, error msg: {}", self.kind, self.msg)
            }
        }
    }

    #[cfg(feature = "std")]
    impl ser::StdError for Error {
        fn source(&self) -> Option<&(dyn ser::StdError + 'static)> {
            None
        }
    }

    impl ser::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: Display,
        {
            Error {
                #[cfg(feature = "alloc")]
                source: None,
                kind: ErrorKind::Custom,

                #[cfg(feature = "alloc")]
                msg: msg.to_string(),
                #[cfg(not(feature = "alloc"))]
                msg: "",
            }
        }
    }

    impl<E> From<minicbor::encode::Error<E>> for Error
    where
        E: Display + 'static,
    {
        fn from(e: minicbor::encode::Error<E>) -> Self {
            let kind = if e.is_message() {
                ErrorKind::Message
            } else {
                ErrorKind::Write
            };
            Error {
                #[cfg(feature = "alloc")]
                source: Some(Box::new(e)),
                kind,
                msg: make_msg(""),
            }
        }
    }

    pub(crate) fn make_kind_err(e: ErrorKind, msg: &'static str) -> Error {
        Error {
            #[cfg(feature = "alloc")]
            source: None,
            kind: e,
            msg: make_msg(msg),
        }
    }
}

pub mod de {
    use crate::lib::*;
    use core::fmt::{self, Debug, Display, Formatter};
    
    use minicbor::data::Type;

    use super::make_msg;
    use serde::de;

    #[derive(Debug)]
    pub struct Error {
        source: Option<minicbor::decode::Error>,
        pub kind: ErrorKind,
        #[cfg(not(feature = "alloc"))]
        msg: &'static str,
        #[cfg(feature = "alloc")]
        msg: String,
    }

    #[cfg(feature = "std")]
    impl de::StdError for Error {
        fn source(&self) -> Option<&(dyn de::StdError + 'static)> {
            if let Some(e) = self.source.as_ref() {
                Some(e)
            } else {
                None
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum ErrorKind {
        Message,
        /// Decoding has (unexpectedly) reached the end of the input slice.
        EndOfInput,
        /// Data item to decode is not a valid `char`.
        InvalidChar,
        TypeMismatch(Option<Type>),
        /// An unknown enum variant was encountered.
        UnknownVariant,
        /// A value was missing at the specified index.
        MissingValue,
        /// 128-bit integers are not supported at this time
        Unsupported128BitInteger,

        Custom,
        Unknow,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                ErrorKind::Message => write!(f, "Message"),
                ErrorKind::EndOfInput => write!(f, "EndOfInput"),
                ErrorKind::InvalidChar => write!(f, "InvalidChar"),
                ErrorKind::TypeMismatch(t) => {
                    if let Some(ty) = t.as_ref() {
                        write!(f, "TypeMismatch{{ {} }}", ty)
                    } else {
                        write!(f, "TypeMismatch")
                    }
                }
                ErrorKind::UnknownVariant => write!(f, "UnknownVariant"),
                ErrorKind::MissingValue => write!(f, "MissingValue"),
                ErrorKind::Unsupported128BitInteger => write!(f, "Unsupported128BitInteger"),
                ErrorKind::Custom => write!(f, "Custom"),
                ErrorKind::Unknow => write!(f, "Unknow"),
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            if let Some(e) = self.source.as_ref() {
                core::fmt::Display::fmt(&e, f)
            } else {
                write!(f, "error kind: {}, msg: {}", self.kind, self.msg)
            }
        }
    }

    impl de::Error for Error {
        fn custom<T>(msg: T) -> Self
        where
            T: Display,
        {
            make_custom_err(msg)
        }
    }

    #[inline]
    fn make_custom_err<T: Display>(msg: T) -> Error {
        Error {
            source: None,
            kind: ErrorKind::Custom,

            #[cfg(feature = "alloc")]
            msg: msg.to_string(),
            #[cfg(not(feature = "alloc"))]
            msg: "",
        }
    }

    impl From<minicbor::decode::Error> for Error {
        fn from(e: minicbor::decode::Error) -> Self {
            let kind = if e.is_end_of_input() {
                ErrorKind::EndOfInput
            } else if e.is_message() {
                ErrorKind::Message
            } else if e.is_type_mismatch() {
                // never reach
                ErrorKind::TypeMismatch(None)
            } else if e.is_unknown_variant() {
                ErrorKind::UnknownVariant
            } else if e.is_missing_value() {
                ErrorKind::MissingValue
            } else {
                // #[cfg(feature = "std")]
                // if e.is_custom() {
                //     ErrorKind::Custom
                // }
                ErrorKind::Unknow
            };
            Error {
                source: Some(e),
                kind,
                msg: make_msg(""),
            }
        }
    }
    #[inline]
    pub(crate) fn make_kind_err(e: ErrorKind, msg: &'static str) -> Error {
        Error {
            source: None,
            kind: e,
            msg: make_msg(msg),
        }
    }
    #[inline]
    pub(crate) fn type_mismatch(t: Type, s: &'static str) -> Error {
        Error {
            source: None,
            kind: ErrorKind::TypeMismatch(Some(t)),
            msg: make_msg(s),
        }
    }
}

#[cfg(not(feature = "alloc"))]
#[inline]
fn make_msg(msg: &'static str) -> &'static str {
    msg
}

use crate::lib::*;
#[cfg(feature = "alloc")]
#[inline]
fn make_msg(msg: &str) -> String {
    msg.into()
}
