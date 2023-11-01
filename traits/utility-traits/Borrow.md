# Borrow and BorrowMut
Both traits are used to **treat borrowed types like owned types**.<br>

The `Borrow` is similar to `AsRef` (`a` is an instance of `A`):
- `impl Borrow<B> for A` indicates that a `a.borrow()` returns `&B`.
- `impl BorrowMut<B> for A` indicates that a `a.borrow_mut()` returns `&mut B`.

<br>

The main difference: `AsRef` is used with concrete types for **type conversion**, `Borrow` is used with genereics.<br>
Also `Borrow` is **stricter** than `AsRef`: if type `T` implements `Borrow` it means that `Eq`, `Ord` and `Hash` are **equivalent** for `&T`.<br>
Use `Borrow` and `BorrowMut` if `Eq`, `Ord` and `Hash` must be **equivalent** for **borrowed** and **owned** values: `x.borrow() == y.borrow()` should give the same result as `x == y`.<br>

<br>

> **Note**:<br>
> Choose `Borrow` when you’re building a data structure that treats **owned** (`T`) and **borrowed** (`&T`) values in **equivalent** ways, such as **hashing** and **comparison**.<br>

<br>

### Example 1
For types ``A`` and ``B`` ``impl Borrow<B> for A`` indicates that a borrowed ``A`` may be used where a ``B`` is desired.<br>

For instance, ``std::collections::HashMap.get()`` uses ``Borrow`` for its ``get()`` method, allowing a ``HashMap`` with keys of ``K`` to be indexed with a ``&Q``.<br>

```Rust
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

We can **insert key** of `String` type into `HashMap` and then use **key** of `&str` type for **searching**.<br>

```Rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("Foo".to_string(), 42);

assert_eq!(map.get("Foo"), Some(&42));
```

This is because the `std` has ``impl Borrow<&str> for String``.

<br>

## AsRef vs. Borrow
`Borrow` and `AsRef` are almost the same.<br>
```Rust
impl Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for MyBox<&str> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
```

<br>

## Declarations
### Borrow
```Rust
pub trait Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    fn borrow(&self) -> &Borrowed;
}
```

<br>

### BorrowMut
```Rust
pub trait BorrowMut<Borrowed>: Borrow<Borrowed>
where
    Borrowed: ?Sized,
{
    fn borrow_mut(&mut self) -> &mut Borrowed;
}
```

<br>

### Blanket implementation of `Borrow<T>` for `T`, `&T`, `&mut T` in `std`
```Rust
impl<T: ?Sized> const Borrow<T> for T {
    #[rustc_diagnostic_item = "noop_method_borrow"]
    fn borrow(&self) -> &T {
        self
    }
}

impl<T: ?Sized> const BorrowMut<T> for T {
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T: ?Sized> const Borrow<T> for &T {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> const Borrow<T> for &mut T {
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<T: ?Sized> const BorrowMut<T> for &mut T {
    fn borrow_mut(&mut self) -> &mut T {
        &mut **self
    }
}
```

<br>

# More examples
More examples [here](https://github.com/carmenere/easy-rust/blob/main/examples/traits/borrow-for-case-insensitive-str.md) and [here](https://github.com/carmenere/easy-rust/blob/main/examples/hashmaps/borrow-own-hashmap-implementation.md).
