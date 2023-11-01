# Example: own hashmap implementation
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