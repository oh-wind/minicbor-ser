#![allow(unused_variables)]

use crate::error::de::type_mismatch;

use super::error::de::{self as error, Error};
use super::Config;
use minicbor::data::Type;
use serde::de::{self, Unexpected};

pub struct Deserializer<'d> {
    decoder: minicbor::Decoder<'d>,
    depth: u32,
    flatten_top: bool,
    vec_bytes: Option<u8>
}

impl<'de> Deserializer<'de> {
    pub fn new(data: &'de [u8]) -> Self {
        Deserializer {
            decoder: minicbor::Decoder::new(data),
            depth: 0,
            flatten_top: false,
            vec_bytes:None,
        }
    }
    pub fn new_with_config(data: &'de [u8], cfg: Config) -> Self {
        Deserializer {
            decoder: minicbor::Decoder::new(data),
            depth: 0,
            flatten_top: cfg.top_flatten,
            vec_bytes: None,
        }
    }
    pub fn decoder(&mut self) -> &mut minicbor::Decoder<'de> {
        &mut self.decoder
    }

    pub fn deserialize_tag<V>(&mut self, tag: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        Err(type_mismatch(
            Type::Tag,
            "this type is not currently supported.",
        ))
    }

    /// internal API
    #[doc(hidden)]
    #[inline]
    fn depth_add(&mut self, depth: i32) -> Result<(), error::Error> {
        let m = self.depth as i32 + depth;
        if m < 0 {
            return Err(error::make_kind_err(
                error::ErrorKind::EndOfInput,
                "End of input.",
            ));
        }
        self.depth = m as u32;
        Ok(())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Bool => self.deserialize_bool(visitor),
            Type::Null => self.deserialize_option(visitor),
            Type::Undefined => self.deserialize_unit(visitor),
            Type::U8 => self.deserialize_u8(visitor),
            Type::U16 => self.deserialize_u16(visitor),
            Type::U32 => self.deserialize_u32(visitor),
            Type::U64 => self.deserialize_u64(visitor),
            Type::I8 => self.deserialize_i8(visitor),
            Type::I16 => self.deserialize_i16(visitor),
            Type::I32 => self.deserialize_i32(visitor),
            Type::I64 => self.deserialize_i64(visitor),
            Type::F16 => Err(type_mismatch(Type::F16, "rust doesn't support this type")),
            Type::F32 => self.deserialize_f32(visitor),
            Type::F64 => self.deserialize_f64(visitor),
            Type::Simple => Err(type_mismatch(
                Type::Simple,
                "rust doesn't support this type",
            )),
            Type::Bytes => self.deserialize_bytes(visitor),
            Type::BytesIndef => self.deserialize_bytes(visitor),
            Type::String => self.deserialize_str(visitor),
            Type::StringIndef => self.deserialize_str(visitor),
            Type::Array => self.deserialize_seq(visitor),
            Type::ArrayIndef => self.deserialize_seq(visitor),
            Type::Map => self.deserialize_map(visitor),
            Type::MapIndef => self.deserialize_map(visitor),
            Type::Tag => self.deserialize_tag(visitor),
            Type::Break => Err(type_mismatch(
                Type::Break,
                "break stop code outside indefinite length item",
            )),
            Type::Unknown(u) => Err(type_mismatch(
                Type::Unknown(u),
                "rust doesn't support this type",
            )),
            Type::Int => todo!(),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_bool(self.decoder.bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.decoder.i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.decoder.i16()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.decoder.i32()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.decoder.i64()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        if let Some(bs) = self.vec_bytes{
            return visitor.visit_u8(bs)
        }else {
            visitor.visit_u8(self.decoder.u8()?)
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.decoder.u16()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.decoder.u32()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.decoder.u64()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.decoder.f32()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.decoder.f64()?)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_char(self.decoder.char()?)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.decoder.str()?)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_borrowed_bytes(self.decoder.bytes()?)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Null | Type::Undefined => {
                self.decoder.skip()?;
                visitor.visit_none()
            }
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Null | Type::Undefined => visitor.visit_unit(),
            _ => Err(type_mismatch(Type::Null, "expected unit(null)")),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Array | Type::ArrayIndef => {
                let len = self.decoder.array()?;
                self.depth_add(1)?;
                let v = visitor.visit_seq(SeqAccess::new(self, len));
                self.depth_add(-1)?;
                v
            },
            Type::Bytes => {
                let bytes = self.decoder.bytes()?;
                visitor.visit_seq(SeqAccess::new_with_bytes(self, bytes))
            },
            e => {
                if self.flatten_top && self.depth == 0 {
                    return visitor.visit_seq(SeqAccess::new(self, None));
                }
                Err(type_mismatch(e, "expected array"))
            }
        }
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Map | Type::MapIndef => {
                let len = self.decoder.map()?;
                self.depth_add(1)?;
                let v = visitor.visit_map(MapAccess::new(self, len));
                self.depth_add(-1)?;
                v
            }
            e=> Err(type_mismatch(e, "expected map")),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::Map | Type::MapIndef => {
                let len = self.decoder.map()?;
                self.depth_add(1)?;
                let v = visitor.visit_map(MapAccess::new(self, len));
                self.depth_add(-1)?;
                v
            }
            Type::Array | Type::ArrayIndef => {
                let len = self.decoder.array()?;
                self.depth_add(1)?;
                let v = visitor.visit_seq(SeqAccess::new(self, len));
                self.depth_add(-1)?;
                v
            }
            e => {
                if self.flatten_top && self.depth == 0 {
                    return visitor.visit_seq(SeqAccess::new(self, None));
                }
                Err(type_mismatch(e, "expected map or array"))
            }
        }
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.decoder.datatype()? {
            Type::String | Type::StringIndef => visitor.visit_enum(EnumUnitAccess::new(self)),
            Type::Map | Type::MapIndef => {
                let len = self.decoder.map()?;
                if len == Some(1) || len == None {
                    self.depth_add(1)?;
                    let value = visitor.visit_enum(EnumVariantAccess::new(self))?;
                    self.depth_add(-1)?;
                    if len == None && Type::Break != self.decoder.datatype()? {
                        return Err(type_mismatch(
                            Type::Break,
                            "expected map with 1 element, but break code(0xff) was not found",
                        ));
                    }
                    Ok(value)
                } else {
                    Err(type_mismatch(Type::Map, "expected map with 1 element"))
                }
            }
            t => Err(type_mismatch(t, "expected map or string")),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SeqAccess<'a, 'de: 'a> {
    des: &'a mut Deserializer<'de>,
    len: Option<u64>,
    index: u64,
    bytes: Option<&'a [u8]>
}

