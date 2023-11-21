# Serde Data model
There are 2 main layers in serde:
- **data structures**;
- **data formats**;

<br>

Relationship between **data structures** and **data format**:
- A **data structure** can be serialized into/deserialized from any **data format** supported by Serde.
- A **data format** can serialize/deserialize any **data structure** supported by Serde.

<br>

**Data formats** implement traits:
- `Serializer`;
- `Deserializer`;

<br>

**Data structures** implement traits:
- `Serialize`;
- `Deserialize`;

<br>

When **serializing** a **data structure** to some format:
- the `Serialize` implementation for the **data structure** is responsible for **mapping** the **data structure** into the Serde **data model** by invoking exactly one of the `Serializer` methods;
- the `Serializer` implementation for the **data format** is responsible for **mapping** the Serde **data model** into the intended **output format**;

<br>

When **deserializing** a **data structure** from some format:
- the `Deserialize` implementation for the **data structure** is responsible for **mapping** the **data structure** into the Serde **data model** by passing to the `Deserializer` a `Visitor` - implementation that can receive the various types of the **data model**;
the `Deserializer` implementation for the **data format** is responsible for **mapping** the **input data** into the Serde **data model** by invoking exactly one of the `Visitor` methods;

<br>

The Serde **data model** is a simplified form of Rust's type system. It consists of the following **29** types.

<br>

# Using derive
Serde provides a **derive macro** to generate implementations of the `Serialize` and `Deserialize` traits for **data structures**.<br>
Add `serde = { version = "x.x", features = ["derive"] }` as a dependency in **Cargo.toml**.<br>

<br>

# Custom serialization
Serde allows full customization of the **serialization behavior** by manually implementing `Serialize` and `Deserialize` traits for your type.<br>

<br>

The `Serialize` trait looks like this:
```rust
pub trait Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
```

This method's job is to take your **type** `&self` and **map** it into the Serde **data model** by invoking exactly one of the methods on the given `Serializer`.<br>

<br>

As the simplest example, here is the builtin `Serialize impl` for the primitive `i32`:
```rust
impl Serialize for i32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(*self)
    }
}
```

<br>

Serde provides such impls for **all of Rust's primitive types** so you are not responsible for implementing them yourself.

<br>

# Custom deserialization
The `Deserialize` trait looks like this:
```rust
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
```

<br>

This method's job is to **map** the type into the Serde **data model** by providing the `Deserializer` with a `Visitor`.<br>
The `Deserializer` will call one of the `Visitor` methods **depending on the input data**, which is known as **driving** the `Visitor`:
```rust
impl<'de> Deserialize<'de> for i32 {
    fn deserialize<D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(I32Visitor)
    }
}
```

Example of `Visitor` trait that is able to deserialize a primitive i32 from a variety of types:
```rust
use std::fmt;

use serde::de::{self, Visitor};

struct I32Visitor;

impl<'de> Visitor<'de> for I32Visitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(i32::from(value))
    }

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
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(value as i32)
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }

    // Similar for other methods:
    //   - visit_i16
    //   - visit_u8
    //   - visit_u16
    //   - visit_u32
    //   - visit_u64
}
```

<br>

The `Visitor` trait has lots more methods that are not implemented for `I32Visitor`.<br>
Leaving them **unimplemented** means a **type error** is returned if they get called.<br>
For example `I32Visitor` does not implement `Visitor::visit_map`, so trying to deserialize an `i32` when the input contains a map is a **type error**.

<br>

# Links
- [Implementing Serialize](https://serde.rs/impl-serialize.html)
- [Implementing Deserialize](https://serde.rs/impl-deserialize.html)
