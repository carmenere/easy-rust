# ``Arc<T>``
`Arc` stands for **Atomic Reference Counter**.<br>
It's **threadsafe** version of `Rc`.<br>
But, **Rc** is **faster** than **Arc**.

<br>

```Rust
use std::sync::Arc;
```

<br>

# `Rc<T>`
`Rc` stands for **Reference Counter**.<br>
The `Rc<T>` type provides **shared ownership** of some **value** of type `T`, allocated in the **heap**.<br>
The `Rc<T>` type is useful when we **can’t** determine which scope will destroy value *at compile time*.<br>

> Note:<br>
> A value owned by `Rc` pointer is **immutable**, i.e., `Rc` **can't** return a **mutable reference** (`&mut T`).<br>
> `Rc` is **not thread-safe**, it uses **non-atomic** reference counting.<br>
> `Rc` **can't** be **sent between threads**, therefore `Rc` **doesn't** implement `Send`.<br>

<br>

## Cloning an Rc<T>
The `Rc` **keeps track** of the **number of references** to **original value** it wraps.<br>
`.clone()` method called on `Rc<T>` instance or `Rc::clone()` function applied to `Rc<T>` instance both **don’t clone original value** of type `T`, instead they simply create **new pointer** to **value** of type `T` and increment the **strong_count**.<br>
When instance of `Rc` **goes out of scope** it is destroyed and the **strong_count** is decremented by `1`.<br>
When the **strong_count** is reached `0` the **original value** of type `T` is also **dropped**.<br>

<br>

### Example
```Rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```



<br>

## Reference counting loops (aka reference cycles)
**Reference counting loop** (aka **reference cycle**) is a situation when **two** `Rc<T>` instances **point** to **each other**, reference counter will always above zero and the values will never be freed.<br>
**Reference counting loop** is **available** when **interior mutability** is used with `Rc<T>`.<br>
To **avoid** *reference counting loop* there is special type `std::rc::Weak` in Rust.

<br>

### Not working example
```Rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node {next: None}));
    println!("a count: {:?}",  Rc::strong_count(&a));
    let b = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&a))}));
    println!("a count: {:?}",  Rc::strong_count(&a));
    println!("b count: {:?}",  Rc::strong_count(&b));
    let c = Rc::new(RefCell::new(Node {next: Some(Rc::clone(&b))}));

    // Creates a reference cycle
    (*a).borrow_mut().next = Some(Rc::clone(&c));
    println!("a count: {:?}",  Rc::strong_count(&a));
    println!("b count: {:?}",  Rc::strong_count(&b));
    println!("c count: {:?}",  Rc::strong_count(&c));

    // Print a will casue stack overlfow
    // println!("a {:?}",  &a);
}
```

<br>

Here: `c.next -> b.next -> a.next -> c`.<br>

<br>

The `strong_count` for `a`, `b`, and `c` is `2`.<br>
To drop a value inside `Rc` instance, we must ensure that its `strong_count` is equal to `0`.<br>
At the end of `main`, variables `a`, `b` and `c` are dropped, the `strong_count` of these 3 variable is decreased to `1`. But the heap memory of `Rc` (the **original value**) won't be dropped since the reference count is `1`. It is **memory leak**.

<br>

# `Weak` pinter to prevent reference cycles
`Weak` pointer can be created by calling `Rc::downgrade`, and it increases the `weak_count` by `1`.<br>
the `weak_count` **doesn’t** need to be `0` for the `Rc<T>` instance to **drop original value**.<br>
A use case for `Weak`: a tree could use `Rc` **from parent to children**, and `Weak` pointer **from children to their parents**.

<br>

# Working example
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

<br>

The **original value** is accessed by calling `.upgrade()` method on the `Weak` pointer or `Rc::upgrade()` function applied to the `Weak` pointer, which returns an `Option<Rc<T>>`.<br>
`Weak` reference **does not** prevent the value stored in the allocation from being dropped, and `Weak` itself makes **no guarantees** about the **value still being present**.<br>
Thus it may return `None` when upgraded.<br>
Note however that a `Weak` reference **prevents** itself from being deallocated.