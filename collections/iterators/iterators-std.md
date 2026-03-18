# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Crate `std`](#crate-std)
  - [**`std/src/lib.rs`**](#stdsrclibrs)
- [Crate `alloc`](#crate-alloc)
  - [**`alloc/src/lib.rs`**](#allocsrclibrs)
  - [**`alloc/src/slice.rs`**](#allocsrcslicers)
  - [**`alloc/src/vec/mod.rs`**](#allocsrcvecmodrs)
  - [**`alloc/src/vec/into_iter.rs`**](#allocsrcvecinto_iterrs)
- [Crate `core`](#crate-core)
  - [**`core/src/lib.rs`**](#coresrclibrs)
  - [**`core/src/slice/mod.rs`**](#coresrcslicemodrs)
  - [**`core/src/slice/iter.rs`**](#coresrcsliceiterrs)
  - [**`core/src/iter/mod.rs`**](#coresrcitermodrs)
  - [**`core/src/iter/traits/iterator.rs`**](#coresrcitertraitsiteratorrs)
  - [**`core/src/iter/traits/collect.rs`**](#coresrcitertraitscollectrs)
  - [**`core/src/array/mod.rs`**](#coresrcarraymodrs)
  - [**`core/src/array/iter.rs`**](#coresrcarrayiterrs)
<!-- TOC -->

<br>

# Crate `std`
## [**`std/src/lib.rs`**](https://doc.rust-lang.org/stable/src/std/lib.rs.html)
```rust
pub use core::array; // --> core/src/array/mod.rs

extern crate alloc as alloc_crate; // -> alloc/src/lib.rs

pub use alloc_crate::slice; // --> alloc/src/slice.rs
pub use alloc_crate::vec; // --> alloc/src/vec/mod.rs
pub use alloc_crate::str; // --> alloc/src/str.rs
pub use alloc_crate::string; // --> alloc/src/string.rs
```

<br>

# Crate `alloc`
## [**`alloc/src/lib.rs`**](https://doc.rust-lang.org/stable/src/alloc/lib.rs.html)
```rust
pub mod slice; // --> alloc/src/slice.rs
pub mod vec; // --> alloc/src/vec/mod.rs
```

<br>

## [**`alloc/src/slice.rs`**](https://doc.rust-lang.org/stable/src/alloc/slice.rs.html)
```rust
pub use core::slice::{Iter, IterMut}; // --> core/src/slice/mod.rs - {Iter, IterMut}
use crate::vec::Vec; // --> alloc/src/vec/mod.rs - {Vec}

impl<T> [T] {
    // Additional methods
}
```

<br>

## [**`alloc/src/vec/mod.rs`**](https://doc.rust-lang.org/stable/src/alloc/vec/mod.rs.html)
```rust
mod into_iter; // --> alloc/src/vec/into_iter.rs

use core::iter; // --> core/src/iter/mod.rs
use core::slice::{self, SliceIndex}; // --> core/src/slice/mod.rs
pub use self::into_iter::IntoIter; // --> alloc/src/vec/into_iter.rs {IntoIter}

// impl IntoIterator for Vec<T>
impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the vector (from start to end). The vector cannot be used after calling
    /// this.
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let me = ManuallyDrop::new(self);
            let alloc = ManuallyDrop::new(ptr::read(me.allocator()));
            let buf = me.buf.non_null();
            let begin = buf.as_ptr();
            let end = if T::IS_ZST {
                begin.wrapping_byte_add(me.len())
            } else {
                begin.add(me.len()) as *const T
            };
            let cap = me.buf.capacity();
            IntoIter { buf, phantom: PhantomData, cap, alloc, ptr: buf, end }
        }
    }
}

// impl IntoIterator for &Vec<T>
impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter() // --> core/src/slice/mod.rs --> actualy calls impl<T> [T] { .iter(&self) }
    }
}

// impl IntoIterator for &mut Vec<T>
impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut() // --> core/src/slice/mod.rs --> actualy calls impl<T> [T] { .iter_mut(&mut self) }
    }
}

// impl FromIterator for Vec<T>
impl<T> FromIterator<T> for Vec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
        <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
    }
}

// impl AsRef for Vec<T>
impl<T, A: Allocator> AsRef<[T]> for Vec<T, A> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

// impl AsMut for Vec<T>
impl<T, A: Allocator> AsMut<[T]> for Vec<T, A> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}
```

<br>

## [**`alloc/src/vec/into_iter.rs`**](https://doc.rust-lang.org/stable/src/alloc/vec/into_iter.rs.html)
```rust
pub struct IntoIter<T, A: Allocator = Global> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    
    // the drop impl reconstructs a RawVec from buf, cap and alloc
    // to avoid dropping the allocator twice we need to wrap it into ManuallyDrop
    pub(super) alloc: ManuallyDrop<A>,

    pub(super) ptr: NonNull<T>,

    /// If T is a ZST, this is actually ptr+len. This encoding is picked so that
    /// ptr == end is a quick test for the Iterator being empty, that works
    /// for both ZST and non-ZST.
    /// For non-ZSTs the pointer is treated as `NonNull<T>`
    pub(super) end: *const T,
}

// impl Iterator for IntoIter
impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        let ptr = if T::IS_ZST {
            if self.ptr.as_ptr() == self.end as *mut T {
                return None;
            }
            // `ptr` has to stay where it is to remain aligned, so we reduce the length by 1 by
            // reducing the `end`.
            self.end = self.end.wrapping_byte_sub(1);
            self.ptr
        } else {
            if self.ptr == non_null!(self.end, T) {
                return None;
            }
            let old = self.ptr;
            self.ptr = unsafe { old.add(1) };
            old
        };
        Some(unsafe { ptr.read() })
    }
}

unsafe impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A> {}
unsafe impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A> {}

// impl AsRef for IntoIter
impl<T, A: Allocator> AsRef<[T]> for IntoIter<T, A> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
```

<br>

# Crate `core`
## [**`core/src/lib.rs`**](https://doc.rust-lang.org/stable/src/core/lib.rs.html)
```rust
pub mod fmt;
pub mod str;

pub mod alloc; // --> core/src/alloc/mod.rs
pub mod array; // --> core/src/array/mod.rs
pub mod slice; // --> core/src/slice/mod.rs
```

<br>

## [**`core/src/slice/mod.rs`**](https://doc.rust-lang.org/stable/src/core/slice/mod.rs.html)
```rust
mod iter; // --> core/src/slice/iter.rs
pub use iter::{Iter, IterMut}; // --> core/src/slice/iter.rs

impl<T> [T] {
    // ...
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }
    pub const fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }
    // ...
}
```

<br>

## [**`core/src/slice/iter.rs`**](https://doc.rust-lang.org/stable/src/core/slice/iter.rs.html)
```rust
pub struct Iter<'a, T: 'a> {
    /// The pointer to the next element to return, or the past-the-end location
    /// if the iterator is empty.
    /// This address will be used for all ZST elements, never changed.
    ptr: NonNull<T>,
    /// For non-ZSTs, the non-null pointer to the past-the-end element.
    /// For ZSTs, this is `ptr::without_provenance_mut(len)`.
    end_or_len: *const T,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T: 'a> {
    /// The pointer to the next element to return, or the past-the-end location
    /// if the iterator is empty.
    /// This address will be used for all ZST elements, never changed.
    ptr: NonNull<T>,
    /// For non-ZSTs, the non-null pointer to the past-the-end element.
    /// For ZSTs, this is `ptr::without_provenance_mut(len)`.
    end_or_len: *mut T,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iter<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a [T]) -> Self {
        let len = slice.len();
        let ptr: NonNull<T> = NonNull::from_ref(slice).cast();
        // SAFETY: Similar to `IterMut::new`.
        unsafe {
            let end_or_len =
                if T::IS_ZST { without_provenance(len) } else { ptr.as_ptr().add(len) };

            Self { ptr, end_or_len, _marker: PhantomData }
        }
    }
}

impl<'a, T> IterMut<'a, T> {
    #[inline]
    pub(super) const fn new(slice: &'a mut [T]) -> Self {
        let len = slice.len();
        let ptr: NonNull<T> = NonNull::from_mut(slice).cast();
        // SAFETY: There are several things here:
        //
        // `ptr` has been obtained by `slice.as_ptr()` where `slice` is a valid
        // reference thus it is non-NUL and safe to use and pass to
        // `NonNull::new_unchecked` .
        //
        // Adding `slice.len()` to the starting pointer gives a pointer
        // at the end of `slice`. `end` will never be dereferenced, only checked
        // for direct pointer equality with `ptr` to check if the iterator is
        // done.
        //
        // In the case of a ZST, the end pointer is just the length.  It's never
        // used as a pointer at all, and thus it's fine to have no provenance.
        //
        // See the `next_unchecked!` and `is_empty!` macros as well as the
        // `post_inc_start` method for more information.
        unsafe {
            let end_or_len =
                if T::IS_ZST { without_provenance_mut(len) } else { ptr.as_ptr().add(len) };

            Self { ptr, end_or_len, _marker: PhantomData }
        }
    }
}

// Sync and Send
unsafe impl<T: Sync> Sync for Iter<'_, T> {}
unsafe impl<T: Sync> Send for Iter<'_, T> {}

unsafe impl<T: Sync> Sync for IterMut<'_, T> {}
unsafe impl<T: Send> Send for IterMut<'_, T> {}

// Slice is not Iterator
impl<T> !Iterator for [T] {}

// impl IntoIterator for &[T]
impl<'a, T> IntoIterator for &'a [T] {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

// impl IntoIterator for &mut [T]
impl<'a, T> IntoIterator for &'a mut [T] {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

// impl Iterator: indirectly through macro iterator!

// impl Iterator for Iter
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}

// impl Iterator for IterMut
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}

// impl AsRef for Iter
impl<T> AsRef<[T]> for Iter<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

// impl AsRef for IterMut
impl<T> AsRef<[T]> for IterMut<'_, T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
```

<br>

## [**`core/src/iter/mod.rs`**](https://doc.rust-lang.org/stable/src/core/iter/mod.rs.html)
```rust
pub use self::traits::Iterator;
pub use self::traits::{FromIterator, IntoIterator, Product, Sum, DoubleEndedIterator, Extend};

mod adapters; // --> core/src/iter/adapters/mod.rs
mod traits; // --> core/src/iter/traits/mod.rs
```

<br>

## [**`core/src/iter/traits/iterator.rs`**](https://doc.rust-lang.org/stable/src/core/iter/traits/iterator.rs.html)
```rust
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    /// Use [`FromIterator::from_iter()`] as a more readable alternative to [`Iterator::collect()`]
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        // This is too aggressive to turn on for everything all the time, but PR#137908
        // accidentally noticed that some rustc iterators had malformed `size_hint`s,
        // so this will help catch such things in debug-assertions-std runners,
        // even if users won't actually ever see it.
        if cfg!(debug_assertions) {
            let hint = self.size_hint();
            assert!(hint.1.is_none_or(|high| high >= hint.0), "Malformed size_hint {hint:?}");
        }

        FromIterator::from_iter(self)
    }

}
```

## [**`core/src/iter/traits/collect.rs`**](https://doc.rust-lang.org/stable/src/core/iter/traits/collect.rs.html)
```rust
pub trait IntoIterator {
    /// The type of the elements being iterated over.
    type Item;
    /// Which kind of iterator are we turning this into?
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

pub trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

// Blanket impl IntoIterator for any Iterator
impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    #[inline]
    fn into_iter(self) -> I {
        self
    }
}
```

<br>

## [**`core/src/array/mod.rs`**](https://doc.rust-lang.org/stable/src/core/array/mod.rs.html)
```rust
use crate::slice::{Iter, IterMut}; // --> core/src/slice/mod.rs

impl<T, const N: usize> [T; N] {
    
}

// impl IntoIterator for &[T; N]
impl<'a, T, const N: usize> IntoIterator for &'a [T; N] {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

// impl IntoIterator for &mut [T; N]
impl<'a, T, const N: usize> IntoIterator for &'a mut [T; N] {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}

// impl AsRef for [T; N]
impl<T, const N: usize> const AsRef<[T]> for [T; N] {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self[..]
    }
}

// impl AsMut for [T; N]
impl<T, const N: usize> const AsMut<[T]> for [T; N] {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self[..]
    }
}
```

<br>

## [**`core/src/array/iter.rs`**](https://doc.rust-lang.org/stable/src/core/array/iter.rs.html)
```rust
pub struct IntoIter<T, const N: usize> {
    inner: ManuallyDrop<InnerSized<T, N>>,
}

impl<T, const N: usize> IntoIter<T, N> {
    /// Creates a new iterator over the given `array`.
    pub fn new(array: [T; N]) -> Self {
        IntoIterator::into_iter(array)
    }
}

// impl IntoIterator for [T; N]
impl<T, const N: usize> IntoIterator for [T; N] {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the array (from start to end).
    ///
    /// The array cannot be used after calling this unless `T` implements
    /// `Copy`, so the whole array is copied.
    ///
    /// Arrays have special behavior when calling `.into_iter()` prior to the
    /// 2021 edition -- see the [array] Editions section for more information.
    ///
    /// [array]: prim@array
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        // SAFETY: The transmute here is actually safe. The docs of `MaybeUninit`
        // promise:
        //
        // > `MaybeUninit<T>` is guaranteed to have the same size and alignment
        // > as `T`.
        //
        // The docs even show a transmute from an array of `MaybeUninit<T>` to
        // an array of `T`.
        //
        // With that, this initialization satisfies the invariants.
        //
        // FIXME: If normal `transmute` ever gets smart enough to allow this
        // directly, use it instead of `transmute_unchecked`.
        let data: [MaybeUninit<T>; N] = unsafe { transmute_unchecked(self) };
        // SAFETY: The original array was entirely initialized and the the alive
        // range we're passing here represents that fact.
        let inner = unsafe { InnerSized::new_unchecked(IndexRange::zero_to(N), data) };
        IntoIter { inner: ManuallyDrop::new(inner) }
    }

// impl Iterator for IntoIter
impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.unsize_mut().next()
    }
}

// Auto Trait Implementations
impl<T: Send, const N: usize> Send for IntoIter<T, N> {}
impl<T: Sync, const N: usize> Sync for IntoIter<T, N> {}
}
```
