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

## Variant 2: map + collect + or_insert
```Rust
use std::{collections::HashMap};

fn main() {
    let d = "Some sentence where word foo occurs exact one time. Here we see word foo again.";
    let mut hm: HashMap<String, u16> = HashMap::with_capacity(16);

    let _: Vec<()> = d.split_whitespace().map(|v| { *hm.entry(v.to_string()).or_insert(0) += 1 }).collect();
    
    println!("{:?}", hm);
}
```

<br>

## Variant 3: for_each + or_insert
```Rust
use std::{collections::HashMap};

fn main() {
    let d = "Some sentence where word foo occurs exact one time. Here we see word foo again.";
    let mut hm: HashMap<String, u16> = HashMap::with_capacity(16);

    d.split_whitespace().for_each(|v| { *hm.entry(v.to_string()).or_insert(0) += 1 });
    
    println!("{:?}", hm);
}
```

<br>

## Variant 4: for_each + and_modify
```Rust
use std::{collections::HashMap};

fn main() {
    let d = "Some sentence where word foo occurs exact one time. Here we see word foo again.";
    let mut hm: HashMap<String, u16> = HashMap::with_capacity(16);

    d.split_whitespace().for_each(|v| { hm.entry(v.to_string()).and_modify(|v| *v += 1).or_insert(1);});
    
    println!("{:?}", hm);
}
```