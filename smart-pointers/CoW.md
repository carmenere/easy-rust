# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Cow](#cow)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Cow`|[std::borrow::Cow](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html)|

<br>

# Cow<T>
Declaration of type `Cow`:
```Rust
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

<br>

The type `Cow` is a smart pointer providing **clone-on-write** functionality: 
- it provides **immutable** access to **borrowed** data;
- it performs **clone** the data lazily when **mutation** or **ownership** is required.

<br>

`Cow<T>` implements the `Deref` trait which means it can directly call the immutable methods of `T`.<br>
`.to_mut()` returns **mutable reference** to owned data. It **clones** the data if it is **not** already owned.<br>
Multiple calls to `.to_mut()` will produce only **one** `.clone()`.

<br>

If we need to **mutate** `T`, then we can convert it into an **owned** variable using the `into_owned()`:<br>
 - if the variant of `Cow` was already `Owned` then we **move ownership**.
 - if the variant of `Cow` is `Borrowed`, then we **allocate** new memory.
