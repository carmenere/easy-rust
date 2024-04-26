# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)

<br>

# URLs
- [**NonNull<T>**](https://doc.rust-lang.org/std/ptr/struct.NonNull.html);

<br>

# Declaration
```rust
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}
```

<br>

# In a nutshell
The `NonNull<T>` type is like `*mut T` bus has following propeties:
1. Unlike `*mut T`, the `NonNull<T>` pointer **must always** be **non-null**;
2. `NonNull<T>` must be **covariant** over `T`;
3. It **implements** `!Send` and `!Sync`:
```rust
impl<T: ?Sized> !Send for NonNull<T> {}
impl<T: ?Sized> !Sync for NonNull<T> {}
```

<br>

If your type **cannot** safely be covariant, you must ensure it contains some additional field to provide **invariance**. Often this field will be a `PhantomData` type like `PhantomData<Cell<T>>` or `PhantomData<&'a mut T>`.


