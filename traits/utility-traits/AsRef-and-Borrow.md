# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declarations](#declarations)
  * [`Borrow`](#borrow)
  * [`BorrowMut`](#borrowmut)
  * [`AsRef`](#asref)
  * [`AsMut`](#asmut)
* [In a nutshell](#in-a-nutshell)
  * [When to use `Borrow` and `BorrowMut`?](#when-to-use-borrow-and-borrowmut)
  * [When to use `AsRef` and `AsMut`?](#when-to-use-asref-and-asmut)
* [Implementations in `std`](#implementations-in-std)
  * [`Borrow` in `HashMap`](#borrow-in-hashmap)
* [Blanket implementations](#blanket-implementations)
  * [`Borrow`](#borrow-1)
    * [`impl Borrow for T`](#impl-borrow-for-t)
    * [`impl Borrow for &T`](#impl-borrow-for-t-1)
    * [`impl Borrow for &mut T`](#impl-borrow-for-mut-t)
  * [`BorrowMut`](#borrowmut-1)
    * [`impl BorrowMut for T`](#impl-borrowmut-for-t)
    * [`impl BorrowMut for &mut T`](#impl-borrowmut-for-mut-t)
  * [`AsRef`](#asref-1)
    * [`impl AsRef for &T`](#impl-asref-for-t)
    * [`impl AsRef for &mut T`](#impl-asref-for-mut-t)
  * [`AsMut`](#asmut-1)
    * [`impl AsMut for &mut T`](#impl-asmut-for-mut-t)
* [Examples](#examples)
  * [Function that accepts both `&str` and `String`](#function-that-accepts-both-str-and-string)
  * [`AsRef<Path>>`](#asrefpath)
  * [More examples](#more-examples)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Borrow`|[std::borrow::Borrow](https://doc.rust-lang.org/std/borrow/trait.Borrow.html)|
|`BorrowMut`|[std::borrow::BorrowMut](https://doc.rust-lang.org/std/borrow/trait.BorrowMut.html)|
|`AsRef`|[std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)|
|`AsMut`|[std::convert::AsMut](https://doc.rust-lang.org/std/convert/trait.AsMut.html)|

<br>

# Declarations
## `Borrow`
The trait `Borrow` **immutably** borrows from an **owned** value.<br>

```rust
pub trait Borrow<B: ?Sized> {
    fn borrow(&self) -> &B;
}
```

<br>

## `BorrowMut`
The trait `BorrowMut` **mutably** borrows from an **owned** value.<br>
The `Borrow` trait is a **supertrait** of `BorrowMut`, i.e. implementing `BorrowMut` requires to also implement `Borrow`.<br>

```rust
pub trait BorrowMut<B: ?Sized>: Borrow<B> {
    fn borrow_mut(&mut self) -> &mut B;
}
```

<br>

## `AsRef`
The trait `AsRef` converts this type into a **shared** *reference* of antoher type.<br>

```rust
pub trait AsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}
```

<br>

## `AsMut`
Converts this type into a **mutable** *reference* of antoher type.<br>

```rust
pub trait AsMut<T: ?Sized> {
    fn as_mut(&mut self) -> &mut T;
}
```

<br>

# In a nutshell
`Borrow` and `AsRef` are almost the same (`a` is an instance of `A`):
- `impl Borrow<B> for A` indicates that a `a.borrow()` returns `&B`;
- `impl AsRef<U> for A` indicates that a `a.as_ref()` returns `&U`;

<br>

`BorrowMut` and `AsMut` are almost the same (`a` is an instance of `A`):
- `impl BorrowMut<B> for A` indicates that a `a.borrow_mut()` returns `&mut B`;
- `impl AsMut<U> for A` indicates that a `a.as_mut()` returns `&mut U`;

<br>

## When to use `Borrow` and `BorrowMut`?
The trait `Borrow<B>` is **stricter** than `AsRef<U>`. In other words, if `T` implements `Borrow<B>` it means that all traits `Eq`, `Ord` and `Hash` give the **same results** for both `T` and `&B`. In other words, `x.borrow() == y.borrow()` should give the same result as `x == y`.<br>

<br>

## When to use `AsRef` and `AsMut`?
`From` and `Into` traits are also used for **conversion**. So, when implement `From` and `Into`, when implement `AsRef` and `AsMut`?<br>
Both `AsRef<U>` and `AsMut<U>` are **expected** to be **cheap**, i.e., they **don't require copying** of data and **allocation** of new memory and in most cases performed in **constant time** O(1), whereas `From` and `Into` conversions are **not** guaranteed to be cheap.<br>

<br>

# Implementations in `std`
- `Vec<T>`:
  - `impl<T> AsRef<[T]> for Vec<T>`;
  - `impl<T> AsMut<[T]> for Vec<T>`;
  - `impl<T> Borrow<[T]> for Vec<T>`;
  - `impl<T> BorrowMut<[T]> for Vec<T>`;
- `String`:
  - `impl Borrow<str> for String`;
  - `impl BorrowMut<str> for String`;
  - `impl AsRef<[u8]> for String`;
  - `impl AsRef<str> for String`;
  - `impl AsMut<str> for String`;
- `str`:
  - `impl AsRef<str> for str`
  - `impl AsMut<str> for str`
  - `impl AsRef<[u8]> for str`

<br>

## `Borrow` in `HashMap`
[`HashMap` doc](https://doc.rust-lang.org/std/collections/struct.HashMap.html).<br>
[Using `Borrow` in `HashMap`](https://doc.rust-lang.org/std/borrow/trait.Borrow.html#examples):
- The entire hash map is generic over a key type `K`;
- The `get` method is generic over the type `Q`;
- `K` and `Q` are connected: `K` must implement `Borrow<Q>`;

<br>

Slightly simplified defenition of `HashMap<K, V>` look like this:
```rust
use std::borrow::Borrow;
use std::hash::Hash;

pub struct HashMap<K, V> {
    // fields omitted
}

impl<K, V> HashMap<K, V> {
    pub fn insert(&self, key: K, value: V) -> Option<V>
    where K: Hash + Eq
    {
        // ...
    }

    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized
    {
        // ...
    }
}
```

<br>

So,
- the **key** of `String` type is used when **inserting**;
- the **key** of `&str` type is used for **searching**;

This is because `impl Borrow<str> for String`.

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("Foo".to_string(), 42);

assert_eq!(map.get("Foo"), Some(&42));
```

<br>

# Blanket implementations
## `Borrow`
### `impl Borrow for T`
Note, then `&self` is a short form for `self: &Self`, but `Self` is equal to `T` in blanket implementation, so `self` is `&T`.<br>
```rust
impl<T: ?Sized> Borrow<T> for T {
    fn borrow(&self) -> &T {
        self
    }
}
```

<br>

### `impl Borrow for &T`
```rust
impl<T: ?Sized> Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}
```

<br>

### `impl Borrow for &mut T`
```rust
impl<T: ?Sized> Borrow<T> for &mut T {
    fn borrow(&self) -> &T {
        &**self
    }
}
```

<br>

## `BorrowMut`
### `impl BorrowMut for T`
Note, then `&mut self` is a short form for `self: &mut Self`, but `Self` is equal to `T` in blanket implementation, so `self` is `&mut T`.<br>
```rust
impl<T: ?Sized> BorrowMut<T> for T {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}
```

<br>

### `impl BorrowMut for &mut T`
```rust
impl<T: ?Sized> BorrowMut<T> for &mut T {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}
```

<br>

## `AsRef`
### `impl AsRef for &T`
```Rust
impl<T: ?Sized, U: ?Sized> AsRef<U> for &T
where
    T: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}
```

It means: for **any types** `T` and `U`, if `T: AsRef<U>`, then `&T: AsRef<U>` as well.<br>

<br>

### `impl AsRef for &mut T`
```rust
impl<T: ?Sized, U: ?Sized> AsRef<U> for &mut T
where
    T: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        <T as AsRef<U>>::as_ref(*self)
    }
}
```

It means: for **any types** `T` and `U`, if `T: AsRef<U>`, then `&mut T: AsRef<U>` as well.<br>

<br>

## `AsMut`
### `impl AsMut for &mut T`
```rust
impl<T: ?Sized, U: ?Sized> AsMut<U> for &mut T
where
    T: AsMut<U>,
{
    fn as_mut(&mut self) -> &mut U {
        (*self).as_mut()
    }
}
```

<br>

# Examples
## Function that accepts both `&str` and `String`
```rust
fn print<T> (s: T)
where
    T: AsRef<str>
{
    println!("{}", s.as_ref());
}

fn main() {
    let foo = "Foo";
    let bar = String::from("Bar");
    print(foo);
    print(bar);
}
```

<br>

## `AsRef<Path>>`
Consider function `std::fs::File.open()`:
```Rust
fn open<P: AsRef<Path>>(path: P) -> Result<File>
```

This allows `File.open()` to accept not only `Path`, but also `OsStr`, `OsString`, `&str`, `String`, and `PathBuf` with implicit conversion because these types all implement `AsRef<Path>`.

<br>

## More examples
- [borrow-for-case-insensitive-str](../../examples/traits/borrow-for-case-insensitive-str.md)
- [borrow-own-hashmap-implementation](../../examples/hashmaps/borrow-own-hashmap-implementation.md)
