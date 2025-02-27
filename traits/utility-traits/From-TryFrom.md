# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Trait `From`](#trait-from)
  - [Declaration](#declaration)
  - [In a nutshell](#in-a-nutshell)
  - [Example](#example)
- [Trait `TryFrom`](#trait-tryfrom)
  - [Declaration](#declaration-1)
  - [In a nutshell](#in-a-nutshell-1)
  - [Example](#example-1)
- [Blanket implementations](#blanket-implementations)
  - [Every type can be converted to itself](#every-type-can-be-converted-to-itself)
  - [Relation with trait `Into`](#relation-with-trait-into)
    - [Example](#example-2)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`From`|[std::convert::From](https://doc.rust-lang.org/std/convert/trait.From.html)|
|`TryFrom`|[std::convert::TryFrom](https://doc.rust-lang.org/std/convert/trait.TryFrom.html)|

<br>

# Trait `From`
## Declaration
```rust
pub trait From<S> {
    fn from(value: S) -> Self;
}
```

- `Self` implies **destination** type `D`;
- method `from()` performs the conversion;

<br>

## In a nutshell
Trait `From` is used to convert value *from* **source** type `S` *to* **destination** type `D` on which it is implemented.<br>
Trait `From` **must not fail**. If the conversion **can** **fail**, use `TryFrom`.<br>
The `From` is useful for **error handling**.<br>

<br>

## Example
**Notes**:<br>
- The compiler is **unable** to **infer** destination type `D` when `From::from()` is used.
- Explicit type declaration must be used in **let binding**, for example: `let n: Number = From::from(30);`.

<br>

```rust
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let n1 = Number::from(10);
    let n2: Number = From::from(30);
    println!("num: {:?}, {:?}", n1, n2);
}
```

<br>

# Trait `TryFrom`
## Declaration
```Rust
pub trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

<br>

## In a nutshell
`TryFrom<T>` returns `Result<T, E>`.<br>

<br>

## Example
```Rust
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}
```

<br>

# Blanket implementations
## Every type can be converted to itself
```Rust
impl<T> const From<T> for T {
    /// Returns the argument unchanged.
    #[inline(always)]
    fn from(t: T) -> T {
        t
    }
}
```

<br>

## Relation with trait `Into`
There is **blanket implementation** for trait `Into`:
```Rust
impl<S, D> const Into<D> for S
where
    D: ~const From<S>,
{
    fn into(self) -> D {
        D::from(self)
    }
}
```

In other words, implementing `impl From<S> for D` **automatically** implements `impl Into<D> for S`, **but not vise versa**.<br>

So, **this**
```Rust
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}
```
will implicitly creates **this**
```Rust
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number::from(self)
    }
}
```

<br>

### Example
**Notes**:<br>
- The compiler is **unable** to **infer** destination type `D` when `.into()` is used.
- Explicit type declaration must be used in **let binding**, for example: `let n: Number = 5.into();`.

<br>

```rust
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let n: Number = 5.into();
    println!("num: {:?}", n);
}
```
