# Example: count words in string
## Variant 1
```Rust
use std::{collections::HashMap};

fn main() {
    let d = "Some sentence where word foo occurs exact one time. Here we see word foo again.";
    let mut hm: HashMap<String, u16> = HashMap::with_capacity(16);

    for word in d.split_whitespace() {
        let v = hm.entry(word.to_string()).or_insert(0);
        *v += 1;
    }
    
    println!("{:?}", hm);
}
```

<br>

## Variant 2
```Rust
use std::{collections::HashMap};

fn main() {
    let d = "Some sentence where word foo occurs exact one time. Here we see word foo again.";
    let mut hm: HashMap<String, u16> = HashMap::with_capacity(16);

    let _: Vec<()> = d.split_whitespace().map(|v| { *hm.entry(v.to_string()).or_insert(0) += 1 }).collect();
    
    println!("{:?}", hm);
}
```