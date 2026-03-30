# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Iterators and iterables design in Rust](#iterators-and-iterables-design-in-rust)
  - [Iterables](#iterables)
  - [Iterators](#iterators)
- [Diagrams](#diagrams)
  - [Vec::into\_iterator](#vecinto_iterator)
  - [Vec::iter and Vec::iter\_mut](#veciter-and-veciter_mut)
<!-- TOC -->

<br>

# Iterators and iterables design in Rust
**Modules**:
- there are **2 slice modules** in Rust:
  - [**`core::slice`**](https://doc.rust-lang.org/stable/core/slice/index.html)
    - **always available**, including `#[no_std]` environments;
    - defines **methods** for slice manipulation that **don't** require **heap memory**;
  - [**`alloc::slice`**](https://doc.rust-lang.org/stable/alloc/slice/index.html)
    - **requires** the `alloc` crate (usually present in `std`);
    - **extends** **`core::slice`** and adds **adds methods** that potentially require **heap memory**;
- **array module**:
  - [**`core::array`**](https://doc.rust-lang.org/stable/core/array/index.html)
  - [**`std::array`**](https://doc.rust-lang.org/stable/std/array/index.html)
- **vector module**:
  - [**`alloc::vec`**](https://doc.rust-lang.org/stable/alloc/vec/index.html)
  - [**`std::vec`**](https://doc.rust-lang.org/stable/std/vec/index.html)

<br>

Under the hood **`std::array`** uses **`core::array`**, **`alloc::vec`** and **`alloc::slice`**.<br>

<br>

**Slice** `[T]` **doesn't implement** the `Iterator` trait **directly**. Instead
- `[T]` implements `IntoIterator` trait for `&[T]` and `&mut [T]`, **but not** for `[T]`
- `[T]` use **separate structs** that implement the `Iterator` trait:
  - [**`core::slice::Iter`**](https://doc.rust-lang.org/stable/core/slice/struct.Iter.html);
  - [**`core::slice::IterMut`**](https://doc.rust-lang.org/stable/core/slice/struct.IterMut.html);

<br>

**Array** `[T; N]` **doesn't implement** the `Iterator` trait **directly**. Instead
- `[T; N]` implement `IntoIterator` trait for `[T; N]`, `&[T; N]` and `&mut [T; N]`
- `[T; N]` uses **separate structs** that implement the `Iterator` trait:
  - [**`core::slice::Iter`**](https://doc.rust-lang.org/stable/core/slice/struct.Iter.html);
  - [**`core::slice::IterMut`**](https://doc.rust-lang.org/stable/core/slice/struct.IterMut.html);
  - [**`core::array::IntoIter`**](https://doc.rust-lang.org/stable/core/array/struct.IntoIter.html);

<br>

**Vector** `Vec` **doesn't implement** the `Iterator` trait **directly**. Instead
- `Vec` implements `IntoIterator` trait for `Vec<T>`, `&Vec<T, A>` and `&mut Vec<T, A>`
- `Vec` uses **separate structs** that implement the `Iterator` trait:
  - [**`core::slice::Iter`**](https://doc.rust-lang.org/stable/core/slice/struct.Iter.html);
  - [**`core::slice::IterMut`**](https://doc.rust-lang.org/stable/core/slice/struct.IterMut.html);
  - [**`alloc::vec::IntoIter`**](https://doc.rust-lang.org/stable/alloc/vec/struct.IntoIter.html);

<br>

## Iterables
**Blanket impl** `IntoIterator` for any `Iterator`, in other words **any** `Iterator` is **iterable**:
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

`IntoIterator` implemetations for **slices** in file [**`core/src/slice/iter.rs`**](https://doc.rust-lang.org/stable/src/core/slice/iter.rs.html):
```rust
impl IntoIterator for &[T]
    fn into_iter(self) -> core::slice::Iter<T> { self.iter() }

impl IntoIterator for &mut [T]
    fn into_iter(self) -> core::slice::IterMut<T> { self.iter_mut() }
```

<br>

The methods `iter()`/`iter_mut()` available on `Vec` because `Vec` implements `Deref`/`DerefMut` with `Target=[T]`.<br>
`IntoIterator` implemetations for **vectors** in file [**`alloc/src/vec/mod.rs`**](https://doc.rust-lang.org/stable/src/alloc/vec/mod.rs.html):
```rust
impl IntoIterator for Vec<T>
    fn into_iter(self) -> alloc::vec::IntoIter<T> { ... }

impl IntoIterator for &Vec<T>
    fn into_iter(self) -> core::slice::Iter<T> { self.iter() } // --> core/src/slice/mod.rs --> actualy calls impl<T> [T] { .iter(&self) }

impl IntoIterator for &mut Vec<T>
    fn into_iter(self) -> core::slice::IterMut<T> { self.iter_mut() } // --> core/src/slice/mod.rs --> actualy calls impl<T> [T] { .iter_mut(&mut self) }
```

<br>

`IntoIterator` implemetations for **arrays** in file [**`core/src/array/iter.rs`**](https://doc.rust-lang.org/stable/src/core/array/iter.rs.html):
```rust
impl IntoIterator for [T; N]
    fn into_iter(self) -> core::array::IntoIter<T> { ... }
```

<br>

The methods `iter()`/`iter_mut()` available on `&[T; N]`/`&mut [T; N]` because the Rust compiler provides a built-in **unsized coercion** that allows a reference to an array `&[T; N]` to be implicitly converted into a reference to a slice `&[T]`. When you call a method on an **array**, the compiler follows a specific lookup order:
- it first **checks if the method is defined directly** on the array type `[T; N]`;
- if not found, it **tries autoref**: it automatically borrows the array as a reference `&[T; N]`;
- it then **applies coercion**: the reference `&[T; N]` is **coerced to a slice** `&[T]`;
- then the *slice type* `[T]` *methods* **become available** to the *array*;

<br>

`IntoIterator` implemetations for **arrays** in file [**`core/src/array/mod.rs`**](https://doc.rust-lang.org/stable/src/core/array/mod.rs.html):
```rust
impl IntoIterator for &[T; N]
    fn into_iter(self) -> core::slice::Iter<T> { self.iter() }

impl IntoIterator for &mut [T; N]
    fn into_iter(self) -> core::slice::IterMut<T> { self.iter_mut() }
```

<br>

## Iterators
*Slice* **not** `Iterator` in file [**`core/src/slice/iter.rs`**](https://doc.rust-lang.org/stable/src/core/slice/iter.rs.html):
```rust
impl<T> !Iterator for [T] {}
```

<br>

`Iterator` implemetations for [**`core::slice::Iter`**](https://doc.rust-lang.org/stable/core/slice/struct.Iter.html), [**`core::slice::IterMut`**](https://doc.rust-lang.org/stable/core/slice/struct.IterMut.html) in file [**`core/src/slice/iter.rs`**](https://doc.rust-lang.org/stable/src/core/slice/iter.rs.html) **indirectly** through macro `iterator!`:
```rust
// impl Iterator: indirectly through macro iterator!

iterator! {struct Iter ... }
iterator! {struct IterMut ... }

// Expanded version
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

<br>

`Iterator` implemetations for [**`alloc::vec::IntoIter`**](https://doc.rust-lang.org/stable/alloc/vec/struct.IntoIter.html) in file [**`alloc/src/vec/into_iter.rs`**](https://doc.rust-lang.org/stable/src/alloc/vec/into_iter.rs.html):
```rust
impl<T, A: Allocator> Iterator for IntoIter<T, A> {
    type Item = T;
    fn next(&mut self) -> Option<T> {...}
}
```

<br>

`Iterator` implemetations for [**`core::array::IntoIter`**](https://doc.rust-lang.org/stable/core/array/struct.IntoIter.html) in file [**`core/src/array/iter.rs`**](https://doc.rust-lang.org/stable/src/core/array/iter.rs.html):
```rust
impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> { ... }
}
```

<br>

# Diagrams
## Vec::into_iterator
![vec-into_iterator](/img/vec-into_iterator.svg)

<br>

## Vec::iter and Vec::iter_mut
![vec-iter-iter_mut](/img/vec-iter-iter_mut.svg)
