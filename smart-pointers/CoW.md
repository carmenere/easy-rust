# Cow
Defenition of type `Cow`:
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
- it can enclose and provide **immutable** access to **borrowed** data;
- it can **clone** the data lazily when **mutation** or **ownership** is required.

<br>

> **Notes**:<br>
> `Cow<T>` implements the `Deref` trait which means it can directly call the immutable methods of `T`.<br>
> If we need to **mutate** `T`, then we can convert it into an **owned** variable using the `into_owned()`:<br>
>  - if the variant of `Cow` was already `Owned` then we **move ownership**.<br>
>  - if the variant of `Cow` is `Borrowed`, then we **allocate** new memory.<br>
> `.to_mut()` returns **mutable reference** to owned data. It clones the data if it is not already owned.<br>
> Multiple calls to `.to_mut()` will produce only **one** `.clone()`.<br>
