# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [`NonNull<T>`](#nonnullt)
  * [Declaration](#declaration)
  * [In a nutshell](#in-a-nutshell)
* [`Unique<T>`](#uniquet)
  * [Declaration](#declaration-1)
  * [In a nutshell](#in-a-nutshell-1)
* [Null pointer optimization](#null-pointer-optimization)
  * [URLs](#urls-1)
* [In a nutshell](#in-a-nutshell-2)
<!-- TOC -->

<br>

# URLs
- [**std::ptr::NonNull**](https://doc.rust-lang.org/std/ptr/struct.NonNull.html);
- [**Unique<T>**](https://doc.rust-lang.org/src/core/ptr/unique.rs.html);

<br>

# `NonNull<T>`
## Declaration
```rust
#[repr(transparent)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<T: ?Sized> {
    pointer: *const T,
}
```

<br>

**Note**, that `#[repr(transparent)]` and `#[rustc_nonnull_optimization_guaranteed]` attributes are used for `NonNull`.<br>

<br>

## In a nutshell
The `std::ptr::NonNull<T>` type is like `*mut T` but with the **added invariants**:
1. `NonNull<T>` pointer **cannot** be **non-null**;
2. `NonNull<T>` is **covariant** over `T`;
3. `NonNull<T>` **implements** `!Send` and `!Sync`:
```rust
impl<T: ?Sized> !Send for NonNull<T> {}
impl<T: ?Sized> !Sync for NonNull<T> {}
```

<br>

If your type **cannot** safely be covariant, you must ensure it contains some additional field to provide **invariance**.<br>
Often this field will be a `PhantomData` type like `PhantomData<Cell<T>>` or `PhantomData<&'a mut T>`.<br>


<br>

# `Unique<T>`
## Declaration
```rust
#[repr(transparent)]
pub struct Unique<T: ?Sized> {
    pointer: NonNull<T>,
    _marker: PhantomData<T>,
}
```

<br>

**Note**, that `#[repr(transparent)]` attribute is used for `NonNull`.<br>

<br>

## In a nutshell
The `Unique<T>` type is like `NonNull<T>` with the only exception it **implements** `Send` and `Sync` if type `T` does:
```rust
unsafe impl<T: Send + ?Sized> Send for Unique<T> {}
unsafe impl<T: Sync + ?Sized> Sync for Unique<T> {}
```

<br>

# Null pointer optimization
## URLs
- [**Null pointer optimization**](https://doc.rust-lang.org/std/option/index.html#representation);
- [**std::mem::size_of**](https://doc.rust-lang.org/std/mem/fn.size_of.html);
- [**std::mem::align_of**](https://doc.rust-lang.org/std/mem/fn.align_of.html);
- [**ABI compatibility**](https://doc.rust-lang.org/std/primitive.fn.html#abi-compatibility);

<br>

# In a nutshell
Rust guarantees that that:
- `Option<T>` is **exactly** the **same** *size*, *alignment* and *function call ABI* as `T`;
- `Option<&T>` is **exactly** the **same** *size*, *alignment* and *function call ABI* as `&T`;

This is called the **null pointer optimization** or **NPO**.<br>

In other words, Rust guarantees that:
- `size_of::<T>()` **equals** `size_of::<Option<T>>()`;
- `align_of::<T>()` **equals** `align_of::<Option<T>>()`;

<br>