impl<'a, 'de> SeqAccess<'a, 'de> {
    fn new(des: &'a mut Deserializer<'de>, len: Option<u64>) -> Self {
        SeqAccess { des, len,  index: 0, bytes: None }
    }
    fn new_with_bytes(des: &'a mut Deserializer<'de>, bytes: &'a [u8]) -> Self{
        SeqAccess { des, len: Some(bytes.len() as u64), index: 0, bytes: Some(bytes) }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(bytes) = self.bytes{
            if self.index >= self.len.unwrap(){
                return Ok(None)
            }
            self.des.vec_bytes = Some(bytes[self.index as usize])
        }

        self.index += 1;

        match self.len {
            None => {
                let decoder = self.des.decoder();
                match decoder.datatype()? {
                    Type::Break => Ok(None),
                    _ => Ok(Some(seed.deserialize(&mut *self.des)?)),
                }
            }
            Some(len) => {
                if len == 0 {
                    return Ok(None);
                }
                if self.index > len {
                    self.des.vec_bytes = None;
                    return Ok(None);
                }
                Ok(Some(seed.deserialize(&mut *self.des)?))
            }
        }
    }
}


struct MapAccess<'a, 'de: 'a> {
    des: &'a mut Deserializer<'de>,
    len: Option<u64>,
    index: u64,
}
impl<'a, 'de> MapAccess<'a, 'de> {
    fn new(des: &'a mut Deserializer<'de>, len: Option<u64>) -> Self {
        MapAccess { des, len, index: 0 }
    }
}

