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

# Examples
## Visitor pattern for deserialization
### `Visitor`
```rust
trait Visitor {
    type Value;
    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M:: Error>
    where
        M: MapAccess;
}
```

<br>

### `Deserializer`
```rust
trait Deserializer {
    type Error;
    fn deserialize_struct<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor;
}
```

<br>

### Custom type `Point`
```rust
struct Point {
    x: i32,
    y: i32,
}
```

<br>

### Main idea
- `Point::deserialize()` is defined by you.
- `JsonDeserializer::deserialize_struct()` defined by another module to deserialize JSON to the `MapAccess` object.
- `Point::deserialize()` calls `JsonDeserializer::deserialize_struct()` which then calls `PointVisitor::visit_map()` to **finally deserialize** the `MapAccess` object to `Point`.

<br>

This way the `JsonDeserializer` type doesn’t need to know how to instantiate the type being deserialized (here it’s `Point`).<br>
The `Point` type also doesn’t need to know the specifics of the format it is deserialized from. It only has to know how to deserialize from the intermediate format (in this case the `MapAccess` object).<br>

<br>

### impl `Deserialize` for custom type `Point`
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

### impl `Visitor` for custom visitor
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

### `JsonDeserializer`
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