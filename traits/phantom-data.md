# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [`PhantomData`](#phantomdata)
* [Example](#example)
<!-- TOC -->

<br>

# `PhantomData`
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

That’s right, we’re **not** allowed to have a **type parameter** that goes **unused**.<br>

<br>

If we want to have an **unused type parameter** we **have to** add a `PhantomData` to it like so:
```Rust
use std::marker::PhantomData;
struct Tagged<T>(usize, PhantomData<T>);
```

<br>

Declaration of `PhantomData` in `core`:
```Rust
pub struct PhantomData<T: ?Sized>;
```

<br>

`PhantomData` is **marker type**. It is **ZST**.<br>
Adding a `PhantomData<T>` to your type tells the compiler that your type acts like it stores a value of type `T`, even though it doesn’t really.<br>

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