impl<'a, 'de> de::MapAccess<'de> for MapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        self.index += 1;
        match self.len {
            None => {
                let decoder = self.des.decoder();
                match decoder.datatype()? {
                    Type::Break => Ok(None),
                    _ => {
                        let key = seed.deserialize(&mut *self.des)?;
                        // self.index += 1;
                        Ok(Some(key))
                    }
                }
            }
            Some(len) => {
                if len == 0 {
                    return Ok(None);
                }
                if self.index > len {
                    return Ok(None);
                }
                let key = seed.deserialize(&mut *self.des)?;
                Ok(Some(key))
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let decoder = self.des.decoder();
        match decoder.datatype()? {
            Type::Break => Err(type_mismatch(
                Type::Break,
                "expect value, but found break stop code(0xFF).",
            )),
            _ => seed.deserialize(&mut *self.des),
        }
    }
}

struct EnumVariantAccess<'a, 'de: 'a> {
    des: &'a mut Deserializer<'de>,
}
impl<'a, 'de> EnumVariantAccess<'a, 'de> {
    fn new(des: &'a mut Deserializer<'de>) -> Self {
        EnumVariantAccess { des }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for EnumVariantAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.des)?, self))
    }
}

impl<'de, 'a> de::VariantAccess<'de> for EnumVariantAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        de::Deserialize::deserialize(self.des)
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.des)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_seq(self.des, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        de::Deserializer::deserialize_struct(self.des, "", fields, visitor)
    }
}

struct EnumUnitAccess<'a, 'de: 'a> {
    des: &'a mut Deserializer<'de>,
}
impl<'a, 'de> EnumUnitAccess<'a, 'de> {
    fn new(des: &'a mut Deserializer<'de>) -> Self {
        EnumUnitAccess { des }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for EnumUnitAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.des)?, self))
    }
}
impl<'de, 'a> de::VariantAccess<'de> for EnumUnitAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"tuple variant",
        ))
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(de::Error::invalid_type(
            Unexpected::UnitVariant,
            &"struct variant",
        ))
    }
}

#[inline]
pub fn from_slice<'a, T>(data: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::new(data);
    let value = T::deserialize(&mut deserializer)?;
    Ok(value)
}

pub fn from_slice_flat<'a, T>(data: &'a [u8]) -> Result<T, Error>
where
    T: de::Deserialize<'a>,
{
    let mut deserializer = Deserializer::new_with_config(data, Config { top_flatten: true });
    let value = T::deserialize(&mut deserializer)?;
    Ok(value)
}

#[cfg(feature = "alloc")]
#[cfg(test)]
pub mod de_test {

    use super::*;
    use serde::{Deserialize, Serialize};

    #[cfg(not(feature = "std"))]
    use crate::lib::*;

    use crate::lib::BTreeMap;

