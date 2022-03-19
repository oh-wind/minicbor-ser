# minicbor-ser

[![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/minicbor-ser.svg)](https://web.crev.dev/rust-reviews/crate/minicbor-ser/)

This repository provides a simple implementation of [serde] for [minicbor], making it easier to use.

# Quick start

Just like using other serde derived libraries:

```toml
[dependencies]
minicbor-ser = "0.1.*"
```

* Serialization

```rust
use minicbor_ser as cbor;
use serde::Serialize;
fn main(){
    #[derive(Debug, Serialize)]
    struct TestStruct {
        hello: String,
    }

    let test_struct = TestStruct {
            hello: "world".to_string(),
    };

    let value = cbor::to_vec(&test_struct).unwrap();
    assert_eq!(
        [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64],
        value.as_slice(),
    )
}
```

* Deserialization
```rust
use minicbor_ser as cbor;
use serde::Deserialize;
fn main(){
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        hello: String,
    }

    let data = [0xA1u8, 0x65, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x65, 0x77, 0x6F, 0x72, 0x6C, 0x64];
    
    let value: TestStruct = cbor::from_slice(&data[..]).unwrap();
    
    assert_eq!(
        TestStruct {
            hello: "world".to_string(),
        },
        value,
    );
}

```
By default, structs will be serialized as `map` and tuples will be serialized as `array`. If you don't want to wrap top-level struct and tuple with map and array, you can use `to_vec_flat`. To deserialize you should use `from_slice_flat`.

```rust
let expect = [0x01u8, 0x18, 0xff, 0x65, 0x68, 0x65, 0x6c, 0x6c, 0x6f];
let tuple_flatten = (0x01u8, 0xffu8, "hello");
let value = to_vec_flat(&tuple_flatten).unwrap();
assert_eq!(
    expet,
    value.as_slice(),
)

let data = crate::to_vec_flat(&exp).unwrap();
let value = from_slice_flat(&data).unwrap();
assert_eq!(exp, value);

```


# Type mapping table

The following represents how the minicbor-ser will map the types of Rust and CBOR
- ❌    : Not support
- ⚠    : Not perfect yet

|       Rust       |               CBOR                |
| :--------------: | :-------------------------------: |
| unsigned integer |         unsigned integer          |
| negative Integer |         negative Integer          |
|       u128       |                ❌                 |
|       i128       |                ❌                 |
|       &str       |              String               |
|      String      |              String               |
|      struct      |  map (if `flatten_top` is false)  |
|       Map        |                map                |
|      slice       | array (if `flatten_top` is false) |
|      &[u8]       |               bytes               |
|      tuple       | array (if `flatten_top` is false) |
|       Vec        | array (if `flatten_top` is false) |
| newtype variant  |                map                |
|   unit variant   |              String               |
|  tuple variant   |               array               |
|  struct variant  |                map                |


# no-std
The current `no-std` feature of minicbor-ser requires `alloc`. If your machine cannot use `alloc`, it is recommended that you use the `derive` feature that comes with `minicbor`.  
To enable `no-std`, use :

```toml
[dependencies]
minicbor-ser = { version = "0.1.*", default-features = false }
```

# Note
Some types of serialization and deserialization may be different from `minicbor`, depending on how `minicbor` is implemented.
If you need the default implementation of `minicbor`, please use `minicbor_ser::cbor` to access its API.



[serde]: https://serde.rs/
[minicbor]: https://crates.io/crates/minicbor