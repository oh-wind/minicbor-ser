#![allow(unused_variables, dead_code)]
use crate::error::{Error, ErrorKind};
use crate::lib::*;
use core::fmt::Display;
use minicbor::{encode::Write,  Encoder};
use serde::{self, ser};
use serde::serde_if_integer128;

pub struct Serializer<W> {
    pub(crate) encoder: Encoder<W>,
}


impl<T> Serializer<T>
where
    T: Write,
{
    pub fn new(w: T) -> Self {
        Serializer {
            encoder: Encoder::new(w),
        }
    }
    pub fn encoder(&mut self) -> &mut Encoder<T>{
        &mut self.encoder
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
    W::Error: Display,
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
                        state: State::SizedFirst(le),
                    })
                }
            }
            None => {
                self.encoder.begin_array()?;
                Ok(Compound::Map {
                    ser: self,
                    state: State::UnsizedFirst,
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
                        state: State::SizedFirst(le),
                    })
                }
            }
            None => {
                self.encoder.begin_map()?;
                Ok(Compound::Map {
                    ser: self,
                    state: State::UnsizedFirst,
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
        self.encoder.map(1)?.str(variant)?;
        Ok(Compound::Map {
            ser: self,
            state: State::SizedFirst(len),
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
            Err(super::error::make_kind_err(ErrorKind::Unsupported128BitInteger))
        }
    }
    serde_if_integer128! {
        #[inline]
        fn serialize_u128(self, v:u128) ->Result<Self::Ok,Self::Error> {
            Err(super::error::make_kind_err(ErrorKind::Unsupported128BitInteger))
        }
    }
}

#[doc(hidden)]
#[derive(PartialEq, Eq)]
/// Not public API.
pub enum State {
    SizedFirst(usize),
    UnsizedFirst,
    Empty,
    SizedRest,
    UnsizedRest,
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
    W::Error: Display,
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
                    State::SizedFirst(_) => {
                        *state = State::SizedRest;
                    }
                    State::UnsizedFirst => {
                        *state = State::UnsizedRest;
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
                    State::Empty => {}
                    State::UnsizedRest => {
                        ser.encoder.end()?;
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
    W::Error: Display,
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
    W::Error: Display,
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
    W::Error: Display,
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
    W::Error: Display,
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
        match self {
            Compound::Map { ser, state } => {
                match state {
                    State::Empty => {}
                    State::UnsizedRest => {
                        ser.encoder.end()?;
                    }
                    _ => {}
                }
                Ok(())
            }
        }
    }
}

impl<'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
    W::Error: Display,
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
                    State::SizedFirst(_) => {
                        *state = State::SizedRest;
                    }
                    State::UnsizedFirst => {
                        *state = State::UnsizedRest;
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
                    State::Empty => {}
                    State::UnsizedRest => {
                        ser.encoder.end()?;
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
    W::Error: Display,
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
        match self {
            Compound::Map { ser, state } => {
                Ok(())
            }
        }
    }
}



#[inline]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + ser::Serialize 
{
    let mut out = Vec::with_capacity(128);
    to_writer(value, &mut out)?;
    Ok(out)
}

#[inline]
pub fn to_writer<W, T>( value: &T, writer: W) -> Result<(),Error>
where
    W: Write,
    W::Error: Display,
    T: ?Sized + ser::Serialize,
{
    let mut se = Serializer::new(writer);
    value.serialize(&mut se)?;
    Ok(())
}

#[inline]
pub fn by_encoder<T: minicbor::Encode, W>(v: T, serializer:&mut Serializer<W>) -> Result<(), Error>
where 
    W:Write,
    W::Error: Display 
{
    serializer.encoder().encode(v)?;
    Ok(())
}


#[cfg(test)]
mod ser_tests {

    use serde::Serialize;

    use super::*;
    
    macro_rules! assert_result {
        ($expect:expr, $data:expr $(,)?) => ({
            let __s : Vec<u8> = to_vec(&$data).unwrap();
            let __s = __s.as_slice();
            assert_eq!($expect, __s, "\n left hex: {:x?} \n right hex: {:x?}\n", $expect, __s);
        });
    }

    #[test]
    fn test_array(){
        let expect = [0x88u8,0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x18,0x18, 0x18, 0xFF];
        let const_array = [0u8, 1, 2, 3, 4, 5, 0x18, 0xff];
        let vec_array = [0u8, 1, 2, 3, 4, 5, 0x18, 0xff].to_vec();
        assert_result!(expect, const_array);
        assert_result!(expect, vec_array);
    }

    #[test]
    fn test_map(){
        // {"hello": "world"}
        let expect = [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64];
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
    struct TestStruct2{
        a: [u8;2],
        b: TestStruct,
    }

    #[test]
    fn test_struct(){
        let expect = [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64];
        let test_struct = TestStruct {
            hello: "world".to_string(),
        };
        assert_result!(expect, test_struct);
        let expect = [0xA2, 0x61, 0x61, 0x82, 01, 02, 0x61, 0x62, 0xA1, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64];
        let test_struct2 = TestStruct2{
            a: [1,2],
            b: test_struct,
        };
        assert_result!(expect, test_struct2);
    }

    #[test]
    fn test_tuple(){
        let expect = [0x88u8,0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x18,0x18, 0x18, 0xFF];
        let tuple_array = (0u8, 1, 2, 3, 4, 5, 0x18, 0xff);
        assert_result!(expect, tuple_array);
    }


    #[derive(Debug, Serialize)]
    enum TestEnum {A,B(i32),C(TestStruct),D(&'static [u8])}
    #[test]
    fn test_enum(){
        let a = TestEnum::A;
        let b = TestEnum::B(1);
        let c = TestEnum::C(TestStruct{hello: "world".to_string()});
        let d = TestEnum::D(&[1,2,3,4][..]);
        assert_result!([0x61, 0x41], a);
        assert_result!([0xa1, 0x61, 0x42, 0x1], b);
        assert_result!([0xa1, 0x61, 0x43, 0xa1, 0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x65, 0x77, 0x6f, 0x72, 0x6c, 0x64], c);
        assert_result!([0xa1, 0x61, 0x44, 0x84, 0x01, 0x02, 0x03, 0x04], d);
    }
}
