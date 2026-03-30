# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declarations](#declarations)
  * [`RcInner<T>`](#rcinnert)
  * [`Rc<T>`](#rct)
* [Rc memory layout](#rc-memory-layout)
* [In a nutshell](#in-a-nutshell)
  * [Example](#example)
* [Cloning](#cloning)
* [Deref](#deref)
<!-- TOC -->

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Rc`|[**std::rc::Rc**](https://doc.rust-lang.org/stable/std/rc/struct.Rc.html)|

<br>

# Declarations
## `RcInner<T>`
```rust
#[repr(C)]
struct RcInner<T: ?Sized> {
  strong: Cell<usize>,
  weak: Cell<usize>,
  value: T,
}
```

**Note**, that `#[repr(C)]` attribute is used for `RcInner`.<br>

<br>

## `Rc<T>`
```rust
pub struct Rc<
  T: ?Sized,
  A: Allocator = Global,
> {
  ptr: NonNull<RcInner<T>>,
  phantom: PhantomData<RcInner<T>>,
  alloc: A,
}
```

**Note**, that `Rc` **uses** `NonNull`.<br>

<br>

# Rc memory layout
Consider example:
```rust
use std::rc::Rc;

fn main() {
  let vec = vec![1.0, 2.0, 3.0];
  let foo = Rc::new(vec);
  let a = Rc::clone(&foo);
  let b = Rc::clone(&foo);
}
```

<br>

It will be represented in memory as follows:<br>
![rc](/img/rc.png)

<br>

# In a nutshell
The `Rc` stands for **Reference Counted**.<br>
The `Rc<T>` is just a **pointer** on the **stack** that **points to** `RcInner<T>` which is allocated in the **heap**:
```rust
use std::rc::Rc;

fn main() {
    println!("size_of::<Rc<u16>>: {}", size_of::<Rc<u16>>());
    println!("size_of::<Rc<String>>: {}", size_of::<Rc<String>>());
}
```

<br>

**Output**:
```shell
size_of::<Rc<u16>>: 8
size_of::<Rc<String>>: 8
```

The `Rc<T>` type is **thread-unsafe reference-counting pointer**. It uses **non-atomic reference counting**.<br>
The `Rc<T>` type **keeps track** of the **number of references** to **original value** it wraps.<br>
The `Rc<T>` type is useful when we **can’t determine** *at compile time* in which **scope** the **value** `T` will be **destroyed**.<br>

To avoid names clashes with `T`'s methods, all *methods* of `Rc` are **associated functions** and they must be called using **fully qualified syntax**, example: `Rc::get_mut(...)`.<br>

The `Rc` **can't** be **sent between threads**, therefore `Rc<T>` implements `!Send` and `!Sync`:
```rust
impl<T: ?Sized, A: Allocator> !Send for Rc<T, A> {}
impl<T: ?Sized, A: Allocator> !Sync for Rc<T, A> {}
```

<br>

## Example
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

# Cloning
`Rc`'s implementation of `Clone` trait may be called using **fully qualified syntax** or **method-call syntax**:
- `rc.clone();`
- `Rc::clone(&rc);`

The `Rc::clone()` **doesn't clone original wrapped value** of type `T`, instead it creates new instance of `Rc<T>` and **increments** the **strong_count**.<br>
When instance of `Rc` **goes out of scope** it is destroyed and the **strong_count** is **decremented** by `1`.<br>
When the **strong_count** is reached `0` the **original value** of type `T` is also **dropped**.<br>

<br>

# Deref
The `Rc` implements `Deref` trait, so you can call `T`'s methods on a value of type `Rc<T>`.