    #[test]
    fn test_seq() {
        let expect = [[2, 3, 0xff]];
        let data = [0x81u8, 0x83, 2, 3, 0x18, 0xff];
        let value: Vec<Vec<u8>> = from_slice(&data).unwrap();
        let s = value[0].as_slice();
        assert_eq!(expect[0], s);

        let expect = "hello".to_string();
        let data = [0x45u8, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
        let value: Vec<u8> = from_slice(&data).unwrap();
        let hello = String::from_utf8(value).unwrap();
        assert_eq!(expect, hello);
    }

    #[test]
    fn test_tuple() {
        let expect = (0x01_u8, 0xff, "hello");
        let data = [0x01_u8, 0x18, 0xff, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
        let value: (u8, i32, String) = from_slice_flat(&data).unwrap();
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestStruct {
        a: u8,
        b: u16,
        c: u32,
        d: u64,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct TestStruct2 {
        a: BTreeMap<i32, i32>,
        b: u8,
    }

    #[test]
    fn test_struct() {
        let expect = TestStruct {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
        };
        let data = [0x84u8, 1, 2, 3, 4];
        let value: TestStruct = from_slice(&data).unwrap();
        assert_eq!(expect, value);

        let data = crate::ser::to_vec(&expect).unwrap();
        let value: TestStruct = from_slice(&data).unwrap();
        assert_eq!(expect, value, "expect: {:x?}, value: {:x?}", expect, value);

        // Out of order map
        let data = [
            0xa4, 0x61, 0x62, 0x02, 0x61, 0x61, 0x01, 0x61, 0x64, 0x04, 0x61, 0x63, 0x03,
        ];
        let value: TestStruct = from_slice(&data).unwrap();
        assert_eq!(expect, value);

        let data = [
            0xBF, 0x61, 0x62, 0x02, 0x61, 0x61, 0x01, 0x61, 0x64, 0x04, 0x61, 0x63, 0x03, 0xFF,
        ];
        let value: TestStruct = from_slice(&data).unwrap();
        assert_eq!(expect, value);

        let exp = TestStruct2 {
            a: {
                let mut mp = BTreeMap::new();
                mp.insert(1, 1);
                mp.insert(0xff, 0xff);
                mp
            },
            b: 0,
        };

        let data = [
            0xA2, 0x61, 0x61, 0xA2, 0x01, 0x01, 0x18, 0xFF, 0x18, 0xFF, 0x61, 0x62, 0x00,
        ];
        let value: TestStruct2 = from_slice(&data).unwrap();
        assert_eq!(exp, value);
    }

    #[test]
    fn test_flat() {
        let exp = (0x01u8, "a", 0xffi32);
        let data = [0x01, 0x61, 0x61, 0x18, 0xFF];
        let value = from_slice_flat(&data).unwrap();
        assert_eq!(exp, value);

        let data = crate::to_vec_flat(&exp).unwrap();
        let value = from_slice_flat(&data).unwrap();
        assert_eq!(exp, value);
    }

    macro_rules! test_enum {
        ($($data:expr , $test:expr ;)+) => {$({
            let __data = $data;
            let __value: TestEnum = from_slice(&__data[..]).unwrap();
            assert_eq!($test, __value, "\n data: {:x?}", __data);

            let __data = crate::ser::to_vec(&$test).unwrap();
            let __value: TestEnum = from_slice(&__data[..]).unwrap();
            assert_eq!($test, __value, "\n data: {:x?}", __data);
        })+}
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum TestEnum {
        A,
        B(u8),
        C(TestStruct),
        D((u8, u8)),
    }
    #[test]
    fn test_enum() {
        test_enum! { [0x61, 0x41], TestEnum::A; }
        test_enum! { [0xA1, 0x61, 0x42, 0x01], TestEnum::B(1); }
        test_enum! { [0xA1, 0x61, 0x42, 0x18, 0xff], TestEnum::B(0xff);}
        test_enum! { [0xA1, 0x61, 0x43, 0xa4, 0x61, 0x62, 0x02, 0x61, 0x61, 0x01, 0x61, 0x64, 0x04, 0x61, 0x63, 0x03],
        TestEnum::C(TestStruct{a: 1, b: 2, c: 3, d: 4});}
        test_enum! { [0xBF, 0x61, 0x43, 0xa4, 0x61, 0x62, 0x02, 0x61, 0x61, 0x01, 0x61, 0x64, 0x04, 0x61, 0x63, 0x03, 0xFF],
        TestEnum::C(TestStruct{a: 1, b: 2, c: 3, d: 4});}
        test_enum! {     [0xA1, 0x61, 0x44, 0x82, 0x01, 0x02 ], TestEnum::D((1, 2));}
    }
}
