# Cow<B>
Path to type in std: ``std::borrow:Cow``.

Defenition of type ``Cow``:
```Rust
pub enum Cow<'a, B> where
    B: 'a + ToOwned + ?Sized, {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

The type ``Cow`` is a smart pointer providing **clone-on-write** functionality: 
- it can enclose and provide **immutable** access to **borrowed** data;
- it can **clone** the data lazily when **mutation** or **ownership** is required.

The type is designed to work with general **borrowed** data via the ``Borrow`` trait.<br>

``Cow`` implements ``Deref``, which means that you can call non-mutating methods directly on the data it encloses.