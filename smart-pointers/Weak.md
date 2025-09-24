# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
  - [`RcBox`](#rcbox)
- [Reference counting loops](#reference-counting-loops)
  - [Example1](#example1)
  - [Example2](#example2)
- [`Weak`](#weak)
- [Examples](#examples)
  - [Bypassing reference counting loop](#bypassing-reference-counting-loop)
<!-- TOC -->

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

# Reference counting loops
**Reference counting loop** (aka **reference cycle**) is a situation when **two** `Rc<T>` or `Arc<T>` instances **point** to **each other**, *reference counter* will **always** above zero and the values will **never** be freed.<br>
**Reference counting loop** is **available** when **interior mutability** is used with `Rc<T>`.<br>
To **avoid** *reference counting loop* there is special type [Weak](./Weak.md) in Rust.

<br>

## Example1
Consider example when linked list that is looped: `a -> b -> c -> a`.<br>

![ref_cycle](/img/ref_cycle.png)

<br>

1. After `c` goes out of scope the state of list will be as follows:
![ref_cycle](/img/drop_c.png)

<br>

1. Then after `b` goes out of scope the state of list will be as follows:
![ref_cycle](/img/drop_b.png)

<br>

3. Then after `a` goes out of scope the state of list will be as follows:
![ref_cycle](/img/drop_a.png)

<br>

Consider there is **no** ref from `c` to `a`.<br>
Then after `a` had destroyed, its **strong** filed would become **0** and heap allocated value also would be dropped and its in turn trigger drop `Rc` in its **next** filed.<br>
This in turn **decrements** strong of `b` and so on.<br>
As result all nodes of list will be droped.<br>

<br>

## Example2
```Rust
use std::{cell::RefCell, sync::Arc};

impl Drop for Data {
    fn drop(&mut self) {
        println!("data {} drops here", self.x);
    }
}
pub struct Data {
    pub x: usize,
    pub next: Option<Arc<RefCell<Data>>>,
}

fn main() {
    let data1 = RefCell::new(Data { x: 42, next: None });
    let data2 = RefCell::new(Data { x: 53, next: None });
    let ptr1 = Arc::new(data1);
    let ptr2 = Arc::new(data2);
    ptr1.borrow_mut().next = Some(ptr2.clone());
    ptr2.borrow_mut().next = Some(ptr1.clone());
    println!("Hello, world!");
}
```

The output is just a line of `Hello, world!`, and **no** drops happen here.<br>
That's becase even the pointers `ptr1` and `ptr2` are **dropped** at the end of the main function, **but** the reference counters of `data1` and `data2` are still **1**, not decresed to **0**.<br>

<br>

Here: `c.next -> b.next -> a.next -> c`.<br>

<br>

The `strong_count` for `a`, `b`, and `c` is `2`.<br>
To drop a value inside `Rc` instance, we must ensure that its `strong_count` is equal to `0`.<br>
At the end of `main`, variables `a`, `b` and `c` are dropped, the `strong_count` of these 3 variable is decreased to `1`.<br>
But the heap memory of `Rc` (the **original value**) won't be dropped since the reference count is `1`. It is **memory leak**.

<br>

# `Weak`
The `Weak` type can be used to **break cycles**: a tree could use `Rc` or `Arc` **from parent to children**, and `Weak` pointer **from children to their parents**. Then,
dropping of a parent node is not prevented through the existence of its child nodes.<br>

Usage:
- to obtain `Weak` pointer call `downgrade()` method on `Rc` or `Arc`. Every instance of `Weak` increases the `weak_count` by `1`;
- to access the `T` through `Weak<T>`, it can be **upgraded** to an `Arc<T>`/`Rc<T>` or `Rc<T>` by `Weak::upgrade()` method;

<br>

A `T` can be shared between several `Arc<T>`/`Rc<T>` and `Weak<T>` objects, but when **all** `Arc<T>`/`Rc<T>` objects are **gone**, the `T` is **dropped**, **regardless** of whether there are any `Weak<T>` objects left. This means that a `Weak<T>` can exist without a `T`.<br>

So, the `Weak` reference **does not** prevent the value stored in the allocation from being dropped, and `Weak` makes **no guarantees** about the **value still being present**.<br>

That's why `Weak::upgrade()` method returns an `Option<Arc<T>>` or `Option<Rc<T>>`: the `None` means that the `T` **has already been dropped**.<br>

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
