#![allow(unused_variables, dead_code)]
use super::Config;
pub use crate::error::en::Error;
use crate::error::en::ErrorKind;
use crate::lib::*;
use core::fmt::Display;
use minicbor::{encode::Write, Encoder};
use serde::serde_if_integer128;
use serde::{self, ser};

pub struct Serializer<W> {
    pub(crate) encoder: Encoder<W>,
    depth: u32,
    flatten_top: bool,
}

impl<T> Serializer<T>
where
    T: Write,
{
    pub fn new(w: T) -> Self {
        Serializer {
            encoder: Encoder::new(w),
            depth: 0,
            flatten_top: false,
        }
    }
    pub fn new_with_config(w: T, cfg: Config) -> Self {
        Serializer {
            encoder: Encoder::new(w),
            depth: 0,
            flatten_top: cfg.top_flatten,
        }
    }
    pub fn encoder(&mut self) -> &mut Encoder<T> {
        &mut self.encoder
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.encoder.bool(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.encoder.i8(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.encoder.i16(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.encoder.i32(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.encoder.i64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.encoder.u8(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.encoder.u16(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.encoder.u32(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.encoder.u64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.encoder.f32(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.encoder.f64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.encoder.char(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.encoder.str(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.encoder.bytes(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.encoder.null()?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        self.encoder.map(1)?.str(variant)?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.depth == 0 && self.flatten_top {
            return Ok(Compound::Map {
                ser: self,
                state: State::FlattenFirst,
            });
        }
        match len {
            Some(le) => {
                if le == 0 {
                    self.encoder.array(0)?;
                    Ok(Compound::Map {
                        ser: self,
                        state: State::Empty,
                    })
                } else {
                    self.encoder.array(le as u64)?;
                    Ok(Compound::Map {
                        ser: self,
                        state: State::First(Some(le)),
                    })
                }
            }
            None => {
                self.encoder.begin_array()?;
                Ok(Compound::Map {
                    ser: self,
                    state: State::First(None),
                })
            }
        }
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.encoder.map(1)?.str(variant)?;
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        if self.flatten_top && self.depth == 0 {
            return Ok(Compound::Map {
                ser: self,
                state: State::FlattenFirst,
            });
        }
        match len {
            Some(le) => {
                if le == 0 {
                    self.encoder.map(0)?;
                    Ok(Compound::Map {
                        ser: self,
                        state: State::Empty,
                    })
                } else {
                    self.encoder.map(le as u64)?;
                    Ok(Compound::Map {
                        ser: self,
                        state: State::First(Some(le)),
                    })
                }
            }
            None => {
                self.encoder.begin_map()?;
                Ok(Compound::Map {
                    ser: self,
                    state: State::First(None),
                })
            }
        }
    }

    #[inline]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        if self.flatten_top && self.depth == 0 {
            return Ok(Compound::Map {
                ser: self,
                state: State::FlattenFirst,
            });
        }
        self.encoder.map(1)?.str(variant)?;
        Ok(Compound::Map {
            ser: self,
            state: State::First(Some(len)),
        })
    }

    #[inline]
    fn collect_str<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Display,
    {
        // Temporary:
        self.serialize_str(&value.to_string())
    }

    serde_if_integer128! {
        #[inline]
        fn serialize_i128(self, v:i128) ->Result<Self::Ok,Self::Error> {
            Err(super::error::en::make_kind_err(ErrorKind::Unsupported128BitInteger, "128-bit integers are not currently supported."))
        }
    }
    serde_if_integer128! {
        #[inline]
        fn serialize_u128(self, v:u128) ->Result<Self::Ok,Self::Error> {
            Err(super::error::en::make_kind_err(ErrorKind::Unsupported128BitInteger, "128-bit integers are not currently supported."))
        }
    }
}

#[doc(hidden)]
#[derive(PartialEq, Eq)]
/// Not public API.
pub enum State {
    First(Option<usize>),
    Empty,
    Rest(Option<usize>),
    FlattenFirst,
    FlattenRest,
}

#[doc(hidden)]
/// Not public API.
pub enum Compound<'a, W: 'a> {
    Map {
        ser: &'a mut Serializer<W>,
        state: State,
    },
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                match *state {
                    State::First(size) => {
                        ser.depth += 1;
                        *state = State::Rest(size);
                    }
                    State::FlattenFirst => {
                        ser.depth += 1;
                        *state = State::FlattenRest;
                    }
                    _ => {}
                }
                value.serialize(&mut **ser)?;
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Rest(size) => {
                        if size.is_none() {
                            ser.encoder.end()?;
                        }
                        ser.depth -= 1;
                    }
                    State::FlattenRest => {
                        ser.depth -= 1;
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match *self {
            Compound::Map { .. } => ser::SerializeMap::serialize_entry(self, key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Map { .. } => ser::SerializeMap::end(self),
        }
    }
}

impl<'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                match *state {
                    State::First(size) => {
                        ser.depth += 1;
                        *state = State::Rest(size);
                    }
                    State::FlattenFirst => {
                        ser.depth += 1;
                        *state = State::FlattenFirst;
                    }
                    _ => {}
                }

                // The CBOR Key type allows for any type that implements
                key.serialize(&mut **ser)?;
                Ok(())
            }
        }
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match *self {
            Compound::Map {
                ref mut ser,
                ref mut state,
            } => {
                value.serialize(&mut **ser)?;
                Ok(())
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Rest(size) => {
                        if size.is_none() {
                            ser.encoder.end()?;
                        }
                        ser.depth -= 1;
                    }
                    State::FlattenRest => {
                        ser.depth -= 1;
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeStructVariant for Compound<'a, W>
where
    W: Write,
    W::Error: Display + 'static,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ser::Serialize,
    {
        match *self {
            Compound::Map { .. } => ser::SerializeStruct::serialize_field(self, key, value),
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeStruct::end(self)
    }
}

#[cfg(feature = "alloc")]
#[inline]
/// Serialize a COBR to Vec.
///
/// Have to be aware of is, this function will map the top-level `Struct` and `Tuple` to a cbor `map` and `array`.
/// If you want the top-level structure to be expanded, use the [`to_vec_flat`] function.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + ser::Serialize,
{
    let mut out = Vec::with_capacity(128);
    to_writer(value, &mut out)?;
    Ok(out)
}

#[cfg(feature = "alloc")]
#[inline]
/// Serialize a CBOR to Vec.
///
/// This function will serialize top-level `struct` and `tuple` in order.
/// So you should make sure their fields are in the same order.
pub fn to_vec_flat<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + ser::Serialize,
{
    let mut out = Vec::with_capacity(128);
    to_writer_cfg(value, &mut out, Config { top_flatten: true })?;
    Ok(out)
}

#[inline]
pub fn to_writer_cfg<W, T>(value: &T, writer: W, cfg: Config) -> Result<(), Error>
where
    W: Write,
    W::Error: Display + 'static,
    T: ?Sized + ser::Serialize,
{
    let mut se = Serializer::new_with_config(writer, cfg);
    value.serialize(&mut se)?;
    Ok(())
}

#[inline]
pub fn to_writer<W, T>(value: &T, writer: W) -> Result<(), Error>
where
    W: Write,
    W::Error: Display + 'static,
    T: ?Sized + ser::Serialize,
{
    let mut se = Serializer::new(writer);
    value.serialize(&mut se)?;
    Ok(())
}

#[inline]
pub fn by_encoder<T, W>(v: T, serializer: &mut Serializer<W>) -> Result<(), Error>
where
    T: minicbor::Encode,
    W: Write,
    W::Error: Display + 'static,
{
    serializer.encoder().encode(v)?;
    Ok(())
}

#[cfg(all(test, feature = "alloc"))]
mod ser_tests {

    use serde::Serialize;

    use super::*;

    macro_rules! assert_result {
        ($expect:expr, $data:expr , $flt:expr) => {{
            let mut out = Vec::with_capacity(128);
            to_writer_cfg(&$data, &mut out, Config { top_flatten: $flt }).unwrap();
            let __s: Vec<u8> = out;
            let __s = __s.as_slice();
            assert_eq!(
                $expect, __s,
                "\n left hex: {:x?} \n right hex: {:x?}\n",
                $expect, __s
            );
        }};
        ($expect:expr, $data:expr $(,)?) => {{
            assert_result!($expect, $data, false)
        }};
    }

    #[test]
    fn test_array() {
        let expect = [
            0x88u8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x18, 0x18, 0x18, 0xFF,
        ];
        let const_array = [0u8, 1, 2, 3, 4, 5, 0x18, 0xff];
        let vec_array = [0u8, 1, 2, 3, 4, 5, 0x18, 0xff].to_vec();
        let exp_empty = [0x80]; //empty array
        let empty_arr: [u8; 0] = [];

        assert_result!(expect, const_array);
        assert_result!(expect, vec_array);
        assert_result!(exp_empty, empty_arr);
    }

    #[test]
    fn test_map() {
        // {"hello": "world"}
        let expect = [
            0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64,
        ];
        let mut map = BTreeMap::new();
        map.insert("hello".to_string(), "world".to_string());
        assert_result!(expect, map);
        // {1:1}
        let expect = [0xA1u8, 1, 1];
        let mut map = BTreeMap::new();
        map.insert(1, 1);
        assert_result!(expect, map);
    }

    #[derive(Debug, Serialize)]
    struct TestStruct {
        hello: String,
    }

    #[derive(Debug, Serialize)]
    struct TestStruct2 {
        a: [u8; 2],
        b: TestStruct,
    }

    #[test]
    fn test_struct() {
        let expect = [
            0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64,
        ];
        let test_struct = TestStruct {
            hello: "world".to_string(),
        };
        assert_result!(expect, test_struct);
        let expect = [
            0xA2, 0x61, 0x61, 0x82, 01, 02, 0x61, 0x62, 0xA1, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F,
            0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64,
        ];
        let test_struct2 = TestStruct2 {
            a: [1, 2],
            b: test_struct,
        };
        assert_result!(expect, test_struct2);
    }

    #[test]
    fn test_tuple() {
        let expect = [
            0x88u8, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x18, 0x18, 0x18, 0xFF,
        ];
        let tuple_array = (0u8, 1, 2, 3, 4, 5, 0x18, 0xff);
        assert_result!(expect, tuple_array);

        let expect = [0x01u8, 0x18, 0xff, 0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
        let tuple_flatten = (0x01u8, 0xffu8, "hello");
        assert_result!(expect, tuple_flatten, true);
    }

    #[derive(Debug, Serialize)]
    enum TestEnum {
        A,
        B(i32),
        C(TestStruct),
        D(&'static [u8]),
    }
    #[test]
    fn test_enum() {
        let a = TestEnum::A;
        let b = TestEnum::B(1);
        let c = TestEnum::C(TestStruct {
            hello: "world".to_string(),
        });
        let d = TestEnum::D(&[1, 2, 3, 4][..]);
        assert_result!([0x61, 0x41], a);
        assert_result!([0xa1, 0x61, 0x42, 0x1], b);
        assert_result!(
            [
                0xa1, 0x61, 0x43, 0xa1, 0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x65, 0x77, 0x6f, 0x72,
                0x6c, 0x64
            ],
            c
        );
        assert_result!([0xa1, 0x61, 0x44, 0x84, 0x01, 0x02, 0x03, 0x04], d);
    }
}
