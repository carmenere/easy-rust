# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Example](#example)

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Display`|[std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)|

<br>

# Declaration
```rust
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result;
}
```

<br>

# In a nutshell
`Debug` and `Display` traits are part of the `std::fmt` module.<br>
The `Display` trait is for **custom output** and so **cannot** be **derived**. It formats the value using the **given formatter**.<br>
Implementing `Display` trait will **automatically implement** `ToString`.<br>

<br>

# Example
```Rust
use core::fmt;
use std::fmt::Display;

struct Foo {
    a: i32
}

impl Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Foo.a: {}", self.a)
    }
}

fn main() {
    let x = Foo { a: 1 };
    println!("{}", x);
    println!("{}", x.to_string());
}
```
