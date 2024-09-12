# Serde data model
Serde consists of 3 layers:
1. **Data formats** like JSON, Toml, XML, and so on.
2. **Rust data types**.
3. **Serde types** intermediate types that provide mapping between **Data formats** and **Rust data types**.

<br>

In serde the *data formats* are **completely decoupled** from the *deserialization*:
- *Rust data types* don't know about *data formats*;
- *data formats* don't know about *Rust data types*;

This is achieved through **visitor pattern** (aka **double dispatch**).<br>

The `deserialize()` method takes in a `Deserializer`, which is responsible for converting the **input format** into some kind of **intermediate format** (**serde type**).<br>
It needs this **intermediate format**, because a generic `Deserializer` cannot possibly know to instantiate every struct type.<br>
It takes a `visitor` which implements **deserialization** from the **intermediate format** into the **specific type** and it calls **appropriate method** for this `visitor`, passing it a `value` of **intermediate type**.<br>

Example:
- the `deserializer.deserialize_xxx()` calls appropriate `visitor`'s method `visitor.visit_xxx()`:
```Rust
deserializer.deserialize_xxx()
                |-> visitor.visit_xxx()
```

<br>

# Example1: `MyDeserializer` for `bool`
## Overview
```rust
use serde::de;

struct MyDeserializer;

impl<'de, 'a> de::Deserializer<'de> for &'a mut MyDeserializer<'de> {
	...
}

pub fn from_str<T: Deserialize>(input: &str) -> Result<T, Error> {
	let deserializer = MyDeserializer::new();
	T::deserialize(deserializer)?
}
```

<br>

## impl `de::Error`
```rust
use serde::de;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeserializeError {
    #[error("Failed to parse: {0}")]
    Parse(String),
    #[error("Unsupported: {0}")]
    Unsupported(String),
    #[error("Error: {0}")]
    Message(String),
}

impl de::Error for DeserializeError {
    fn custom<T: Display>(msg: T) -> Self {
        DeserializeError::Message(msg.to_string())
    }
}
```

<br>

## impl `de::Visitor`
```rust
struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for bool {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bool(BoolVisitor)
    }
}
```

<br>

## impl `de::Deserializer`
```rust
use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any,
};

pub struct BoolDeserializer<'de> {
    input: &'de str,
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut BoolDeserializer<'de> {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(DeserializeError::Unsupported(
            "Unsupported type".to_string(),
        ))
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        if self.input == "true" {
            visitor.visit_bool(true)
        } else if self.input == "false" {
            visitor.visit_bool(false)
        } else {
            Err(DeserializeError::Parse("Invalid boolean value".to_string()))
        }
    }

    forward_to_deserialize_any! {
        i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
```

<br>

## Tests
```rust
fn from_str<'de, T: Deserialize<'de>>(input: &'de str) -> Result<T, DeserializeError> {
    let mut deserializer = BoolDeserializer { input };
    T::deserialize(&mut deserializer)
}

#[cfg(test)]
mod test {
    #[test]
    fn deserialize_true() {
        let value: bool = super::from_str("true").unwrap();
        assert!(value);
    }

    #[test]
    fn deserialize_error() {
        let value: String = super::from_str("true").unwrap();
        assert_eq!(value, "true");
    }
}
```

<br>

# Example2: `MyDeserializer` for `NewType(bool)`
```rust
#[test]
fn deserialize_newtype() {
    use serde::de::{Error, Visitor};
    use std::fmt;

    #[derive(Debug, PartialEq)]
    struct NewType(bool);
    struct NewTypeVisitor;

    impl<'de> Visitor<'de> for NewTypeVisitor {
        type Value = NewType;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(NewType(v))
        }
    }

    impl<'de> Deserialize<'de> for NewType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bool(NewTypeVisitor)
        }
    }

    let value: NewType = super::from_str("true").unwrap();
    assert_eq!(value, NewType(true));
}
```

<br>

# Example3: TwitterId
```rust
use std::fmt;
use std::str::FromStr;
use std::convert::TryFrom;
use serde::de::{ Deserialize, Deserializer, Visitor, Error };

struct I64Visitor;

impl Visitor<'_> for I64Visitor {
    type Value = i64;
    
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("numeric string or i64")
    }
    
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }
    
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        i64::try_from(v).map_err(E::custom)
    }
    
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        i64::from_str(v).map_err(|err| {
            E::custom(format_args!(
                "cannot decode i64 out of `{}`: {}", v, err
            ))
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TwitterId(pub i64);

impl<'de> Deserialize<'de> for TwitterId {
    fn deserialize<D>(deserializer: D) -> Result<TwitterId, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(I64Visitor).map(TwitterId)
    }
}

fn main() -> anyhow::Result<()> {
    let twitter_id: TwitterId = serde_json::from_str(r#"10972114391"#)?;
    println!("{:?}", twitter_id);
    Ok(())
}
```

<br>

# Example4: Custom type `Point`
## `Visitor`
```rust
trait Visitor {
    type Value;
    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M:: Error>
    where
        M: MapAccess;
}
```

<br>

## `Deserializer`
```rust
trait Deserializer {
    type Error;
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor;
}
```

<br>

## Custom type `Point`
```rust
struct Point {
    x: i32,
    y: i32,
}
```

<br>

## Main idea
- `Point::deserialize()` is defined by you.
- `JsonDeserializer::deserialize_struct()` defined by another module to deserialize JSON to the `MapAccess` object.
- `Point::deserialize()` calls `JsonDeserializer::deserialize_struct()` which then calls `PointVisitor::visit_map()` to **finally deserialize** the `MapAccess` object to `Point`.

<br>

This way the `JsonDeserializer` type doesn’t need to know how to instantiate the type being deserialized (here it’s `Point`).<br>
The `Point` type also doesn’t need to know the specifics of the format it is deserialized from. It only has to know how to deserialize from the intermediate format (in this case the `MapAccess` object).<br>

<br>

## impl `Deserialize` for custom type `Point`
```Rust
impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct(PointVisitor)
    }
}
```

<br>

## impl `Visitor` for custom visitor
```Rust
struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn visit_map<V>(self, mut map: V) -> Result<Point, V::Error>
    where
        V: MapAccess<'de>,
    {
        let x;
        let y;

        // Extract x and y from the map
        // ...

        Ok(Point::new(x, y))
    }
}
```

<br>

## `JsonDeserializer`
```rust
struct JsonDeserializer {
    //...
}

struct JsonError;

impl Deserializer for JsonDeserializer {
    type Error = JsonError;
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where 
        V: Visitor
    {
        //imagine there is a method to get a map access object on the JsonDeserializer
        let map = self.parse_map().map_err(|_| JsonError)?;

        visitor.visit_map(map).map_err(|_| JsonError)
    }
}
```

<br>