# Contents
<!-- TOC -->
* [Contents](#contents)
* [Slice](#slice)
  * [**src/core/slice/mod.rs**](#srccoreslicemodrs)
    * [Methods of slice: iter() and iter_mut()](#methods-of-slice-iter-and-iter_mut)
  * [**src/core/slice/iter.rs**](#srccoresliceiterrs)
    * [Slice not Iterator](#slice-not-iterator)
    * [impl IntoIterator for slice](#impl-intoiterator-for-slice)
    * [Structs Iter/IterMut](#structs-iteritermut)
    * [impl Iterator for Iter/IterMut](#impl-iterator-for-iteritermut)
* [Traits](#traits)
  * [**src/core/iter/traits/iterator.rs**](#srccoreitertraitsiteratorrs)
    * [Iterator](#iterator)
  * [**src/core/iter/traits/collect.rs**](#srccoreitertraitscollectrs)
    * [IntoIterator](#intoiterator)
    * [Blanket impl for any Iterator](#blanket-impl-for-any-iterator)
    * [Trait FromIterator](#trait-fromiterator)
* [Vec](#vec)
  * [**src/alloc/vec/mod.rs**](#srcallocvecmodrs)
    * [Methods to get slice from Vec](#methods-to-get-slice-from-vec)
    * [impl Deref/DerefMut for Vec](#impl-derefderefmut-for-vec)
    * [impl FromIterator for Vec](#impl-fromiterator-for-vec)
    * [impl IntoIterator for Vec](#impl-intoiterator-for-vec)
  * [**src/alloc/vec/into_iter.rs**](#srcallocvecinto_iterrs)
    * [impl Send/Sync for IntoIter](#impl-sendsync-for-intoiter)
    * [Struct IntoIter](#struct-intoiter)
    * [impl Iterator for IntoIter](#impl-iterator-for-intoiter)
    * [impl AsRef for IntoIter](#impl-asref-for-intoiter)
<!-- TOC -->

<br>

# Slice
## [**src/core/slice/mod.rs**](https://doc.rust-lang.org/src/core/slice/mod.rs.html)
### Methods of slice: iter() and iter_mut()
```rust
#[cfg(not(test))]
impl<T> [T] {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }
}
```

<br>

## [**src/core/slice/iter.rs**](https://doc.rust-lang.org/src/core/slice/iter.rs.html)
### Slice not Iterator
```rust
impl<T> !Iterator for [T] {}
```

<br>

### impl IntoIterator for slice
```rust
impl<'a, T> IntoIterator for &'a [T] {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut [T] {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
```

<br>

### Structs Iter/IterMut
```rust
pub struct Iter<'a, T: 'a> {
    ptr: NonNull<T>,
    end_or_len: *const T,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T: 'a> {
    ptr: NonNull<T>,
    end_or_len: *mut T,
    _marker: PhantomData<&'a mut T>,
}
```

<br>

### impl Iterator for Iter/IterMut
```rust
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
}
```

<br>

# Traits
## [**src/core/iter/traits/iterator.rs**](https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html)
### Iterator
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
  
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
  }
}
```

## [**src/core/iter/traits/collect.rs**](https://doc.rust-lang.org/src/core/iter/traits/collect.rs.html)
### IntoIterator
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

### Blanket impl for any Iterator
```rust
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

### Trait FromIterator
```rust
pub trait FromIterator<A>: Sized {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
```

# Vec
## [**src/alloc/vec/mod.rs**](https://doc.rust-lang.org/src/alloc/vec/mod.rs.html)
### Methods to get slice from Vec
```rust
impl<T, A: Allocator> Vec<T, A> {
    pub const fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
}
```

<br>

### impl Deref/DerefMut for Vec
```rust
impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    type Target = [T];
    
    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, A: Allocator> ops::DerefMut for Vec<T, A> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
```

<br>

### impl FromIterator for Vec
SpecFromIter: https://doc.rust-lang.org/src/alloc/vec/spec_from_iter.rs.html
```rust
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;

  fn collect<B: FromIterator<Self::Item>>(self) -> B
  where Self: Sized,
  {
    FromIterator::from_iter(self)
    <B as FromIterator>::from_iter(self)
  }
}

pub trait FromIterator<A>: Sized {
  fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}

impl<T> FromIterator<T> for Vec<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Vec<T> {
    <Self as SpecFromIter<T, I::IntoIter>>::from_iter(iter.into_iter())
  }
}
```

<br>

### impl IntoIterator for Vec
```rust
impl<T, A: Allocator> IntoIterator for Vec<T, A> {
    type Item = T;
    type IntoIter = IntoIter<T, A>;

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

impl<'a, T, A: Allocator> IntoIterator for &'a mut Vec<T, A> {
    type Item = &'a mut T;
    type IntoIter = slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T, A: Allocator> IntoIterator for &'a Vec<T, A> {
    type Item = &'a T;
    type IntoIter = slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
```

<br>

## [**src/alloc/vec/into_iter.rs**](https://doc.rust-lang.org/src/alloc/vec/into_iter.rs.html)
### impl Send/Sync for IntoIter
```rust
unsafe impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A> {}
unsafe impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A> {}
```

<br>

### Struct IntoIter
```rust
pub struct IntoIter<T, A: Allocator = Global> {
    pub(super) buf: NonNull<T>,
    pub(super) phantom: PhantomData<T>,
    pub(super) cap: usize,
    pub(super) alloc: ManuallyDrop<A>,
    pub(super) ptr: NonNull<T>,
    pub(super) end: *const T,
}

impl<T, A: Allocator> IntoIter<T, A> {
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len()) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { &mut *self.as_raw_mut_slice() }
    }
}
```

<br>

### impl Iterator for IntoIter
```rust
impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {}
}
```

<br>

### impl AsRef for IntoIter
```rust
impl<T, A: Allocator> AsRef<[T]> for IntoIter<T, A> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}
```
