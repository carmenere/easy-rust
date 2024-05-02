# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
  - [`RcBox`](#rcbox)
- [In a nutshell](#in-a-nutshell)
- [Examples](#examples)
  - [Bypassing reference counting loop](#bypassing-reference-counting-loop)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Weak`|[**std::rc::Weak**](https://doc.rust-lang.org/stable/std/rc/struct.Weak.html)|

<br>

# Declaration
## `RcBox`
```rust
pub struct Weak<T, A = Global>
where
    A: Allocator,
    T: ?Sized
{
    ptr: NonNull<RcBox<T>>,
    alloc: A,
}
```

<br>

# In a nutshell
`Weak` pointer can be created by calling `Rc::downgrade()`, and it increases the `weak_count` by `1`.<br>
The `weak_count` **doesn’t** need to be `0` for the `Rc<T>` instance to **drop original value**.<br>
The **original value** is accessed by calling `.upgrade()` method on the `Weak` pointer which returns an `Option<Rc<T>>`.<br>
`Weak` reference **does not** prevent the value stored in the allocation from being dropped, and `Weak` itself makes **no guarantees** about the **value still being present**.<br>
Thus it may return `None` when upgraded.<br>

A use case for `Weak`: a tree could use `Rc` **from parent to children**, and `Weak` pointer **from children to their parents**.<br>

<br>

# Examples
## Bypassing reference counting loop
```Rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node {next: None, head: None}));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    let b = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&a)), head: None}));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    let c = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&b)), head: None}));

    // Creates a reference cycle
    (*a).borrow_mut().head = Some(Rc::downgrade(&c));
    println!("a strong count: {:?}, weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count: {:?}, weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c strong count: {:?}, weak count: {:?}", Rc::strong_count(&c), Rc::weak_count(&c));

    println!("a {:?}",  &a);
}
```
