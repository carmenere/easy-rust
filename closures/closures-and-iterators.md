# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Closures](#closures)
  - [Syntax](#syntax)
    - [Notation](#notation)
    - [Closure with args](#closure-with-args)
    - [Closure without args](#closure-without-args)
    - [Compare to functions](#compare-to-functions)
  - [Types of closures](#types-of-closures)
  - [The relationship between `FnOnce`, `FnMut`, and `Fn`](#the-relationship-between-fnonce-fnmut-and-fn)
  - [Examples of how Rust infers type of closure](#examples-of-how-rust-infers-type-of-closure)
- [Closure type](#closure-type)
- [Returning closures](#returning-closures)
- [`move` keyword](#move-keyword)
    - [Example](#example)
    - [Example](#example-1)
- [How the compiler implements closures](#how-the-compiler-implements-closures)
- [Another examples](#another-examples)
    - [Function that accepts one closures](#function-that-accepts-one-closures)
    - [Function that accepts two closures](#function-that-accepts-two-closures)
<!-- TOC -->

<br>

# Iterators and loops
A `for` loop can receive **iterator** or **iterable** (because every **iterator** is **iterable**):
- `.into_iter()` for an **iterator of owned values**;
  - the `for num in vector` is the same as writing `for num in vector1.into_iter()` - it iterates over **owned values**, and `vector1` **no longer exists** after this for loop is done;
- `.iter()` for an **iterator of references**;
  - the `for num in mut vector` is the same as writing `for num in vector.iter()` - it iterates over **immutable references**, so `vector` **still exists** after it is over;
- `.iter_mut()` for an **iterator of mutable references**;
  - the `for num in &mut vector` is the same as writing `for num in vector.iter_mut()` - it iterates over **mutable references**, so `vector` **still exists** after it is over;

<br>

The core of every iterator is a method called `.next()`, which returns an `Option`. When you use an iterator, it calls `.next()` over and over:
- if `.next()` returns `Some`, there are still items left, and the iterator **keeps going**;
- if `None` is returned, the iteration is **finished**;

An iterator gives out a bunch of `Somes` until it is out of items, and then it **only** gives `None`. This is how the `for` loop knows when to stop.<br>

If you wish, you can also **manually call** `.next()` on an iterator:
```rust
fn main() {
    let my_vec = vec!['a', 'b', 'c', 'd'];
    let mut my_vec_iter = my_vec.iter();
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'c'));
    assert_eq!(my_vec_iter.next(), Some(&'d'));
    assert_eq!(my_vec_iter.next(), None); // Now the iterator is out of items, so it returns None.
    assert_eq!(my_vec_iter.next(), None); // You can keep calling .next() on the iterator, and it will simply return None every time.
}
```

<br>

It is possible to make iterators that **never** return `None`, **only** return `None`, and so on.<br>

Here’s an iterator that just gives the number 1 forever:
```rust
struct GivesOne;
impl Iterator for GivesOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

fn main() {
    let mut my_vec_iter = GivesOne;

    assert_eq!(my_vec_iter.next(), Some(1));
    assert_eq!(my_vec_iter.next(), Some(1));
    assert_eq!(my_vec_iter.next(), None); // this panics!
}
```

But you can use the `.take(N)` method to only call it `N` times.<br>

**Note** that the `GivesOne` struct **doesn’t hold anything**. It’s a good example of one of the ways that an *iterator* **differs** from a *collections*. In this case, the `GivesOne` struct is just an **empty struct** that implements the `Iterator` trait.<br>

<br>

# Closures and closures inside iterators
**Closures** are functions that don’t need a name — in other words, **anonymous functions**. Sometimes they are called **lambdas** in other languages.<br>

You can **bind a closure to a variable**, and then it looks exactly like a function when you use it:
```rust
fn main() {
    let my_closure = || println!("This is a closure");
    my_closure();
}
```

In the above example closure takes nothing: `||`.<br>

In between the `||`, we can add **signature of closure** for its **input variables**:
```rust
fn main() {
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5+5);
}
```

<br>

For longer *closures*, you can add a **code block**: `|| {}`.<br>
Also you can add a **returning value** in the signature: `|| -> u64 {}` or `|x: i32| -> u64 {}`.<br>

One thing that makes *closures* special is that they **can capture variables** from their environment that are outside the closure, even if you only write `||`.<br>
You can think of a closure as a standalone type that can hold references in the same way that a struct can.<br>

<br>

Usually you see *closures* in Rust inside of methods because it is very convenient to have a closure inside. The convenience comes from the fact that the user can write the **body** of the *closure* **differently** each time, *depending on the situation*.<br>

<br>

# Closures: lazy and fast
A classic example of using `.map()` to make a new `Vec` from an existing `Vec`:
```rust
fn main() {
    let num_vec = vec![2, 4, 6];
    let double_vec: Vec<i32> = num_vec
    .iter()
    .map(|num| num * 2)
    .collect();
}
```

That was pretty easy and prints `[4, 8, 12]`. But let’s see what happens when we **don’t collect** into a `Vec`: the compiler **issues is a warning**:
```rust
fn main() {
    let num_vec = vec![2, 4, 6];
    num_vec
    .iter()
    .enumerate()
    .map(|(index, num)| format!("Index {index} is {num}"));
}
```
**Output**:
```rust
warning: unused `Map` that must be used
 --> chapter_03/src/main.rs:3:5
  |
3 | /     num_vec
4 | |     .iter()
5 | |     .enumerate()
6 | |     .map(|(index, num)| format!("Index {index} is {num}"));
  | |__________________________________________________________^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = num_vec
  |     +++++++
```

<br>

Consider **chain of 3 methods**:
```rust
num_vec.iter().enumerate().map()
```

<br>

Rust avoids this sort of operation:
- **iterate** over all the items in the `Vec`;
- **enumerate** over all the items from the iterator;
- **map** over all the enumerated i32s;

<br>

Instead, an iterator with a *method* and another *method* and another *method* simply *creates* a **single structure** and waits until we decide what to do with it:
- `let num_vec = vec![2, 4, 6];` it is a `Vec<i32>`;
- `.iter()` now it is an `Iter<i32>`;
- `.enumerate()` now it is an `Enumerate<Iter<i32>>`;
- `.map()` now it is a `Map<Enumerate<Iter<i32>>>`;

If we add `.collect::<Vec<i32>>()`, it knows what to do.<br>

This is one of the ways that Rust keeps even fancy functional-looking code **as fast as** any other kind of code. This is an example of an idea in Rust called **zero-cost abstractions**.<br>

<br>
