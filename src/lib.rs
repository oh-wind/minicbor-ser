#![cfg_attr(not(feature = "std"), no_std)]
//!
//! A simple implementation of [serde] for [minicbor]
//!
//! [serde]: https://serde.rs/
//! [minicbor]: https://crates.io/crates/minicbor
//!
//! * serialisation
//!
//! ```rust
//! use serde::Serialize;
//! #[derive(Debug, Serialize)]
//! struct TestStruct {
//!    hello: String,
//! }
//!
//! let test_struct = TestStruct {
//!         hello: "world".to_string(),
//! };
//!
//! let value = to_vec(&test_struct).unwrap();
//! assert_eq!(
//!     [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64],
//!     value.as_slice(),
//! )
//! ```
//!
//! * Deserialization
//!
//! ```rust
//! use serde::Deserialize;
//! #[derive(Debug, Deserialize, PartialEq)]
//! struct TestStruct {
//!     hello: String,
//! }
//!
//! let data = [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64];
//!
//! let value: TestStruct = from_slice(&data[..]).unwrap();
//!
//! assert_eq!(
//!     TestStruct {
//!         hello: "world".to_string(),
//!     },
//!     value,
//! );
//! }
//!
//! ```

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

pub mod de;
pub(crate) mod error;
pub mod ser;
pub use minicbor as cbor;

mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;

        #[cfg(feature = "std")]
        pub use std::*;
    }

    pub use self::core::cell::{Cell, RefCell};
    pub use self::core::clone::{self, Clone};
    pub use self::core::convert::{self, From, Into};
    pub use self::core::default::{self, Default};
    pub use self::core::fmt::{self, Debug, Display};
    pub use self::core::hash::{self, Hash};
    pub use self::core::iter::FusedIterator;
    pub use self::core::marker::{self, PhantomData};
    pub use self::core::ops::{Bound, RangeBounds};
    pub use self::core::result::{self, Result};
    pub use self::core::{borrow, char, cmp, iter, mem, num, ops, slice, str};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::string::{String, ToString};
    #[cfg(feature = "std")]
    pub use std::string::{String, ToString};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::{self, Vec};
    #[cfg(feature = "std")]
    pub use std::vec::{self, Vec};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::boxed::Box;
    #[cfg(feature = "std")]
    pub use std::boxed::Box;

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::collections::{btree_map, BTreeMap};
    #[cfg(feature = "std")]
    pub use std::collections::{btree_map, BTreeMap};
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Config {
    top_flatten: bool,
}

pub use de::from_slice;
pub use de::from_slice_flat;
pub use ser::to_writer;
pub use ser::to_writer_cfg;

#[cfg(feature = "alloc")]
pub use ser::to_vec;
#[cfg(feature = "alloc")]
pub use ser::to_vec_flat;

#[test]
fn test_ser() {
    use serde::Serialize;
    use self::lib::*;
    #[derive(Debug, Serialize)]
    struct TestStruct {
        hello: String,
    }

    let test_struct = TestStruct {
        hello: "world".to_string(),
    };

    let value = to_vec(&test_struct).unwrap();
    assert_eq!(
        [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64],
        value.as_slice(),
    )
}

#[test]
fn test_de() {
    use serde::Deserialize;
    use self::lib::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        hello: String,
    }

    let data = [
        0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64,
    ];

    let value: TestStruct = from_slice(&data[..]).unwrap();

    assert_eq!(
        TestStruct {
            hello: "world".to_string(),
        },
        value,
    );
}
