# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [AsRef and AsMut](#asref-and-asmut)
  - [When to use AsRef and AsMut?](#when-to-use-asref-and-asmut)
    - [Antipattern](#antipattern)
  - [Declarations](#declarations)
    - [AsRef](#asref)
    - [AsMut](#asmut)
    - [Blanket implementation of `AsRef` in `std`](#blanket-implementation-of-asref-in-std)
      - [Example](#example)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`AsMut`|[std::convert::AsMut](https://doc.rust-lang.org/std/convert/trait.AsMut.html)|
|`AsRef`|[std::convert::AsRef](https://doc.rust-lang.org/std/convert/trait.AsRef.html)|

<br>

# AsRef and AsMut
Compared to `Deref`, which has an *implicit* behavior, `AsRef` is an **explicit** conversion.<br>

For types `A` and `B` (`a` is an instance of `A`):
- `impl AsRef<B> for A` indicates that a `a.as_ref()` returns `&B`.
- `impl AsMut<B> for A` indicates that a `a.as_mut()` returns `&mut B`.

<br>

For instance:
- `Vec<T>` implements `AsRef<[T]>`;
- `String` implements `AsRef<str>` and `AsRef<[u8]>`.

<br>

## When to use AsRef and AsMut?<br>
`From` and `Into` traits are also used for **conversion**. So, when implement `From` and `Into`, when implement `AsRef` and `AsMut`?<br>

`AsRef` and `AsMut` conversions are expected to be **cheap** - i.e. they don't require any data **copying** or **allocation** of new memory and in most cases performed in **constant time** O(1), whereas `From` and `Into` conversions are **not** guaranteed to be cheap.<br>

### Antipattern
`AsRef` is useful where you **only** need to **borrow** the input parameter.<br>
If you find yourself doing `let x = param.as_ref().to_owned();` you’re doing it **wrong**. Use `Into` instead.<br>

```Rust
fn f<S>(p: S)
where S: AsRef<String> { 
    let x = p.as_ref().to_owned();
}
```

<br>

## Declarations
### AsRef
```Rust
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}
```

<br>

### AsMut
```Rust
pub trait AsMut<T>
where
    T: ?Sized,
{
    fn as_mut(&mut self) -> &mut T;
}
```

<br>

### Blanket implementation of `AsRef` in `std`
```Rust
impl<a', T, U> AsRef<U> for &a' T
where T: AsRef<U> + ?Sized,
      U: ?Sized
{
    fn as_ref(&self) -> &U {
        (*self).as_ref()
    }
}
```

<br>

It means: for **any types** `T` and `U`, if `T: AsRef<U>`, then `&T: AsRef<U>` as well.

<br>

#### Example
Consider function `std::fs::File.open()`:
```Rust
fn open<P: AsRef<Path>>(path: P) -> Result<File>
```

<br>

This allows `File.open()` to accept not only `Path`, but also `OsStr`, `OsString`, `&str`, `String`, and `PathBuf` with implicit conversion because these types all implement `AsRef<Path>`.
