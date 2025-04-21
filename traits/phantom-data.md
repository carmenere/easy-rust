# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [`PhantomData`](#phantomdata)
* [Example](#example)
<!-- TOC -->

<br>

# `PhantomData`
Sometimes we have a situation when **type var** or **lifetimes** are **logically associated with a struct**, but **not** actually part of a field.<br>

Consider example:
```Rust
struct Tagged<T>(usize);
```

**Output**:
```bash
error[E0392]: parameter `T` is never used
 --> src/main.rs:1:15
  |
1 | struct Tagged<T>(usize);
  |               ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`
```

That’s right, compiler **does not** allow to have a **type var** that is **unused** within **struct** or **enum**.<br>

There is special type `PhantomData` that **allows to declare unused lifetimes** or **type vars** for structs/enums.<br>

`PhantomData` is **marker type**. It is **ZST**, in other words **it consumes no space at runtime**.<br>
Adding a `PhantomData<T>` to your type tells the compiler that your type acts like it stores a value of type `T`, even though it doesn’t really.<br>

Also `PhantomData` tells to **drop checker** that the _defined type_ **owns value of generic type** `T`.<br>

<br>

From the compiler point of view type `A` **doesn't** own type `T` in the code below:
```rust
struct A<T> {
    data: *const T
}
```

<br>

But if we add `PhantomData<T>`, then **drop checker** consider that **type** `A` **owns** `T`:
```rust
struct A<T> {
    data: *const T,
    _d: PhantomData<T>,
}
```

<br>

# Example
```Rust
use std::marker::PhantomData;

struct author_id;
struct post_id;

struct ID<P>(i64, PhantomData<P>);

fn get_post(aid: ID<author_id>, pid: ID<post_id>) -> String {
    format!("{aid}, {pid}", aid=aid.0, pid=pid.0)
}

fn main() {
    let id1: ID<author_id> = ID(5, PhantomData);
    let id2: ID<post_id> = ID(77, PhantomData);
    println!("{}", get_post(id1, id2));
    
    // println!("{}", get_post(id2, id1));  // ERROR
}
```