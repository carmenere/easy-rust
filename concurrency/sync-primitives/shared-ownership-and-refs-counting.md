# Table of contents
- [Table of contents](#table-of-contents)
- [Shared ownership](#shared-ownership)
  - [Statics](#statics)
  - [Leaking](#leaking)
  - [Reference counting](#reference-counting)
    - [Example: `Rc`](#example-rc)
    - [Example: `Arc`](#example-arc)

<br>

# Shared ownership
When sharing data between threads where **neither** thread is guaranteed to outlive the other, **neither** of them can be the owner of that data. Any data shared between them will need to live as long as the longest living thread.<br>

There are 3 approaches to create something that is **not owned by a single thread**:
- statics;
- leaking;
- reference counting;

<br>

## Statics
The `static` value is **owned** by the **entire program**, instead of an individual thread. A `static` value has a constant initializer, is **never dropped** and already exists before the `main` function. Every thread can borrow it, since it has guaranteed to always exist.<br>

Example:
```rust
use std::thread;

static Foo: [u64; 5] = [1,2,3,4,5];

fn main() {
    thread::spawn(|| dbg!(&Foo));
    thread::spawn(|| dbg!(&Foo));
}
```

<br>

## Leaking
Another way to share ownership is by **leaking an allocation**. Using `Box::leak()`, one can release ownership of a `Box`, promising to never drop it. In other words, the `Box` will live forever, without an owner.

Example:
```rust
use std::thread;

fn main() {
    let foo: &'static [u64; 3] = Box::leak(Box::new([1,2,3]));
    thread::spawn(move || dbg!(foo));
    thread::spawn(move || dbg!(foo));
}
```

Reference is a `Copy`, meaning that whet it is **moved**, the original still exists.<br>

<br>

## Reference counting
Another way to share ownership is by **tracking the number of owners**, we can make sure the value is dropped only when there are **no owners left**. The Rust standard library provides this functionality through the `std::rc::Rc` and `std::sync::Arc` types. The `Rc` is **not** thread safe. Both `Rc` and `Arc` do **not** give you **mutable access** to their contained values.<br>
**Cloning** `Rc` or `Arc` **increment** a counter. Both the **original** and **cloned** value will refer to the **same allocation**, the **share ownership**.<br>
**Dropping** the value will **decrement** the counter. Only the **last instance** of `Rc` or `Arc`, which will see the counter equal **zero** will **dealocate** containing value.<br>

<br>

### Example: `Rc`
```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new([1,2,3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
}
```

<br>

### Example: `Arc`
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let a = Arc::new([1,2,3]);
    let b = a.clone();

    thread::spawn(move || {
        dbg!(b)
    });
}
```

Rust allows us to shadow variables by defining a new variable with the same name. If you shadow variable in the same scope the original variable is no longer avaliable in this scope. But the same name can be **reused** within **new scope** `{}`, while leaving the original variable avaliable out of scope.<br>

By **wrapping a closure in a new scope** (with `{}`) we can clone variables before moving them into the closure, without having to rename them:
```rust
use std::thread;
use std::sync::Arc;

fn main() {
    let a = Arc::new([1,2,3]);
    let b = a.clone();

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a)
        }
    });
    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a)
        }
    });
}
```
