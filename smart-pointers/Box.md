# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declaration](#declaration)
* [In a nutshell](#in-a-nutshell)
* [Box memory layout](#box-memory-layout)
* [Examples](#examples)
  * [Custom implementation of Box](#custom-implementation-of-box)
<!-- TOC -->

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Box`|[**std::boxed::Box**](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html)|

<br>

# Declaration
```rust
pub struct Box<T: ?Sized,A: Allocator = Global>(Unique<T>, A);
```

```rust
impl<T> Box<T> {
    pub fn new(x: T) -> Self {
        #[rustc_box]
        Box::new(x)
    }
}
```

**Note**, that `Box` **uses** `Unique`.<br>

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

A `Box` can be useful when you:
- have a type whose size cannot be known at compile time;
- want to transfer ownership of a large amount of data, to avoid copying large amounts of data on the stack, instead store the data on the heap in a `Box`, so only pointer is moved;

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

<br>

# Box memory layout
Consider example:
```rust
fn main() {
  let vec = vec![1.0, 2.0, 3.0];
  let foo = Box::new(vec);
}
```

<br>

It will be represented in memory as follows:<br>
![box](/img/box.png)

<br>

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


