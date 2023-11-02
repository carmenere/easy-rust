# Send and Sync
From compiler point of view **thread** is a **scope** `{}`.<br>
`rustc` determines is it **safe** or **not** to **move** or **share** by immutable reference some **value** to **another thread** (**scope**).<br>
`Sync` and `Send` are **marker traits**:
- `Sync` means that **sharing** (by immutable reference) *between threads* is **safe**.
- `Send` means that **passing** (by value) to *another thread* is **safe** and it can be **created** in **one** thread (scope) and **dropped** in **another** thread (scope).

<br>

> **Note**:<br>
> `T` is `Sync` if and only if `&T` is `Send`.

<br>

Types that are **neither** `Send` **nor** `Sync` **can't** be **passed** or **shared** between threads.<br>

<br>

In other words:
- `Send` allows an object to be used by two threads `A` and `B` at **different** times. Thread `A` can create and use an object, then send it to thread `B`, so thread `B` can use the object while thread `A` cannot.
- `Sync` allows an object to to be used by two threads `A` and `B` at the **same** time.

<br>

# !Send and !Sync
- `!Send` types can only ever be **owned** by a **single thread**, since they **cannot** be moved or copied to other threads, i.e., **type is bound to the current thread**:
  - for example: `MutexGuard`, because of the `unlock` syscall which must occur in the same thread where `lock` was.
- `!Sync` types can only be **used** by a **single thread** at any **different** time, since their **references cannot** be moved or copied to other threads. But **instances** of `!Sync` types **can** still be **moved** between threads if they implement `Send`.

<br>

# Send + Sync
Most types are `Send` + `Sync`:
- `i8`, `f32`, `bool`, `char`, `&str`, ...
- `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...
- `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...
- `AtomicBool`, `AtomicU8`, ...
- `Arc<T>`
- `Mutex<T>`

<br>

# Send + !Sync
These types **can be moved** to other threads, but they’re **not thread-safe**:
  - `Cell`
  - `RefCell`
  - `mpsc::Sender<T>`
  - `mpsc::Receiver<T>`

Because `Cell` and `RefCell` implement **interior mutability**.

<br>

# !Send + Sync
These types are **thread-safe**, but they **cannot be moved** to another thread:
- `MutexGuard<T: Sync>`

<br>

`MutexGuard<T: Sync>` uses **OS kernel API** (`POSIX Threads`, aka `pthread`) in particular **syscalls**: `pthread_mutex_lock()` and `pthread_mutex_unlock()`.<br>
The `pthread_mutex_unlock()` **must** be called in **the same thread** where `pthread_mutex_lock()` was called.

<br>

# !Send + !Sync
These types are **not thread-safe** and **cannot be moved** to other threads:
  - `Rc<T>`
  - **raw pointers** (`*const T`, `*mut T`)
  - types from **external libraries** that are **not thread safe**
