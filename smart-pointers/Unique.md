# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)

<br>

# URLs
- [**Unique<T>**](https://doc.rust-lang.org/src/core/ptr/unique.rs.html);

<br>

# Declaration
```rust
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    _marker: PhantomData<T>,
}
```

<br>

# In a nutshell
The `Unique<T>` type is like `NonNull<T>` with the only exception it **implements** `Send` and `Sync` if type `T` does:
```rust
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}
```
