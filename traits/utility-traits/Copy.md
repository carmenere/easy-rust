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
|`Copy`|[std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)|

<br>

# Declaration
```rust
pub trait Copy: Clone { }
```

<br>

# In a nutshell
The `Copy` trait is the **marker trait**.<br>
Differs from `Clone` in that `Copy` is **implicit** and an **inexpensive bit-wise copy**, while `Clone` is **always explicit** and **may** or **may not** be **expensive**.
