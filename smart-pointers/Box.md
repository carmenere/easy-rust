# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Examples](#examples)
  - [Custom implementation of Box](#custom-implementation-of-box)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Box`|[**std::boxed::Box**](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html)|

<br>

# Declaration
```rust
pub struct Box<T: ?Sized,A: Allocator = Global>(Unique<T>, A);

impl<T> Box<T> {
    pub fn new(x: T) -> Self {
        #[rustc_box]
        Box::new(x)
    }
}
```

<br>

# In a nutshell
Allocates memory on the **heap** and then places value of type `T` into it.<br>

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

The `Box` type **implements** `Send` and `Sync` if `T` does: 
```rust
impl<T: ?Sized, A> Send for Box<T, A>
where
    A: Send,
    T: Send,
```

```rust
impl<T: ?Sized, A> Sync for Box<T, A>
where
    A: Sync,
    T: Sync,
```

# Examples
## Custom implementation of Box
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


