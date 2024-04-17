# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)
- [Declaration](#declaration)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Copy`|[std::marker::Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)|

<br>

# In a nutshell
Differs from `Copy` in that `Copy` is **implicit** and an **inexpensive bit-wise copy**, while `Clone` is **always explicit** and **may** or **may not** be **expensive**.

<br>

# Declaration
```rust
pub trait Copy: Clone { }
```