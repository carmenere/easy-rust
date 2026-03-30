# ``HashMap``
#### Example
```Rust
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct TwoStrings{
    a: String,
    b: String
}

pub fn main() {
    let mut map : HashMap<TwoStrings, i32> = HashMap::new();
    
    let key_one = TwoStrings{ a: "a".to_owned(), b: "b".to_owned() };
    let key_two = TwoStrings{ a: "c".to_owned(), b: "d".to_owned() };
    
    map.insert(key_one.clone(), 2i32);
    map.insert(key_two.clone(), 2i32);
    
    let entry = map.get(&key_one); //retrieve from the hashmap
}
```