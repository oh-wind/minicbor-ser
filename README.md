# minicbor-ser

This repository provides a simple implementation of [serde] for [minicbor], making it easier to use.

# Quick start

Just like using other serde derived libraries:

```toml
[dependencies]
minicbor-ser = "0.1.1"
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
# Type mapping table

The following represents how the minicbor-ser will map the types of Rust and CBOR
- ❌    : Not support
- ⚠    : Not perfect yet

|       Rust       |       CBOR       |
| :--------------: | :--------------: |
| unsigned integer | unsigned integer |
| negative Integer | negative Integer |
|       u128       |        ❌        |
|       i128       |        ❌        |
|       &str       |      String      |
|      String      |      String      |
|      struct      |       map        |
|       Map        |       map        |
|      slice       |      array       |
|      &[u8]       |      bytes       |
|      tuple       |      array       |
|       Vec        |      array       |
| newtype variant  |       map        |
|   unit variant   |      String      |
|  tuple variant   |      array       |
|  struct variant  |       map        |
|        ⚠        |       tag        |


# no-std
The current `no-std` feature of minicbor-ser requires `alloc`. If your machine cannot use `alloc`, it is recommended that you use the `derive` feature that comes with `minicbor`.  
To enable `no-std`, use :

```toml
[dependencies]
minicbor-ser = { version = "0.1.1", default-features = false }
```



[serde]: https://serde.rs/
[minicbor]: https://crates.io/crates/minicbor