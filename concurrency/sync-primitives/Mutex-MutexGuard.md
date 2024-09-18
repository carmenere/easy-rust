# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declarations](#declarations)
  - [`Mutex<T>`](#mutext)
  - [`MutexGuard<T>`](#mutexguardt)
  - [Errors](#errors)
    - [`PoisonError`](#poisonerror)
    - [`TryLockError`](#trylockerror)
  - [Results](#results)
    - [`TryLockResult`](#trylockresult)
    - [`LockResult`](#lockresult)
- [In a nutshell](#in-a-nutshell)
  - [Example](#example)
- [Lock poisoning](#lock-poisoning)
- [`Arc<Mutex<T>>`](#arcmutext)
  - [Example](#example-1)
- [Lifetime of the `MutexGuard`](#lifetime-of-the-mutexguard)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Mutex`|[**std::sync::Mutex**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html)|
|`MutexGuard`|[**std::sync::MutexGuard**](https://doc.rust-lang.org/stable/std/sync/struct.MutexGuard.html)|
|`PoisonError`|[**std::sync::PoisonError**](https://doc.rust-lang.org/stable/std/sync/struct.PoisonError.html)|
|`TryLockError`|[**std::sync::TryLockError**](https://doc.rust-lang.org/stable/std/sync/enum.TryLockError.html)|
|`TryLockResult`|[**std::sync::TryLockResult**](https://doc.rust-lang.org/stable/std/sync/type.TryLockResult.html)|
|`LockResult`|[**std::sync::LockResult**](https://doc.rust-lang.org/stable/std/sync/type.LockResult.html)|

<br>

# Declarations
## `Mutex<T>`
```rust
pub struct Mutex<T: ?Sized> {
    inner: sys::Mutex,
    poison: poison::Flag,
    data: UnsafeCell<T>,
}
```

<br>

## `MutexGuard<T>`
```rust
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a Mutex<T>,
    poison: poison::Guard,
}
```

<br>

## Errors
### `PoisonError`
```rust
pub struct PoisonError<T> {
    guard: T,
}
```

Some usefull methods of `PoisonError<T>`:
- [**into_inner()**](https://doc.rust-lang.org/stable/std/sync/struct.PoisonError.html#method.into_inner)
  - **consumes** this error, returning the underlying data of type `T`;

<br>

### `TryLockError`
```rust
pub enum TryLockError<T> {
    Poisoned(PoisonError<T>),
    WouldBlock,
}
```

<br>

## Results
Below:
- The `Guard` parameter means a type that carries the value, this is `MutexGuard` for `Mutex`;
- The `Ok` variant means that lock is **acquired** and **not** poisoned;
- The Err variant means that lock was **not** *acquired* because it's already **locked** (`WouldBlock`) or because mutex is **poisoned** (`Poisoned`);

<br>

### `TryLockResult`
```rust
pub type TryLockResult<Guard> = Result<Guard, TryLockError<Guard>>;
```

<br>

### `LockResult`
```rust
pub type LockResult<Guard> = Result<Guard, PoisonError<Guard>>;
```

<br>

# In a nutshell
A `Mutex` is very similar to `RwLock`, but slightly simpler. **Instead** of keeping track of the number of **shared** and **exclusive** borrows like an `RwLock`, it **only allows exclusive borrows**. The job of **mutex** is to ensure threads have exclusive access to some data by **temporarily blocking** other threads that try to access it **at the same time**.<br>
Unlike other languages, in Rust **mutex** `Mutex<T>` is a generic over type `T`, which is the type of the data the **mutex** is protecting. The data can only be accesse through the **mutex**.<br>
Conceptually a **mutex** has only two states: **locked** and **unlocked**. When a thread attempts to **lock** an already locked mutex it will be blocked: the thread is put to sleep until mutex become **unlocked**.<br>
**Unlocking** is only possible on locked mutex and must be done by the **same thread** that locked it. To ensure a locked mutex can only be unlocked by the thread that locked it, it **doesn't have** an `unlock()` method. **Instead**, its `lock()` method returns a special type called a `MutexGuard`. This type represents the **guarantee** that we have locked the mutex. It behaves like an exclusive reference through the `DerefMut` trait. Unlocking the mutex is done by **dropping the guard**: the `Drop` implementation of the **guard** will **unlock** the mutex.<br>

> Note:<br>
> Keep amount of time a mutex is locked **as short as possible**.<br>

<br>

- The `Mutex<T>` type **implements** `Send` and `Sync` if type `T` does:
```rust
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
```
- The `MutexGuard<T>` type **implements** `!Send`:
```rust
impl<T: ?Sized> !Send for MutexGuard<'_, T> {}
```
- The `MutexGuard<T>` type **implements** `Sync` if type `T` does:
```rust
unsafe impl<T: ?Sized + Sync> Sync for MutexGuard<'_, T> {}
```

<br>

Some usefull methods of `Mutex<T>`:
- [**lock()**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.lock):
  - **blocks** the current thread until it is avaliable to acquire the mutex;
  - it returns `Result<MutexGuard<T>>` and then `.unwrap()` returns `MutexGuard<T>`;
- [**try_lock()**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.try_lock):
  - if the lock **could not** be acquired at this time, then `WouldBlock` error is returned;
- [**into_inner()**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.into_inner)
  - **consumes** this mutex, returning the underlying data;
  - returns `LockResult<T>`;
- [**is_poisoned()**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.is_poisoned)
  - **determines** whether the mutex is poisoned;
- [**clear_poison()**](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html#method.clear_poison)
  - **recovers** from a poisoned state and marking that it has recovered;

<br>

Both `lock()` and `try_lock()` return **RAII guard**. **RAII guard** means that the **lock** will be **released** when the **guard** is **dropped**.<br>

<br>

So `MutexGuard<T>` is a **smart pointer** that is a **RAII guard**. The `MutexGuard<T>` holds the data.<br>
When `MutexGuard<T>` **goes out of scope** it **releases lock** to the data. But it can be released as soon as possible by `drop()` function.<br>

Due to **deref coercions** we can call `T`’s methods on the `MutexGuard<T>` instance. Its `deref()` method returns pointer to internal value.<br>

<br>

## Example
```Rust
use std::{sync::Mutex, thread};

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for i in 1..=10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for i in 1..=10 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 100);
}
```

In the example above, the var `guard` is of type `MutexGuard<i32>`.<br>

<br>

# Lock poisoning
If thread **panics** while holding the lock then mutex becomes **poisoned**. **Lock poisoning** indicates other threads that the data that is protected by a mutex **potentially** is in an **inconsistent state**.<br>
Any further attemt to lock poisoned mutex returns an `Err` to indicate it has been poisoned, in other words any calling `lock()` or `try_lock()` methods on **poisoned mutex** returns `PoisonError<T>`.<br>

Calling `lock()` on a *poisoned mutex* **still locks** the *mutex*. The `Err` returned by `lock()` contains the `MutexGuard`, allowing us:
- **correct** or **somehow process** an inconsistet state **if possible**;
- **propagate panic**;


<br>

# `Arc<Mutex<T>>`
Wrapping a `Mutex<T>` in an `Arc<T>` is a common pattern to **share mutable data across threads**.

## Example
```Rust
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![100, 200, 300]));
    let ids = [1, 2, 3];
    let mut threads = Vec::with_capacity(10);
    for id in ids {
        let v = v.clone();
        threads.push(
            thread::spawn(move || {
                let v = &v;
                let lock = v.lock();
                if lock.is_ok() {
                    let mut vsafe = lock.unwrap();
                    vsafe.push(id.clone());
                    println!("My thread id is {}. v.len() = {}", id.clone(), vsafe.len())
                }
                else {
                    println!("My thread id is {}. Can not take lock!", id.clone())
                }
            })
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```

<br>

# Lifetime of the `MutexGuard`
When we assign the name to the **guard** wtih a `let` then it will be dropped at the end of scope or explicitly by `drop(guard)` function.<br>
But we can use **guard** without assigning it a name. For example, if you have a `Mutex<Vec<i32>>`, you can **lock** the mutex, **push** an item into the `Vec`, and **unlock** the mutex again, **in a single statement**:
```rust
list.lock().unwrap().push(1);
```
