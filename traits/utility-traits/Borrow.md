# Borrow and BorrowMut
Both traits are used to **treat borrowed types like owned types**.<br>

The `Borrow` is similar to `AsRef` (`a` is an instance of `A`):
- `impl Borrow<B> for A` indicates that a `a.borrow()` returns `&B`.
- `impl BorrowMut<B> for A` indicates that a `a.borrow_mut()` returns `&mut B`.

<br>

### Example 1: Own HashMap implementation
```Rust
use std::borrow::Borrow;

#[derive(Debug)]
struct MyBox<T>(T);

impl Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

struct MyHashMap<K, V> {
    keys: Vec<K>,
    vals: Vec<V>
}

impl<K,V> MyHashMap<K,V> {
    fn new() -> Self {
        MyHashMap {
            keys: vec![],
            vals: vec![]
        }
    }
    fn insert(&mut self, k: K, v: V) {
        self.keys.push(k);
        self.vals.push(v)
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized
    {
        let mut found = None;
        for (index, k) in self.keys.iter().enumerate() {
            if k.borrow() == key {
                found = Some(&self.vals[index])
            }
        }
        found
    }
}

impl<K, V> Default for MyHashMap<K,V> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut hm: MyHashMap<MyBox<&str>, MyBox<&str>> = MyHashMap::new();
    hm.insert(MyBox("key"), MyBox("value"));
    println!("{:?}", hm.get("key"));
}
```

<br>

### Example 2
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

The main difference: `AsRef` is used with concrete types for **type conversion**, `Borrow` is used with genereics.<br>
Also `Borrow` is **stricter** than `AsRef`: if type `T` implements `Borrow` it means that `Eq`, `Ord` and `Hash` are **equivalent** for `&T`, in other words, `&T` **hashes** and **compares** the same way as the value of `T`.<br>
Use `Borrow` and `BorrowMut` if `Eq`, `Ord` and `Hash` must be **equivalent** for **borrowed** and **owned** values: `x.borrow() == y.borrow()` should give the same result as `x == y`.

<br>

### Example: CaseInsensitiveStr
Consider type `CaseInsensitiveStr`:
```Rust
#[derive(Debug)]
pub struct CaseInsensitiveStr(&'static str);

impl PartialEq for CaseInsensitiveStr {
    fn eq(&self, other: &Self) -> bool {
        // Note that the comparison here is required to ignore ascii case
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl Eq for CaseInsensitiveStr { }
```

<br>

`CaseInsensitiveStr("OK")` is **equal** to `CaseInsensitiveStr("ok")`.

<br>

**Question**: is it correct to implement `Borrow<CaseInsensitiveStr>` for `MyBox<CaseInsensitiveStr>`?<br>

To answer the question above consider following code:
```Rust
use std::borrow::Borrow;

#[derive(Debug)]
struct MyBox<T>(T);

impl Borrow<str> for MyBox<&str> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<CaseInsensitiveStr> for MyBox<CaseInsensitiveStr> {
    fn borrow(&self) -> &CaseInsensitiveStr {
        &self.0
    }
}

#[derive(Debug)]
pub struct CaseInsensitiveStr(&'static str);

impl PartialEq for CaseInsensitiveStr {
    fn eq(&self, other: &Self) -> bool {
        // Note that the comparison here is required to ignore ascii case
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl Eq for CaseInsensitiveStr { }

struct MyHashMap<K, V> {
    keys: Vec<K>,
    vals: Vec<V>
}

impl<K,V> MyHashMap<K,V> {
    fn new() -> Self {
        MyHashMap {
            keys: vec![],
            vals: vec![]
        }
    }
    fn insert(&mut self, k: K, v: V) {
        self.keys.push(k);
        self.vals.push(v)
    }

    fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + ?Sized
    {
        let mut found = None;
        for (index, k) in self.keys.iter().enumerate() {
            if k.borrow() == key {
                found = Some(&self.vals[index])
            }
        }
        found
    }
}

impl<K, V> Default for MyHashMap<K,V> {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    assert_eq!(CaseInsensitiveStr("KEY"), CaseInsensitiveStr("key"));

    let mut hm: MyHashMap<MyBox<CaseInsensitiveStr>, MyBox<&str>> = MyHashMap::new();
    hm.insert(MyBox(CaseInsensitiveStr("key")), MyBox("value"));
    println!("{:?}", hm.get(&CaseInsensitiveStr("key")));
    println!("{:?}", hm.get(&CaseInsensitiveStr("KE")));
    println!("{:?}", hm.get(&CaseInsensitiveStr("KEY")));
}
```

<br>

**Output**:
```bash
~/Projects/play-rust [master] % cargo run
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/ololo`
Some(MyBox("value"))
None
Some(MyBox("value"))
```

<br>

`CaseInsensitiveStr` **cannot** be used as a key for a `HashMap` and `Borrow<CaseInsensitiveStr>` must **not** be implemented for `CaseInsensitiveStr` because `hm.get(&CaseInsensitiveStr("key"))` and `hm.get(&CaseInsensitiveStr("KEY"))` return the same object.<br>
But `AsRef` can be implemented for `CaseInsensitiveStr`.<br>

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
