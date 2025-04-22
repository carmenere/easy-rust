# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Trait `Index`](#trait-index)
  * [Declaration](#declaration)
* [Trait `IndexMut`](#trait-indexmut)
  * [Declaration](#declaration-1)
  * [Aliasing vector and its element](#aliasing-vector-and-its-element)
<!-- TOC -->

<br>

# URLs
|Trait| URL                                                                           |
|:----|:------------------------------------------------------------------------------|
|`Index`| [std::ops::Index](https://doc.rust-lang.org/std/ops/trait.Index.html)         |
|`IndexMut`| [std::ops::IndexMut](https://doc.rust-lang.org/std/ops/trait.IndexMut.html)|

<br>

# Trait `Index`
## Declaration
```rust
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    
    fn index(&self, index: Idx) -> &Self::Output;
}
```

<br>

Associated type `Output` defines **returned type after indexing**.<br>
The method `into()` performs the **indexing operation** (`container[N]`). It **may panic** if the **index is out of bounds**.<br>

The **indexing operation** `container[N]` is a **syntactic sugar** for: `Index::index(&'a container, N)`.<br> 

<br>

# Trait `IndexMut`
## Declaration
```rust
pub trait IndexMut<Idx: ?Sized>: Index<Idx> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

<br>

The method `index_mut()` performs the **mutable indexing operation** (`container[N]`). It **may panic** if the **index is out of bounds**.<br>

The **mutable indexing operation** `container[N]` is a **syntactic sugar** for: `Index::index_mut(&'a container, N)`.<br>

<br>

## Aliasing vector and its element
Consider code:
```rust
let mut data = vec![1, 2, 3];
let x = &data[0];
data.push(4);
println!("{}", x);
```

<br>

The `&data[0]` is a **syntactic sugar** for: `let x = data.index(0)` or `let x = Index::index(&data, 0)`.<br>

<br>

Implementation of `std::ops::Index` for `Vec<T>`:
```rust
impl<T, I: SliceIndex<[T]>, A: Allocator> Index<I> for Vec<T, A> {
    type Output = I::Output;
  
    fn index(&self, index: I) -> &Self::Output {
        Index::index(&**self, index)
    }
}
```

<br>

According to **lifetime elision**, compiler assigns the same lifetime `'a` for both `&self` and `&Self::Output`.<br>
This means, that `data` is **considered borrowed until last use of** `x` **at the caller**.<br>

<br>