# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Intro example](#intro-example)
- [Declarations](#declarations)
  - [`ArcInner<T>`](#arcinnert)
  - [`Arc<T>`](#arct)
- [Arc memory layout](#arc-memory-layout)
- [In a nutshell](#in-a-nutshell)
<!-- TOC -->

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Arc`|[**std::sync::Arc**](https://doc.rust-lang.org/stable/std/sync/struct.Arc.html)|

<br>

# Intro example
We use an `Rc` to give a variable **more than one owner**. If we are doing the **same thing** in a **thread**, we need an `Arc`.<br>
`Arc` stands for **atomic reference counter**. **Atomic** means that it uses **atomic operations**.<br>
You **can’t** change data with just an `Arc`, though — it’s **just a reference counter**. You must wrap the data in a `Mutex`, and then you wrap the `Mutex` in an `Arc`: `Arc<Mutex<T>>`.<br>

**Example**:
```rust
use std::sync::{Arc, Mutex};

fn main() {
    let my_number = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    for _ in 0..2 {
        let my_number = Arc::clone(&my_number);
        threads.push(std::thread::spawn(move || {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        }));
    }
    threads.into_iter().for_each(|h| h.join().unwrap());
    println!("Value is: {my_number:?}");
}
```

<br>

# Declarations
## `ArcInner<T>`
```rust
#[repr(C)]
struct ArcInner<T: ?Sized> {
    strong: atomic::AtomicUsize,
    weak: atomic::AtomicUsize,
    data: T,
}
```

**Note**, that `#[repr(C)]` attribute is used for `ArcInner`.<br>

<br>

## `Arc<T>`
```rust
pub struct Arc<T, A = Global>
where
    A: Allocator,
    T: ?Sized
{
    ptr: NonNull<ArcInner<T>>,
    phantom: PhantomData<ArcInner<T>>,
    alloc: A,
}
```

**Note**, that `Arc` **uses** `NonNull`.<br>

<br>

# Arc memory layout
Consider example:
```rust
use std::sync::Arc;

fn main() {
  let vec = vec![1.0, 2.0, 3.0];
  let foo = Arc::new(vec);
  let a = Arc::clone(&foo);
  let b = Arc::clone(&foo);
}
```

<br>

It will be represented in memory as follows:<br>
![arc](/img/arc.png)

<br>

# In a nutshell
`Arc` stands for **Atomic Reference Counted**.<br>
The `Arc<T>` is just a **pointer** on the **stack** that **points to** `ArcInner<T>` which is allocated in the **heap**:
```rust
use std::sync::Arc;

fn main() {
  println!("size_of::<Arc<u16>>: {}", size_of::<Arc<u16>>());
  println!("size_of::<Arc<String>>: {}", size_of::<Arc<String>>());

}
```

**Output**:
```shell
size_of::<Arc<u16>>: 8
size_of::<Arc<String>>: 8

```

<br>

The `Arc<T>` type is **thread-safe reference-counting pointer**. It uses **atomic reference counting**.<br>
But, **Rc** is **faster** than **Arc**.<br>

The `Arc<T>` type **implements** `Send` and `Sync` if type `T` does:
```rust
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Send> Send for Arc<T, A> {}
unsafe impl<T: ?Sized + Sync + Send, A: Allocator + Sync> Sync for Arc<T, A> {}
```

<br>

