# Borrow and BorrowMut
Both traits are used to **treat borrowed types like owned types**.<br>

The `Borrow` is similar to `AsRef` (`a` is an instance of `A`):
- `impl Borrow<B> for A` indicates that a `a.borrow()` returns `&B`.
- `impl BorrowMut<B> for A` indicates that a `a.borrow_mut()` returns `&mut B`.

<br>

There is a **difference** between `Borrow` and `AsRef` and they both have their own uses:
- the `Borrow` trait is used to **borrow** data;
- the `AsRef` trait is used for **type conversion**.

<br>

Also `Borrow` is **stricter** than `AsRef`: if type `T` implements `Borrow` it means that `&T` **hashes** and **compares** the same way as the value of `T`.<br>

<br>

## When to use Borrow and BorrowMut?<br>
Use `Borrow` and `BorrowMut` if `Eq`, `Ord` and `Hash` must be **equivalent** for **borrowed** and **owned** values: `x.borrow() == y.borrow()` should give the same result as `x == y`.

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

We can **insert** **key** of ``String`` type into ``HashMap`` and then use **key** of ``&str`` type for **searching**.<br>

```Rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("Foo".to_string(), 42);

assert_eq!(map.get("Foo"), Some(&42));
```

This is because the `std` has ``impl Borrow<&str> for String``.

<br>

### Example 2
```Rust
pub struct CaseInsensitiveString(String);

impl PartialEq for CaseInsensitiveString {
    fn eq(&self, other: &Self) -> bool {
        // Note that the comparison here is required to ignore ascii case
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for CaseInsensitiveString { }

impl Hash for CaseInsensitiveString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in self.0.as_bytes() {
            c.to_ascii_lowercase().hash(state)
        }
    }
}
```

<br>

Can `CaseInsensitiveString` implement `Borrow<&str>`?

Obviously, `CaseInsensitiveString` and `&str` have **different** implementations of `Hash`:
- `&str` **doesn't** ignore case;
- `CaseInsensitiveString` **ignores** case.

<br>

So, `CaseInsensitiveString` **cannot** be used as a key for a `HashMap` and `Borrow<&str>` must **not** be implemented for `CaseInsensitiveString`.<br>
But `AsRef` can be implemented for `CaseInsensitiveString`.<br>

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
