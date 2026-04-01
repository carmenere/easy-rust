# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Example](#example)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Default`|[std::default::Default](https://doc.rust-lang.org/std/default/trait.Default.html)|

<br>

# Declaration
```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```

<br>

# In a nutshell
Returns the **default value** for a **type**. **Default values** are often some kind of **initial value**, in other words, anything else that may **make sense as a default**.<br>

The `Default` trait is **derivable** and can be used with `#[derive]` if **all** of the type’s fields implement `Default`:
- when **derived**, it will use the **default value** for **each** field’s type;
- when using `#[derive(Default)]` on an `enum`, you need to choose which variant will be default. You do this by placing the `#[default]` attribute on the variant:
```rust
#[derive(Default)]
enum Kind {
    #[default]
    A,
    B,
    C,
}
```

<br>

# Example
Most frequently used types in the Rust standard library implement `Default` trait.<br>
The `Default::default()` is like a `new()` method that can’t take any arguments.<br>

```rust
fn main() {
    let default_i8: i8 = Default::default();
    let default_str: String = Default::default();
    let default_bool: bool = Default::default();
    println!("default_i8={default_i8}\ndefault_str={default_str}\ndefault_bool={default_bool}");
}
```

**Output**:
```rust
default_i8=0
default_str=
default_bool=false
```

So, **default value** is a some kind of **initial value**.<br>

<br>

> **Note**, to implement `Default` using `#[derive(Default)]`, **all** of a type’s parameters **must** implement `Default`.<br>

<br>

You can pick a **default variant** for an `enum`, as long as it is a **unit enum variant** (**has no data** in it) by using the #[derive(Default)] attribute **on top** and then `#[default]` **over** the *default variant*:
Default for enum:
```rust
#[derive(Default)]
enum Operation {
    #[default]
    Add,
    Subtract,
}
```
