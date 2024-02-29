# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Display](#display)
  - [Example: Display](#example-display)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Display`|[std::fmt::Display](https://doc.rust-lang.org/std/fmt/trait.Display.html)|

<br>

# Display

`Debug` is **derivable** trait, but `Display` **not** and have to manually write the implementation.<br>
Implementing `Display` trait will **automatically implement** `ToString`.<br>

<br>

## Example: Display
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
