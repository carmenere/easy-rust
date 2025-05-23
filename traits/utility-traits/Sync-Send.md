# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Send and Sync](#send-and-sync)
* [!Send and !Sync](#send-and-sync-1)
* [Send + Sync](#send--sync)
* [Send + !Sync](#send--sync-1)
* [!Send + Sync](#send--sync-2)
* [!Send + !Sync](#send--sync-3)
  * [`Rc<T>`](#rct)
  * [`*mut T`](#mut-t)
  * [`*const T`](#const-t)
<!-- TOC -->
  
<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Send`|[std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html)|
|`Sync`|[std::marker::Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)|

<br>

# Send and Sync
From compiler point of view **thread** is a **scope** `{}`.<br>

**Rule**: type `&T` is `Send` if `T` is `Sync`.<br>

<br>

If type `T` implements `Send` it means that `T` is a **thread safe** and can be passed to another thread, in other words, **can cross a thread boundary**:
- if `T` is a `Copy` it can be **copied** to another thread;
- if `T` is **not** `Copy` it can **only** be **moved** to another thread, in other words, **ownership** of a value of type `T` is transferred to another thread;

<br>

If type `T` implements `Sync` it means that shared reference `&T` is a **thread safe** and can be **copied** to another thread.<br>

<br>

So:
- `Sync`
  - means that **sharing** by **immutable reference** *between threads* is **safe**;
  - allows an object to be used by two threads `A` and `B` at the **same** time;
- `Send`
  - means that **passing** by **value** to *another thread* is **safe**, in other words type `T` it can be **created** in **one** thread (**scope**) and **dropped** in **another** thread (**scope**).
  - allows an object to be used by two threads `A` and `B` at **different** times:
    - thread `A` can **create** and use an object;
    - then object is sent to thread `B` and thread `B` **can** use the object while thread `A` **cannot**;

<br>

Both `Sync` and `Send` are **marker** traits and they both are **unsafe**:
```rust
pub unsafe auto trait Send { }
pub unsafe auto trait Sync { }
```

Both `Sync` and `Send` are [**auto traits**](https://doc.rust-lang.org/reference/special-types-and-traits.html#auto-traits), which means that they are automatically implemented for your types based on their fields. A **struct** in which **all fields** are `Send` and `Sync` is **also** `Send` and `Sync`.<br>

The way to opt out of `Send` or/and `Sync` is to add a field to your type that does not implement the trait. For that purpouse, there is special `std::marker::PhantomData<T>` type.<br>
The `std::marker::PhantomData<T>` is treated by the compiler as a `T`, except it doesn't actually exist at runtime, in other words, it takes no space at runtime.<br>

Example (disable `Sync` for type `X`):
```rust
use std::cell::Cell;
use std::marker::PhantomData;

struct X {
    a: i32,
    _not_sync: PhantomData<Cell<()>>
}

fn main() {

}
```

<br>

# !Send and !Sync
- `!Send` types **cannot** be moved or copied to other threads, i.e., **type is bound to the current thread**;
- `!Sync` types can only be **used** by a **single thread** at any **different** time, since their **references cannot** be moved or copied to other threads. But **instances** of `!Sync` types **can** still be **moved** between threads if they implement `Send`;

<br>

# Send + Sync
**All primitive types** are both `Send` and `Sync`:
- `i8`, `f32`, `bool`, `char`, `&str`, ...;
- `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...;
- `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...;
- `AtomicBool`, `AtomicU8`, ...;
- `Arc<T>`;
- `Mutex<T>`;

<br>

# Send + !Sync
These types **can be moved** to other threads, but they’re **not thread-safe**:
  - `Cell`;
  - `RefCell`;
  - `UnsafeCell`;
  - `OnceCell`;
  - `mpsc::Sender<T>`;
  - `mpsc::Receiver<T>`;

<br>

# !Send + Sync
These types are **thread-safe**, but they **cannot be moved** to another thread:
- `MutexGuard<T: Sync>`;

<br>

`MutexGuard<T: Sync>` uses **OS kernel API** (`POSIX Threads`, aka `pthread`) in particular **syscalls**: `pthread_mutex_lock()` and `pthread_mutex_unlock()`.<br>
The `pthread_mutex_unlock()` **must** be called in **the same thread** where `pthread_mutex_lock()` was called.<br>
More details [here](https://whenderson.dev/blog/rust-mutexes/).

<br>

# !Send + !Sync
These types are **not thread-safe** and **cannot be moved** to other threads:
  - `Rc<T>`;
  - **raw pointers** (`*const T`, `*mut T`) are neither `Send` nor `Sync`, since the compiler doesn't know much about what they represent;
  - types from **external libraries** that are **not thread safe**;

<br>

Explicit implementation of `Send` or/and `Sync` trait for `!Sync` or/and `!Send` types **requires** `unsafe` keyword, since the compiler cannot check for you if it's correct.<br>
Example:
```rust
struct Y {
    a: *mut i32
}

unsafe impl Send for Y {}
unsafe impl Sync for Y {}

fn main() {

}
```

<br>

## `Rc<T>`
If 2 threads attempt to clone `Rc` that points to the same value, they might try to increment the reference counter at the same time, which is **UB**, because `Rc` doesn't use **atomic operations**.

So, `Rc<T>` **implements** `!Send` and `!Sync`:
```rust
impl<T: ?Sized, A: Allocator> !Send for Rc<T, A> {}
impl<T: ?Sized, A: Allocator> !Sync for Rc<T, A> {}
```

<br>

## `*mut T`
The `*mut T` **implements** `!Send` and `!Sync`:
```rust
impl<T> !Send for *mut T
where
    T: ?Sized,
```
```rust
impl<T> !Sync for *mut T
where
    T: ?Sized,
```

<br>

## `*const T`
The `*const T` **implements** `!Send` and `!Sync`:
```rust
impl<T> !Send for *const T
where
    T: ?Sized,
```
```rust
impl<T> !Sync for *const T
where
    T: ?Sized,
```

<br>

# Examples
## Example 1
Here:
- `Foo` is `Send`
- `Foo` is **not** `Sync` (because of `_not_sync: PhantomData<Cell<()>>`)
- `Foo` is `Copy`
- `foo` is **copied** to thread

<br>

**Code**:
```rust
use std::time::Instant;
use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
struct Foo {
    a: i64,
    _not_sync: PhantomData<Cell<()>>
}

fn main() {
    let now = Instant::now();

    let mut num = 1000;
    let mut foo = Foo{a: 10, _not_sync: PhantomData};

    std::thread::scope(|s| {
        let r = s.spawn(move || {
            println!("num = {:p}", {&foo});
        });

        s.spawn(move || {
            println!("num = {:p}", {&foo});
        });
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

This code is **compiled** and its output:
```shell
num = 0x16db16b08
num = 0x16d90ab08
Elapsed: 350.92µs
```

Note, that **addresses are different**.<br>

<br>

## Example 2
Here:
- `Foo` is `Send`
- `Foo` is **not** `Sync`
- `Foo` is **not** `Copy`
- `foo` is **moved** to thread

<br>

**Code**:
```rust
use std::time::Instant;
use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Debug)]
struct Foo {
    a: i64,
    _not_sync: PhantomData<Cell<()>>
}

fn main() {
    let now = Instant::now();

    let mut num = 1000;
    // let mut foo = Foo{a: 10};
    let mut foo = Foo{a: 10, _not_sync: PhantomData};

    std::thread::scope(|s| {
        let r = s.spawn(move || {
            println!("num = {:p}", {&foo});
        });

        s.spawn(move || {
            println!("num = {:p}", {&foo});
        });
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

This code is **not** compiled and its output:
```shell
error[E0382]: use of moved value: `foo`
```

<br>

## Example 3
Here:
- `Foo` is `Send`
- `Foo` is **not** `Sync`
- `Foo` is `Copy`
- `foo` is passed to thread by **shared reference**

<br>

**Code**:
```rust
use std::time::Instant;
use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
struct Foo {
    a: i64,
    _not_sync: PhantomData<Cell<()>>
}

fn main() {
    let now = Instant::now();

    let mut num = 1000;
    // let mut foo = Foo{a: 10};
    let mut foo = Foo{a: 10, _not_sync: PhantomData};

    std::thread::scope(|s| {
        let r = s.spawn(|| {
            println!("num = {:p}", {&foo});
        });

        s.spawn(|| {
            println!("num = {:p}", {&foo});
        });
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

This code is **not** compiled and its output:
```shell
error[E0277]: `Cell<()>` cannot be shared between threads safely
```

<br>

## Example 4
Here:
- `Foo` is `Send`
- `Foo` is `Sync`
- `Foo` is **not** `Copy`
- `foo` is passed to thread by **shared reference**

<br>

**Code**:
```rust
use std::time::Instant;
use std::cell::Cell;
use std::marker::PhantomData;

#[derive(Debug)]
struct Foo {
    a: i64,
}

fn main() {
    let now = Instant::now();

    let mut num = 1000;
    // let mut foo = Foo{a: 10};
    let mut foo = Foo{a: 10};

    std::thread::scope(|s| {
        let r = s.spawn(|| {
            println!("num = {:p}", {&foo});
        });

        s.spawn(|| {
            println!("num = {:p}", {&foo});
        });
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

This code is **compiled** and its output:
```shell
num = 0x16ee92698
num = 0x16ee92698
Elapsed: 245.42µs
```

Note, that **addresses are the same**.<br>
