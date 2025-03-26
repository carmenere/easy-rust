# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declaration](#declaration)
* [In a nutshell](#in-a-nutshell)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Debug`|[std::fmt::Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html)|

<br>

# Declaration
```rust
pub type Result = result::Result<(), Error>;

pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

<br>

# In a nutshell
`Debug` and `Display` traits are part of the `std::fmt` module.<br>
The `Debug` formats the value using the **given formatter**. It is invoked by `{:?}` and `{:#?}` format specifiers.<br>
The `Debug` trait is **derivable** and can be used with `#[derive]` if **all** of the type’s fields implement `Debug`.

