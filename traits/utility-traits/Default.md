# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)
- [Declaration](#declaration)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Default`|[std::default::Default](https://doc.rust-lang.org/std/default/trait.Default.html)|

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

# Declaration
```rust
pub trait Default: Sized {
    fn default() -> Self;
}
```