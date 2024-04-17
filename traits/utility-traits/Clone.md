# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [In a nutshell](#in-a-nutshell)
- [Declaration](#declaration)
- [Blanket implementations](#blanket-implementations)
    - [`impl<T, N> Clone for [T; N]`](#implt-n-clone-for-t-n)
    - [`impl<T> Clone for &T`](#implt-clone-for-t)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Clone`|[std::clone::Clone](https://doc.rust-lang.org/std/clone/trait.Clone.html)|

<br>

# In a nutshell
Differs from `Copy` in that `Copy` is **implicit** and an **inexpensive bit-wise copy**, while `Clone` is **always explicit** and **may** or **may not** be **expensive**.<br>
Unlike `ToOwned` trait the `Clone` works **only** for going **from** `&T` **to** `T`.<br>

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

# Blanket implementations
### `impl<T, N> Clone for [T; N]`
```rust
impl<T: Clone, const N: usize> Clone for [T; N] {
   fn clone(&self) -> Self {
      SpecArrayClone::clone(self)
   }
}
```

<br>

### `impl<T> Clone for &T`
```rust
impl<T: ?Sized> Clone for &T {
   fn clone(&self) -> Self {
      *self
   }
}
```