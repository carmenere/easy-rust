<!-- TOC -->
* [Crate serde](#crate-serde)
  * [Serde data model](#serde-data-model)
    * [Module serde::de](#module-serdede)
    * [Module serde::ser](#module-serdeser)
    * [serde_json::Serializer](#serde_jsonserializer)
    * [serde_json::Deserializer](#serde_jsondeserializer)
  * [Relationship between data formats, data types and intermediate types](#relationship-between-data-formats-data-types-and-intermediate-types)
  * [Example: impl custom `serde::de::Deserialize` and `serde::de::Visitor` for `i32`](#example-impl-custom-serdededeserialize-and-serdedevisitor-for-i32)
  * [Crate `serde_json`](#crate-serde_json)
    * [Example](#example)
    * [`serde_json::Value`](#serde_jsonvalue)
    * [Macros `serde_json::json`](#macros-serde_jsonjson)
* [Custom serializers/deserializers](#custom-serializersdeserializers)
  * [Attributes `serialize_with`/`deserialize_with`/`with`](#attributes-serialize_withdeserialize_withwith)
    * [Example 1: Custom serializer using `serialize_with`](#example-1-custom-serializer-using-serialize_with)
    * [Example 2: Custom serializer implementing `Serialize`](#example-2-custom-serializer-implementing-serialize)
    * [Example 3: Custom deserializer](#example-3-custom-deserializer)
  * [Crate `serde_with`](#crate-serde_with)
* [`serde` source code snippets](#serde-source-code-snippets)
  * [Methods for deserialize numbers of `serde::de::Deserializer` trait](#methods-for-deserialize-numbers-of-serdededeserializer-trait)
  * [Methods for deserialize numbers of `serde::de::Deserialize` trait](#methods-for-deserialize-numbers-of-serdededeserialize-trait)
<!-- TOC -->

<br>

# Crate serde
## Serde data model
Serde consists of 3 layers:
1. **Data formats** like **JSON**, **YAML**, **TOML**, **CSV** and so on.
2. **Data types** or **Rust data types**.
3. **Serde intermediate types** (aka **intermediate format**) that provide mapping between **Data formats** and **Data types**.

<br>

### Module serde::de
Deserialization in serde uses **visitor pattern** (aka **double dispatch**).<br>

Provides traits
- [**serde::de::Deserialize**](https://docs.rs/serde/latest/serde/de/trait.Deserialize.html): a type that implements `Deserialize` is a **data type**;
- [**serde::de::Deserializer**](https://docs.rs/serde/latest/serde/de/trait.Deserializer.html): a type that implements `Deserializer` is a **data format**;
- [**serde::de::Visitor**](https://docs.rs/serde/latest/serde/de/trait.Visitor.html): this trait represents a **visitor** that walks through a **deserializer**;

<br>

### Module serde::ser
Provides traits
- [**serde::ser::Serialize**](https://docs.rs/serde/latest/serde/ser/trait.Serialize.html): a type that implements `Serialize` is a **data type**;
- [**serde::ser::Serializer**](https://docs.rs/serde/latest/serde/ser/trait.Serializer.html): a type that implements `Serializer` is a **data format**;

<br>

### serde_json::Serializer
Struct [**serde_json::Serializer**](https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html).<br>
A structure for serializing Rust values into JSON.<br>

**Methods**:
- `new(writer: W) -> Self` creates a new **JSON serializer**;

<br>

### serde_json::Deserializer
Struct [**serde_json::Deserializer**](https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html).<br>
A structure that deserializes JSON into Rust values.<br>

<br>

**Methods**:
- `new(read: R) -> Self` create a new **JSON deserializer** from one of the possible `serde_json` **input sources**;
- `from_reader(reader: R) -> Self` creates a **JSON deserializer** from an `io::Read`;
- `from_slice(bytes: &'a [u8]) -> Self ` creates a **JSON deserializer** from a `&str`;
- `from_str(s: &'a str) -> Self ` creates a **JSON deserializer** from a `&str`;

<br>

## Relationship between data formats, data types and intermediate types
In serde the _data formats_ are **completely decoupled** from _data types_:
- _data types_ don't know about _data formats_;
- _data formats_ don't know about _data types_;

<br>

**Deserialization**:<br>
![Deserialization](/img/deserialization.png)

<br>

**Serialization**:<br>
![Serialization](/img/serialization.png)

<br>

So, **data type** `T` that implements `de::Deserialize` can be _deserialized_ **from** any **data format** supported by Serde.<br>
So, **data type** `T` that implements `de::Serialize` can be _serialized_ **to** any **data format** supported by Serde.<br>

<br>

Serde provides `Serialize`/`Deserialize` implementations for many _Rust primitive_ and _standard library types_.<br>
Serde provides a **procedural macro** called `serde_derive` to **automatically generate** `Serialize`/`Deserialize` **implementations** for `structs` and `enums` in your program.<br>
The `Serializer`/`Deserializer` **implementations** are provided by **third-party crates**, for example `serde_json`, `serde_yaml` and so on.<br>

<br>

**Relationship** between `Deserialize`, `Deserializer` and `Visitor`:
- The `Deserialize` trait provides the `deserialize()` method. This method takes in a **deserializer**.<br>
- The **deserializer** must implement `Deserializer` trait.<br>
- The **deserializer** is _responsible_ for converting the **input format** into some kind of **intermediate format** (**serde type**).<br>
- The `Deserializer` trait provides various methods, and they all takes a **visitor**. The **visitor** must implement `Visitor` trait.<br>
- The **visitor** is _responsible_ for converting the **intermediate format** into the **specific data type**.<br>
- The **deserializer** calls **appropriate method** for **visitor** it got, passing it a **value** of **intermediate type**.<br>
- The `Visitor` trait provides various methods, and they all takes a **value** of **intermediate type**.<br>

<br>

The `serde_json::from_str()` is an **entrypoint** to **deserializing** process:
- `serde_json::from_str<'a, T: Deserialize<'a>>(input) -> Result<T>`
  - calls `from_trait(input)` which:
    - inits **deserializer**: `let mut de = Deserializer::new(read);`
    - calls `<T as de::Deserialize>::deserialize(de)` and passes **deserializer** `de` to it
      - calls appropriate method on deserializer `deserializer.deserialize_xxx(visitor)` and passes to it `visitor` that implements `Visitor`
        - then `deserialize_xxx(visitor)`:
          - performs parsing of **input** and produces **value** of **intermediate type**
          - calls appropriate method on `visitor`: `visitor.visit_xxx(value)` where `value` is of some **serde intermediate type**
            - it converts from **serde intermediate type** to appropriate rust **data type** and returns **instance** of appropriate **data type**

<br>

The `serde_json::to_string()` is an **entrypoint** to **serializing** process:
- `serde_json::to_string<T: ?Sized + Serialize>(value: &T) -> Result<String>`
  - calls `to_vec<T>(value: &T) -> Result<Vec<u8>>`
    - calls `to_writer<W, T>(writer: W, value: &T) -> Result<()>` which:
      - init **serializer**:: `ser = Serializer::new(writer);`
      - calls `serialize()` method on instance of type `T`: `value.serialize(&mut ser)` and passes **serializer** `ser` to it
        - calls appropriate method on serializer `serializer.serialize_xxx(...)`

<br>

**Consider example**:
```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    
    let p: Person = serde_json::from_str(data)?;
}
```

<br>

Let's find the definition of `from_str` function [**here**](https://docs.rs/serde_json/latest/src/serde_json/de.rs.html):
```rust
pub fn from_str<'a, T>(s: &'a str) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    from_trait(read::StrRead::new(s))
}
```

Which in turn calls `from_trait`:
```rust
fn from_trait<'de, R, T>(read: R) -> Result<T>
where
    R: Read<'de>,
    T: de::Deserialize<'de>,
{
    let mut de = Deserializer::new(read);
    let value = tri!(de::Deserialize::deserialize(&mut de)); // <T as de::Deserialize>::deserialize(&mut de)

    // Make sure the whole stream has been consumed.
    tri!(de.end());
    Ok(value)
}

// We only use our own error type; no need for From conversions provided by the
// standard library's try! macro. This reduces lines of LLVM IR by 4%.
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}
```

Pay attention to `de::Deserialize::deserialize(&mut de)` it is the same as `<T as de::Deserialize>::deserialize(&mut de)`, where
- `T` is a **data type** that we get after input had been deserialized. In above example it is variable `p` that contain value of `Person` type;
- `de` is a **deserializer** that implements `Deserializer` trait and can work with input **data format**;
So, the `Person` type must implement `de::Deserialize`.<br>

<br>

## Example: impl custom `serde::de::Deserialize` and `serde::de::Visitor` for `i32`
```rust
use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;
use serde::de::{ Deserialize, Deserializer, Visitor, Error };

impl<'de> Deserialize<'de> for i32 {
    fn deserialize<D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(I32Visitor)
    }
}

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value as i64)
    }
}
```

<br>

## Crate `serde_json`
### Example
```rust
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Debug, Deserialize,Serialize)]
struct Test {
    m: u16,
    o: Option<u16>
}

fn main() {
let d1 = r#"{"m": 100, "o": 20}"#;
let t: Test = serde_json::from_str(&d1).unwrap();
println!("{}", serde_json::to_string_pretty(&t).unwrap());

let d2 = r#"{"m": 100}"#;
let t: Test = serde_json::from_str(&d2).unwrap();
println!("{}", serde_json::to_string_pretty(&t).unwrap());
}
```

### `serde_json::Value`
```rust
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Map<String, Value>),
}
```

Represents any valid JSON value.

### Macros `serde_json::json`
Construct a `serde_json::Value` from a JSON literal.

```rust
let code = 200;
let features = vec!["serde", "json"];

let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }
});
```

<br>

# Custom serializers/deserializers
## Attributes `serialize_with`/`deserialize_with`/`with`
- `#[serde(serialize_with = "path")]`
  - Serialize this field using a custom function. The given function must be callable as `fn<S>(&T, S) -> Result<S::Ok, S::Error> where S: Serializer`
- #[serde(deserialize_with = "path")]
  - Deserialize this field using a custom function. The given function must be callable as `fn<'de, D>(D) -> Result<T, D::Error> where D: Deserializer<'de>`.
- #[serde(with = "module")]
  - Combination of `serialize_with` and `deserialize_with`. Serde will use `$module::serialize` as the `serialize_with` function and `$module::deserialize` as the `deserialize_with` function.

<br>

### Example 1: Custom serializer using `serialize_with`
```rust
use serde::{Serialize, Serializer};

fn serialize_option_string<S>(value: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(v),
        None => serializer.serialize_str(""),
    }
}

#[derive(Serialize)]
struct MyStruct {
    #[serde(serialize_with = "serialize_option_string")]
    name: Option<String>,
}

fn main() {
    let my_struct = MyStruct {
        name: None,
    };

    let json = serde_json::to_string(&my_struct).unwrap();
    println!("Serialized JSON: {}", json);
}
```

<br>

### Example 2: Custom serializer implementing `Serialize`
```rust
use serde::{Serialize, Serializer, ser::SerializeStruct};
use chrono::{DateTime, Utc, NaiveDate};

struct Event {
  name: String,
  date: DateTime<Utc>,
}

fn serialize_date<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
        S: Serializer,
{
  let formatted_date = date.format("%Y-%m-%d").to_string();
  serializer.serialize_str(&formatted_date)
}

impl Serialize for Event {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
          S: Serializer
  {
    let mut state = serializer.serialize_struct("Event", 2)?;
    state.serialize_field("name", &self.name)?;
    state.serialize_field("date", &serialize_date(&self.date, serializer)?)?;
    state.end()
  }
}

fn main() {
  let event = Event {
    name: "RustConf".to_string(),
    date: DateTime::from_utc(NaiveDate::from_ymd(2022, 9, 12).and_hms(0, 0, 0), Utc),
  };

  let serialized = serde_json::to_string(&event).unwrap();
  println!("Serialized: {}", serialized);
}
```

<br>

### Example 3: Custom deserializer
```rust
use serde::{Deserialize, Deserializer};
use chrono::{DateTime, Utc, NaiveDate};
use serde::de::{self, Visitor};
use std::fmt;

struct DateTimeVisitor;

impl<'de> Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in the format YYYY-MM-DD")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(|date| DateTime::from_utc(date.and_hms(0, 0, 0), Utc))
            .map_err(de::Error::custom)
    }
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DateTimeVisitor)
}

#[derive(Deserialize, Debug)]
struct Event {
    name: String,
    #[serde(deserialize_with = "deserialize_date")]
    date: DateTime<Utc>,
}

fn main() {
    let data = r#"{"name": "RustConf", "date": "2025-03-20"}"#;
    let event: Event = serde_json::from_str(data).unwrap();
    dbg!(event);
}
```

<br>

## Crate `serde_with`
The `serde_with` crate which allows to derive `Serialize` and `Deserialize` using implementations of the `Display` and `FromStr` traits.<br>

<br>

# `serde` source code snippets
## Methods for deserialize numbers of `serde::de::Deserializer` trait
Methods for deserialize **numbers** (`deserialize_i8/u8/i16/u16` and so on) are generated by macros `deserialize_number!`.<br>

```rust
macro_rules! deserialize_number {
    ($method:ident) => {
        deserialize_number!($method, deserialize_number);
    };

    ($method:ident, $using:ident) => {
        fn $method<V>(self, visitor: V) -> Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            self.$using(visitor)
        }
    };
}

pub(crate) enum ParserNumber {
    F64(f64),
    U64(u64),
    I64(i64),
    #[cfg(feature = "arbitrary_precision")]
    String(String),
}

impl ParserNumber {
    fn visit<'de, V>(self, visitor: V) -> Result<V::Value>
    where
    V: de::Visitor<'de>,
    {
        match self {
            ParserNumber::F64(x) => visitor.visit_f64(x),
            ParserNumber::U64(x) => visitor.visit_u64(x),
            ParserNumber::I64(x) => visitor.visit_i64(x),
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(x) => visitor.visit_map(NumberDeserializer { number: x.into() }),
        }
    }
}

deserialize_number!(deserialize_i8);
deserialize_number!(deserialize_i16);
deserialize_number!(deserialize_i32);
deserialize_number!(deserialize_i64);
deserialize_number!(deserialize_u8);
deserialize_number!(deserialize_u16);
deserialize_number!(deserialize_u32);
deserialize_number!(deserialize_u64);
#[cfg(not(feature = "float_roundtrip"))]
deserialize_number!(deserialize_f32);
deserialize_number!(deserialize_f64);

impl<'de, R: Read<'de>> Deserializer<R> {
    fn parse_number(&mut self, positive: bool, significand: u64) -> Result<ParserNumber> {
        Ok(match tri!(self.peek_or_null()) {
            b'.' => ParserNumber::F64(tri!(self.parse_decimal(positive, significand, 0))),
            b'e' | b'E' => ParserNumber::F64(tri!(self.parse_exponent(positive, significand, 0))),
            _ => {
                if positive {
                    ParserNumber::U64(significand)
                } else {
                    let neg = (significand as i64).wrapping_neg();

                    // Convert into a float if we underflow, or on `-0`.
                    if neg >= 0 {
                        ParserNumber::F64(-(significand as f64))
                    } else {
                        ParserNumber::I64(neg)
                    }
                }
            }
        })
    }
    
    fn parse_integer(&mut self, positive: bool) -> Result<ParserNumber> {
        let next = match tri!(self.next_char()) {
            Some(b) => b,
            None => {
                return Err(self.error(ErrorCode::EofWhileParsingValue));
            }
        };

        match next {
            b'0' => {
                // There can be only one leading '0'.
                match tri!(self.peek_or_null()) {
                    b'0'..=b'9' => Err(self.peek_error(ErrorCode::InvalidNumber)),
                    _ => self.parse_number(positive, 0),
                }
            }
            c @ b'1'..=b'9' => {
                let mut significand = (c - b'0') as u64;

                loop {
                    match tri!(self.peek_or_null()) {
                        c @ b'0'..=b'9' => {
                            let digit = (c - b'0') as u64;

                            // We need to be careful with overflow. If we can,
                            // try to keep the number as a `u64` until we grow
                            // too large. At that point, switch to parsing the
                            // value as a `f64`.
                            if overflow!(significand * 10 + digit, u64::MAX) {
                                return Ok(ParserNumber::F64(tri!(
                                    self.parse_long_integer(positive, significand),
                                )));
                            }

                            self.eat_char();
                            significand = significand * 10 + digit;
                        }
                        _ => {
                            return self.parse_number(positive, significand);
                        }
                    }
                }
            }
            _ => Err(self.error(ErrorCode::InvalidNumber)),
        }
    }
    
    pub(crate) fn deserialize_number<'any, V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'any>,
    {
        let peek = match tri!(self.parse_whitespace()) {
            Some(b) => b,
            None => {
                return Err(self.peek_error(ErrorCode::EofWhileParsingValue));
            }
        };

        let value = match peek {
            b'-' => {
                self.eat_char();
                tri!(self.parse_integer(false)).visit(visitor)
            }
            b'0'..=b'9' => tri!(self.parse_integer(true)).visit(visitor),
            _ => Err(self.peek_invalid_type(&visitor)),
        };

        match value {
            Ok(value) => Ok(value),
            Err(err) => Err(self.fix_position(err)),
        }
    }
}
```

<br>

## Methods for deserialize numbers of `serde::de::Deserialize` trait
[**serde::de::Deserialize**](https://docs.rs/serde/latest/serde/de/trait.Deserialize.html).<br>

Implementations of `Deserialize` for **integer types** are generated by `impl_deserialize_num!`.<br>

For example, `impl<'de> Deserialize<'de> for u32`:
```rust
impl<'de> Deserialize<'de> for u32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> { }
}

impl_deserialize_num! {
    u32, NonZeroU32, deserialize_u32
    num_self!(u32:visit_u32);
    num_as_self!(u8:visit_u8 u16:visit_u16);
    int_to_uint!(i8:visit_i8 i16:visit_i16 i32:visit_i32 i64:visit_i64);
    uint_to_self!(u64:visit_u64);
}

macro_rules! impl_deserialize_num {
    ($primitive:ident, $deserialize:ident $($method:ident!($($val:ident : $visit:ident)*);)*) => {
        impl<'de> Deserialize<'de> for $primitive {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct PrimitiveVisitor;

                impl<'de> Visitor<'de> for PrimitiveVisitor {
                    type Value = $primitive;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(stringify!($primitive))
                    }

                    $($($method!($val : $visit);)*)*
                }

                deserializer.$deserialize(PrimitiveVisitor)
            }
        }
    };
}
```
