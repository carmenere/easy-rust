# Example: CaseInsensitiveStr
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