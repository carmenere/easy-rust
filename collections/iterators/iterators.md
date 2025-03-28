# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [`Iterator`](#iterator)
  * [Declaration](#declaration)
  * [In a nutshell](#in-a-nutshell)
  * [Example](#example)
* [`IntoIterator`](#intoiterator)
  * [Declaration](#declaration-1)
  * [In a nutshell](#in-a-nutshell-1)
  * [Blanket implementations](#blanket-implementations)
    * [`impl<I: Iterator> IntoIterator for I`](#impli-iterator-intoiterator-for-i)
  * [Example](#example-1)
* [`FromIterator`](#fromiterator)
  * [Declaration](#declaration-2)
  * [In a nutshell](#in-a-nutshell-2)
* [Loop syntax](#loop-syntax)
  * [iter()/iter_mut()](#iteriter_mut)
* [`IntoIterator` for `Vec`](#intoiterator-for-vec)
* [`IntoIterator` for `[T; N]` and `[T]`](#intoiterator-for-t-n-and-t)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Iterator`|[std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)|
|`IntoIterator`|[std::iter::IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)|
|`FromIterator`|[std::iter::FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)|

<br>


# `Iterator`
## Declaration
```rust
pub trait Iterator {
  type Item;

  // Required method
  fn next(&mut self) -> Option<Self::Item>;

  // Provided methods
  fn collect<B: FromIterator<Self::Item>>(self) -> B
  where Self: Sized,
  {
    <B as FromIterator>::from_iter(self)
  }
}
```

- Associated type `Item` is the type of the elements being iterated over.
- The `next()` method returns `Some(Item)` or `None` to **indicate** the **end of the sequence**.

<br>

## In a nutshell
An **iterator** is an object that enables to **traverse its elements**.<br>
An **iterator** is **stateful** because it **keeps track** of *where it is* in the iteration process.<br>

An **iterator** supports at least the following features:
- **get** the current element;
- **advance** to the next element;
- **signal** when no more elements are available and return `None`.

In Rust, **iterator** must implement `Iterator` trait.<br>

<br>

`Iterator` can also be useful in generic code:
- you can use a bound like `T: Iterator` to restrict the type variable to types that are iterators;
- you can write `T: Iterator<Item=U>` to restrict the type variable to types that are **iterators** **that yield a particular type** `U`.

<br>

## Example
```rust
struct MyIterator;

impl Iterator for MyIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> { 
        Some("abc".to_string()) 
    }
}

fn main() {
    let mut iter = MyIterator;
    let value = iter.next();
}
```

<br>

# `IntoIterator`
## Declaration
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    // Required method
    fn into_iter(self) -> Self::IntoIter;
}
```

<br>

## In a nutshell
Any type that implements `IntoIterator` is called **iterable**. An **iterable** returns `Iterator` over some item `T`.<br>
**Collections** usually implement `IntoIterator` **to be able to iterate** *over its items*.<br>

<br>

## Blanket implementations
### `impl<I: Iterator> IntoIterator for I`
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

So, every `Iterator` returns **itself** from `.into_iter()`, in other words you **don't need** to implement `IntoIterator` for some type if it implements `Iterator`.<br>

<br>

## Example
```rust
// A sample collection, that's just a wrapper over Vec<T>
#[derive(Debug)]
struct MyCollection(Vec<i32>);

// Let's give it some methods so we can create one and add things
// to it.
impl MyCollection {
  fn new() -> MyCollection {
      MyCollection(Vec::new())
  }

  fn add(&mut self, elem: i32) {
      self.0.push(elem);
  }
}

// and we'll implement IntoIterator
impl IntoIterator for MyCollection {
  type Item = i32;
  type IntoIter = std::vec::IntoIter<Self::Item>;

  fn into_iter(self) -> Self::IntoIter {
      self.0.into_iter()
  }
}

// Now we can make a new collection...
let mut c = MyCollection::new();

// ... add some stuff to it ...
c.add(0);
c.add(1);
c.add(2);

// ... and then turn it into an Iterator:
for (i, n) in c.into_iter().enumerate() {
    assert_eq!(i as i32, n);
}
```

<br>

# `FromIterator`
## Declaration
```rust
pub trait FromIterator<A>: Sized {
    // Required method
    fn from_iter<T>(iter: T) -> Self
       where T: IntoIterator<Item = A>;
}
```

<br>

## In a nutshell
Trait `FromIterator` is used for conversion **from** an `Iterator` **to** _collection_.<br>

By implementing `FromIterator` for a **collection** type, you define how it will be created **from** an **iterator**.<br>
`FromIterator::from_iter()` is rarely called explicitly, and `FromIterator::from_iter()` is usually used through `Iterator::collect()` method.<br>

<br>

# Loop syntax
**Consider example**:
```rust
for item in collection {
    ...
}
```

In this example, after `for in` loop *collection* `collection` is become **invalid**.<br>
Note that `IntoIterator::into_iter(self)` **consumes** iterable because of `self`.<br>
Access to **collections** in loops uses **move semantics** by default.<br>

<br>

To make the `collection` **reusable after loop** use `&` to access to the `collection`:
```rust
for item in &collection {
    ...
}
```

<br>

To **modify item** *during* the loop use `&mut` to access to the `collection`:
```rust
for item in &mut collection {
    ...
}
```

<br>

The `for item in iterable` syntax requires **iterable**, so it works with both **iterators** (every iterator is iterable too) and **collections** that implement `IntoIterator`.<br>
The `for item in iterable` syntax is just a syntactic sugar for:
```rust
let mut iterator = iterable.into_iter();

loop {
    match iterator.next() {
        Some(x) => {
          // body
        },
        None => break,
    }
}
```

**here**:
- `let mut iterator = iterable.into_iter()` returns some **iterator**;
- `iterator.next()` is called repeatedly inside `loop`;

<br>

So, there are 3 variants of `for ... in ...` loop:
- `for item in iterable`
  - here `let mut iterator = iterable.into_iter();` returns an `Iterator<Item=T>` over items of `T` type, **move semantics**;
- `for item in &iterable`
  - here `let mut iterator = (&iterable).into_iter();` returns an `Iterator<Item=&T>` over items of `&T` type, this **allows** to **reuse** *original collection* **after** iteration;
- `for item in &mut iterable`
  - here `let mut iterator = (&mut iterable).into_iter();` returns an `Iterator<Item=&mut T>` over items of `&mut T` type, this **allows** to **reuse** *original collection* **after** iteration;;

<br>

To provide support for **all 3** variants of `for ... in ...` loop the **iterable** must implement `IntoIterator` for **all 3** cases: `T`, `&T` and `&mut T`:
- `T`
```rust
impl IntoIterator for SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=T> {
  }
}
```
- `&T`
```rust
impl IntoIterator for &SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&T> {
  }
}
```
- `&mut T`
```rust
impl IntoIterator for &mut SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&mut T> {
  }
}
```

<br>

## iter()/iter_mut()
Slices implements `iter(&self)` and `iter_mut(&mut self)` methods:
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

The methods `iter`/`iter_mut` available on `Vec` because `Vec` implements `Deref`/`DerefMut` with `Target=[T]`.<br>
Also slices implement `IntoIterator` for `&'a [T]` and for `&'a mut [T]`, **but not** for `[T]`.<br>

<br>

**Conventions**:
- If collection implements `IntoIterator for &SomeCollection<T>` it must implement by convention method `.iter()`;
- If collection implements `IntoIterator for &mut SomeCollection<T>` it must implement by convention method `.iter_mut()`;

<br>

# `IntoIterator` for `Vec`
For example, `Vec` implements `IntoIterator` for **all** cases: `T`, `&T` and `&mut T`.<br>

But **not** every type provides all these 3 implementations:
- `HashSet` and `BinaryHeap` **don’t** implement on **mutable** references;
- **Arrays** `[T; N]` (**until** Rust **1.53**) and **slices** `[]` implement **only** `&T` and `&mut T`: 
    - `&[T]`
    - `&mut [T]`

<br>

# `IntoIterator` for `[T; N]` and `[T]`
**Until** Rust **1.53**, `only` **references to arrays** implement `IntoIterator`.<br>
This means you **can** iterate **over** `&[1, 2, 3]` and `&mut [1, 2, 3]`, but **not** **over** `[1, 2, 3]` **directly**:
```rust
for &e in &[1, 2, 3] {} // Ok
for e in [1, 2, 3] {} // Error
```

<br>

**This feature** has been a **long-standing issue**, but the **solution is not as simple as it seems**.<br>
Just adding the trait implementation would break existing code.<br>
It has been suggested many times to "only implement `IntoIterator` for arrays in Rust 2021".<br>
However, this is simply **not possible**. You can't have a trait implementation exist in one edition and not in another, since editions can be mixed.<br>

<br>

Instead, the trait implementation was added in all editions (starting in Rust **1.53.0**) but with a small hack to avoid breakage **until** Rust **2021**:
- In Rust **2015** and **2018** code, the compiler will still resolve `array.into_iter()` to `(&array).into_iter()` like before, as if the trait implementation does not exist. 
- This only applies to the `.into_iter()` method call syntax. It **doesn't** affect **any other syntax** such as 
`for e in [1, 2, 3]`,  `iter.zip([1, 2, 3])` or `IntoIterator::into_iter([1, 2, 3])`. Those will work in all editions.
