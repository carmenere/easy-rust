# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [`Rc`](#rc)
- [Weak reference](#weak-reference)
- [Avoiding lifetime annotations with `Rc`](#avoiding-lifetime-annotations-with-rc)
<!-- TOC -->

<br>

# `Rc`
`Rc` stands for **reference counter** or **reference counted**.<br>
In Rust, **every variable can only have one owner**:
```rust
fn takes_a_string(foo: String) {
    println!("{}", foo);
}

fn main() {
    let user_name = String::from("User MacUserson");
    takes_a_string(user_name);
    takes_a_string(user_name); // ❌ ERROR: use of moved value: `user_name`
}
```

After `takes_a_string` takes `user_name`, you **can’t use it anymore**. You can give it `user_name.clone()`. However, sometimes
- the `String` is part of a struct, and you can’t clone the struct;
- the `String` is really **long**, and you don’t want to clone it;

<br>

An `Rc` gets around this by letting you **have more than one owner**:
```rust
use std::rc::Rc;

fn takes_a_string(foo: Rc<String>) {
    println!("{}", foo);
    println!("takes_a_string: The number of owners: {}", Rc::strong_count(&foo));
}

fn main() {
    let foo = Rc::new(String::from("Foo"));
    takes_a_string(Rc::clone(&foo));
    takes_a_string(Rc::clone(&foo));
    println!("main: The number of owners: {}", Rc::strong_count(&foo));
}
```
**Output**:
```rust
Foo
takes_a_string: The number of owners: 2
Foo
takes_a_string: The number of owners: 2
main: The number of owners: 1
```

<br>

# Weak reference
Weak pointers are useful because if two `Rc`s point at each other, they **can’t be deallocated**. This is called a **reference cycle**.<br>
If *item 1* has an `Rc` to *item 2* and *item 2* has an `Rc` to *item 1*, they can’t get **strong count** to `0` and will never be able to drop their values. In this case, you can to use **weak references**.<br>

There is `Rc::downgrade(&item)` instead of `Rc::clone(&item)` to make **weak references**. Also, there is `Rc::weak_count(&item)` to see the **weak count**.<br>


Consider you have instance of `Rc` in variable `foo`, then you can **clone** `Rc` with `Rc::clone(&foo)` or `foo.clone()`. Usually, `Rc::clone(&foo)` is **better** because an `Rc` holds a type that might have its own methods (including `.clone()`). Thus, it’s a good way to show that you are cloning the `Rc`, not the object inside it.<br>

<br>

There is also a method for `Rc` called `strong_count()` that shows you how many owners `Rc` instance has at the moment.<br>

<br>

# Avoiding lifetime annotations with `Rc`
Using lifetimes in struct forces to declare them everywhere.<br>

<br>

**Consider struct**:
```rust
struct Foo<'a> {
    name: &'a str,
}
```

Then Rust will require to add `'a` to every struct that somehow uses `Foo`:
```rust
struct Bar<'a> {
    foo: Foo<'a>,
}

struct FizzBaz<'a> {
    foo_list: Vec<Foo<'a>>,
}
```

That works just fine, but it took **a lot of typing**.<br>

It is possible to use `Rc` instead and *get rid of* **all** the lifetime annotations **everywhere** else:
```rust
use std::rc::Rc;

struct Foo {
    name: Rc<String>,
}

struct Bar {
    foo: Foo,
}

struct FizzBaz {
    foo_list: Vec<Foo>,
}
```

<br>