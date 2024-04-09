# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [ToString](#tostring)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`ToString`|[std::string::ToString](https://doc.rust-lang.org/std/string/trait.ToString.html)|

<br>

# ToString
`ToString` trait provides method `.to_string()` to convert **value** to a `String`.<br>
`ToString` trait is **automatically implemented** for any type that implements `Display`.

The standard library implements the `ToString` trait on **any type that implements** the `Display` trait:
```Rust
impl<T> ToString for T 
where
    T: Display + ?Sized
```

Because the standard library has this **blanket implementations**, we can call the `to_string()` of `ToString` trait on **any type** that **implements** the `Display` trait.