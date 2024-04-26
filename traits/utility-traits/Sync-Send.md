# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Send and Sync](#send-and-sync)
- [!Send and !Sync](#send-and-sync-1)
- [Send + Sync](#send--sync)
- [Send + !Sync](#send--sync-1)
- [!Send + Sync](#send--sync-2)
- [!Send + !Sync](#send--sync-3)
  - [`Rc<T>`](#rct)
  - [`*mut T`](#mut-t)
  - [`*const T`](#const-t)
  
<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Send`|[std::marker::Send](https://doc.rust-lang.org/std/marker/trait.Send.html)|
|`Sync`|[std::marker::Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html)|

<br>

# Send and Sync
From compiler point of view **thread** is a **scope** `{}`.<br>
`rustc` uses `Sync` and `Send` **traits** to determine is it **safe** or **not** to **move** or **share** by **immutable reference** some **value** to **another thread** (**scope**):
- `Sync` means that **sharing** (by **immutable reference**) *between threads* is **safe**. `Sync` allows an object to to be used by two threads `A` and `B` at the **same** time;
- `Send` means that **passing** (by **value**) to *another thread* is **safe**, in other words type `T` it can be **created** in **one** thread (**scope**) and **dropped** in **another** thread (**scope**). `Send` allows an object to be used by two threads `A` and `B` at **different** times:
  - thread `A` can **create** and use an object;
  - then object is sent to thread `B` and thread `B` **can** use the object while thread `A` **cannot**;

<br>

> **Note**:<br>
> `T` is `Sync` if and only if `&T` is `Send`.

<br>

Both `Sync` and `Send` are **marker** traits and they both are **unsafe**:
```rust
pub unsafe auto trait Send { }
pub unsafe auto trait Sync { }
```

<br>

# !Send and !Sync
- `!Send` types **cannot** be moved or copied to other threads, i.e., **type is bound to the current thread**;
- `!Sync` types can only be **used** by a **single thread** at any **different** time, since their **references cannot** be moved or copied to other threads. But **instances** of `!Sync` types **can** still be **moved** between threads if they implement `Send`;

<br>

# Send + Sync
Most types are `Send` + `Sync`:
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
  - **raw pointers** (`*const T`, `*mut T`);
  - types from **external libraries** that are **not thread safe**;

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
