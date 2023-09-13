# Debug and Display
`Debug` and `Display` traits are part of the `std::fmt` module.<br>
`Debug` is **derivable** trait, but `Display` **not** and have to manually write the implementation.<br>
Implementing `Display` trait will **automatically implement** `ToString`.<br>

<br>

#### Example: Display
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

<br>

# ToString
`ToString` trait provides method `.to_string()` to convert **value** to a `String`.<br>
`ToString` trait is **automatically implemented** for any type that implements `Display`.
