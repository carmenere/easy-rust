# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declaration](#declaration)
* [In a nutshell](#in-a-nutshell)
* [Blanket implementations](#blanket-implementations)
    * [`impl Clone for [T; N]`](#impl-clone-for-t-n)
    * [`impl Clone for &T`](#impl-clone-for-t)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Clone`|[std::clone::Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)|

<br>

# Declaration
```rust
pub trait Clone: Sized {
    fn clone(&self) -> Self;

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}
```

<br>

# In a nutshell
Differs from `Copy` in that `Clone` is **always explicit** and **may** or **may not** be **expensive**, while `Copy` is **implicit** and an **inexpensive bit-wise copy**.
Unlike `ToOwned` trait the `Clone` works **only** for going **from** `&T` **to** `T`.<br>

<br>

# Blanket implementations
### `impl Clone for [T; N]`
```rust
impl<T: Clone, const N: usize> Clone for [T; N] {
   fn clone(&self) -> Self {
      SpecArrayClone::clone(self)
   }
}
```

<br>

### `impl Clone for &T`
```rust
impl<T: ?Sized> Clone for &T {
   fn clone(&self) -> Self {
      *self
   }
}
```