# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Trait `Into`](#trait-into)
  - [Declaration](#declaration)
  - [In a nutshell](#in-a-nutshell)
  - [Example](#example)
- [Trait `TryInto`](#trait-tryinto)
  - [Declaration](#declaration-1)
  - [In a nutshell](#in-a-nutshell-1)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Into`|[std::convert::Into](https://doc.rust-lang.org/std/convert/trait.Into.html)|
|`TryInto`|[std::convert::TryInto](https://doc.rust-lang.org/std/convert/trait.TryInto.html)|

<br>

# Trait `Into`
## Declaration
```rust
pub trait Into<D> {
    fn into(self) -> D;
}
```

- `Self` implies **source** type `S`;
- method `into()` performs the conversion;

<br>

## In a nutshell
Trait `Into` is used to convert value *from* **source** type `S` on which it is implemented *to* **destination** type `D`.<br>
Trait `Into` **must** **not fail**. If the conversion **can fail**, use `TryInto`.<br>

<br>

**Notes**:<br>
- The trait `Into` **doesn't** automatically implements `From` (as `From` does). Therefore, you should always try to implement `From` and then fall back to `Into` if `From` can’t be implemented.
- Prior to Rust **1.41**, if the *destination type* was **not** part of the **current** crate then you **couldn’t** implement `From` **directly**.

<br>

## Example
For example, the code below will **fail** in Rust prior **1.41** version:
```Rust
struct Wrapper<T>(Vec<T>);
impl<T> From<Wrapper<T>> for Vec<T> {
    fn from(w: Wrapper<T>) -> Vec<T> {
        w.0
    }
}
```

<br>

To bypass this, you could implement `Into` **directly**:
```Rust
struct Wrapper<T>(Vec<T>);
impl<T> Into<Vec<T>> for Wrapper<T> {
    fn into(self) -> Vec<T> {
        self.0
    }
}
```

<br>

# Trait `TryInto`
## Declaration
```Rust
pub trait TryInto<T> {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

<br>

## In a nutshell
`TryInto<T>` returns `Result<T, E>`.<br>
