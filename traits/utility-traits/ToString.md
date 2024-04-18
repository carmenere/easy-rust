# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Blanket implementations](#blanket-implementations)
  - [`impl<T> ToString for T `](#implt-tostring-for-t-)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`ToString`|[std::string::ToString](https://doc.rust-lang.org/std/string/trait.ToString.html)|

<br>

# Declaration
```rust
pub trait ToString {
    fn to_string(&self) -> String;
}
```

<br>

# In a nutshell
Converts the given value to a `String`.
`ToString` trait provides method `.to_string()` to convert **value** to a `String`.<br>
`ToString` trait is **automatically implemented** for any type that implements `Display`.<br>

<br>

# Blanket implementations
## `impl<T> ToString for T `
The standard library implements the `ToString` trait on **any type that implements** the `Display` trait:
```Rust
impl<T: fmt::Display + ?Sized> ToString for T {
    default fn to_string(&self) -> String {
        let mut buf = String::new();
        let mut formatter = core::fmt::Formatter::new(&mut buf);
        // Bypass format_args!() to avoid write_str with zero-length strs
        fmt::Display::fmt(self, &mut formatter)
            .expect("a Display implementation returned an error unexpectedly");
        buf
    }
}
```