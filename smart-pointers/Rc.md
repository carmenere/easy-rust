# `Rc<T>`
In the majority of cases, ownership is clear: you know exactly which variable owns a given value.<br>
However, there are cases when a single value might have multiple owners.<br>

To enable **multiple ownership** there is type `Rc<T>` in Rust, **Rc** is abbreviation for **reference counting**.<br>

In other words, we use the `Rc<T>` type when we want to allocate some data on the heap for **multiple** parts of our program to read and we **can’t** determine *at compile time* which part will finish using the data last.<br>

**Note** that `Rc<T>` is only for use in **single-threaded** scenarios.<br>

<br>

### Example
```Rust
use std::rc::Rc;

fn main() {
    let s = Rc::new("ABC".to_string());
    let s2 = s.clone();
}
```

<br>

## Cloning an Rc<T>
**Cloning** an ``Rc<T>`` **increases** the **reference count**: ``.clone()`` method called on ``Rc<T>`` types **doesn’t** **clone** **value**, instead it simply creates another pointer to it and **increment the reference counter**.

A value owned by ``Rc`` pointer is **immutable**.

<br>

## Reference counting loop
**Reference counting loop** is a situation when **two** ``Rc<T>`` values **point** to **each other**, reference counter will always above zero and the values will never be freed.<br>
**Reference counting loop** is **available** when **interior mutability** is used with ``Rc<T>``.<br>
To **avoid** *reference counting loop* there is special type ``std::rc::Weak`` in Rust.
