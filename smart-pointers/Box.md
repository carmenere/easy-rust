# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Box](#box)
- [Implementation of Box](#implementation-of-box)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Box`|[std::boxed::Box](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html)|

<br>

# Box<T>
```Rust
fn main() {
    let v = Box::new(1);
    println!("v = {}", v);
}
```

Notes:
- The value (`1`) is allocated on the **heap**.
- The data in the **box** is accessed the same way as the data were on the stack. 
- When a `Box` **goes out of scope**, the value of `Box` type (`v`) and the value it points to (`1`) are both dealocated.

<br>

# Implementation of Box
Under the hood `Box` can be represented as:
```Rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// To support dereference: 
//   let v = MyBox::new(1); 
//   let r = *v;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```


