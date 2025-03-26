# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declarations](#declarations)
  * [`Iterator`](#iterator)
  * [`IntoIterator`](#intoiterator)
  * [`FromIterator`](#fromiterator)
* [In a nutshell](#in-a-nutshell)
  * [Iterators](#iterators)
  * [Iterables](#iterables)
  * [`FromIterator`](#fromiterator-1)
* [Blanket implementations](#blanket-implementations)
  * [`IntoIterator`](#intoiterator-1)
    * [`impl<I: Iterator> IntoIterator for I`](#impli-iterator-intoiterator-for-i)
* [Iterators](#iterators-1)
* [Loop syntax](#loop-syntax)
* [`Iterator::collect()`](#iteratorcollect)
* [`Turbofish`](#turbofish)
    * [Example](#example)
* [Method `drain()`](#method-drain)
* [Examples](#examples)
  * [Implementing own `Iterator`](#implementing-own-iterator)
  * [`IntoIterator`](#intoiterator-2)
  * [Implementing `IntoIterator` for `MyCollection`](#implementing-intoiterator-for-mycollection)
* [`IntoIterator` for arrays](#intoiterator-for-arrays)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Iterator`|[std::iter::Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)|
|`IntoIterator`|[std::iter::IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)|
|`FromIterator`|[std::iter::FromIterator](https://doc.rust-lang.org/std/iter/trait.FromIterator.html)|

<br>

# Declarations
## `Iterator`
```rust
pub trait Iterator {
    type Item;

    // Required method
    fn next(&mut self) -> Option<Self::Item>;
}
```

- Associated type `Item` is the type of the elements being iterated over.
- The `next()` method returns `Some(Item)` or `None` to **indicate** the **end of the sequence**.

<br>

## `IntoIterator`
```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    // Required method
    fn into_iter(self) -> Self::IntoIter;
}
```

<br>

## `FromIterator`
```rust
pub trait FromIterator<A>: Sized {
    // Required method
    fn from_iter<T>(iter: T) -> Self
       where T: IntoIterator<Item = A>;
}
```

<br>

# In a nutshell
## Iterators
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

## Iterables
**Iterable type** is an **any type** that implements `IntoIterator` trait. **Collections** usually implement `IntoIterator` **to be able to iterate** *over its items*.<br>
`IntoIterator` returns `Iterator` over some type `U`.<br>

<br>

`IntoIterator` for collection, e.g, `SomeCollection`, can return `Iterator` **over**: 
- *values* of type `T`, **move semantics**;
- *shared references*, **allow** to **reuse** *original collection* **after** iteration;
- *mutable references*, **allow** to **reuse** *original collection* **after** iteration.

<br>



Over **values** of type **T**
```rust
impl IntoIterator for SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=T> {
  }
}
```


Over **shared references**: `&T`
```rust
impl IntoIterator for &SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&T> {
  }
}
```


Over **mutable references**: `&mut T`
```rust
impl IntoIterator for &mut SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&mut T> {
  }
}
```


<br>

## `FromIterator`
Trait `FromIterator` is used for conversion **from** an `Iterator` **to** **collection**.<br>

By implementing `FromIterator` for a **collection** type, you define how it will be created **from** an **iterator**.<br>
`FromIterator::from_iter()` is rarely called explicitly, and `FromIterator::from_iter()` is usually used through `Iterator::collect()` method.

<br>

# Blanket implementations
## `IntoIterator`
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

So, every `Iterator` returns itself from `.into_iter()`, in other words you **don't need** to implement `IntoIterator` for some type if it implements `Iterator`.<br>

<br>

# Iterators
Consider example:
```rust
for item in collection {
    ...
}
```

In this example, after `for` loop *collection* `collection` is become **invalid**.<br>

Access to **collections** in loops uses `move semantics` by default.

<br>

To make the `collection` **reusable after loop** use `immutable reference` to access to the `collection`:
```rust
for item in &collection {
    ...
}
```

<br>

To **modify item** *during* the loop use `mutable reference` to access to the `collection`:
```rust
for item in &mut collection {
    ...
}
```

<br>

For loop under the hood:
1. `into_iter()`
```rust
for item in collection
```
is expanded to
```rust
for item in collection.into_iter()
```

2. `iter()`
```rust
for item in &collection
```
is expanded to
```rust
for item in collection.iter()
```

3. `iter_mut()`
```rust
for item in &mut collection
```
is expanded to
```rust
for item in collection.iter_mut()
```


<br>

# Loop syntax
`for ... in ...` syntax is just a syntactic sugar for an `IntoIterator::into_iter()` invocation, followed by repeated calling of `Iterator::next()`.<br>

Contexts:
- The call `(T).into_iter()` returns an `Iterator` over `T`;
- The call `(&T).into_iter()` returns an `Iterator` over `&T`;
- The call `(&mut T).into_iter()` returns an `Iterator` over `&mut T`.

<br>

`Iterator` over `T`: 
```rust
for x in v {
  // body
}
```

**=>**

```rust
let mut iter = (v).into_iter();
```

**=>**

```rust
loop {
    match iter.next() {
        Some(x) => {
          // body
        },
        None => break,
    }
}
```


`Iterator` over `&T`:
```rust
for x in &v {
  // body
}
```

**=>**

```rust
let mut iter = (&v).into_iter();
```

`Iterator` over `&mut T`
```rust
for x in &mut v {
  // body
}
```

**=>**

```rust
let mut iter = (&mut v).into_iter();
```

<br>

# `Iterator::collect()`
```rust
pub trait Iterator {
    type Item;
    // ...
    fn collect<B>(self) -> B
    where
        B: FromIterator<Self::Item>,
        Self: Sized 
    { 
        // ... 
    }
    // ...
}
```

- Method `.collect::<T>()` of `Iterator` type requires its returning type `T` to implement `FromIterator`.
- Because `collect()` only cares about collection type, not its elements, you can use type `_`, e.g. `.collect::<Vec<_>>()`.
- `FromIterator::from_iter()` is more readable alternative to `Iterator::collect()`: `collect()` is more general, because it can return collection of nay type, while `from_iter()` returns collection only of `Self` type.

`collect()` transforms an **iterator** into a **collection**.<br>
`collect()` can also create instances of types that are not typical collections, e.g., `collect()` can return `Result<SomeCollection<T>, E>`.

<br>

# `Turbofish`
Because `collect()` is so general, it can cause **problems** with **type inference**.<br>
Internally, `collect()` just uses `FromIterator`, but it also **infers** the **type** of the **output**.<br>
Sometimes there **isn't enough** information to infer the type, so you may need to **explicitly** specify the type you want.<br>
There is *special syntax* in Rust called **turbofish**: `::<SomeType>`.<br>
Example: `let all_scores = score_table.values().cloned().collect::<Vec<Score>>();`.<br>
**Turbofish** helps the **inference** algorithm to understand type of item of **resulting collection**.<br>

### Example
```rust
fn main() {
    let numbers: Vec<i32> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
    ];

    let even_numbers = numbers
        .into_iter()
        .filter(|n| n % 2 == 0)
        .collect();

    println!("{:?}", even_numbers);
}
```

**Output**:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0282]: type annotations needed
 --> src/main.rs:6:9
  |
6 |     let even_numbers = numbers
  |         ^^^^^^^^^^^^ consider giving `even_numbers` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playrs` due to previous error
```

<br>

This is because the compiler **doesn’t know** what type you’re trying to collect your **iterator** into.<br>

This can be fixed in two different ways:
- by **declaring** the **type** of variable in `let` **binding**: 
```rust
let even_numbers: Vec<i32> = ...
```
- by using a **turbofish**:
```rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<i32>>();
```

The `::<Vec<i32>>` part is the **turbofish** and means collect this **iterator** into a `Vec<i32>`.<br>

You can actually replace `i32` with `_` in **turbofish** and let the compiler infer it because it knows the **iterator** yields `i32`:
```rust
let even_numbers = numbers
    .into_iter()
    .filter(|n| n % 2 == 0)
    .collect::<Vec<_>>();
```

<br>

# Method `drain()`
`into_iter()` **consumes** the **collection** **itself**, `drain()` only **consumes** the **values** in the collection.<br>

Therefore `drain()` allows draining of only a **part of the collection**.<br>

So,
- use `into_iter()` if you want to *consume* the **entire** collection;
- use `drain()` if you only want to *consume* **part** of the collection or if you want to *reuse* the **emptied collection** later;

<br>

# Examples
## Implementing own `Iterator`
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

## `IntoIterator`
For example, `Vec` implements `IntoIterator` for **all** cases: `T`, `&T` and `&mut T`.<br>

But not every type provides all these 3 implementations:
- `HashSet` and `BinaryHeap` **don’t** implement on **mutable** references;
- `Arrays` (**until** Rust **1.53**) and `Slices` implement **only** `&T` and `&mut T`: 
    - `&[T]`
    - `&mut [T]`

<br>

## Implementing `IntoIterator` for `MyCollection`
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

# `IntoIterator` for arrays
**Until** Rust **1.53**, `only` **references to arrays** implement `IntoIterator`.<br>

This means you **can** iterate **over** `&[1, 2, 3]` and `&mut [1, 2, 3]`, but **not** **over** `[1, 2, 3]` **directly**.

```rust
for &e in &[1, 2, 3] {} // Ok
for e in [1, 2, 3] {} // Error
```

**This feature** has been a **long-standing issue**, but the **solution is not as simple as it seems**.<br>
Just adding the trait implementation would break existing code.<br>
It has been suggested many times to "only implement `IntoIterator` for arrays in Rust 2021".<br>
However, this is simply **not possible**. You can't have a trait implementation exist in one edition and not in another, since editions can be mixed.<br>

Instead, the trait implementation was added in all editions (starting in Rust **1.53.0**) but with a small hack to avoid breakage **until** Rust **2021**:
- In Rust **2015** and **2018** code, the compiler will still resolve `array.into_iter()` to `(&array).into_iter()` like before, as if the trait implementation does not exist. 
- This only applies to the `.into_iter()` method call syntax. It **doesn't** affect **any other syntax** such as 
`for e in [1, 2, 3]`,  `iter.zip([1, 2, 3])` or `IntoIterator::into_iter([1, 2, 3])`. Those will work in all editions.

<br>