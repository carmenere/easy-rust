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
  - [Poisoning](#poisoning)
  - [`Arc<Mutex<T>>`](#arcmutext)
- [Examples](#examples)

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
A `Mutex` is very similar to `RwLock`, but slightly simpler. **Instead** of keeping track of the number of **shared** and **exclusive** borrows like an `RwLock`, it **only allows exclusive borrows**.<br>

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

## Poisoning
If thread that helds the **mutex** is panicked then **mutex** becomes **poisoned** to **signal** other threads that the **wrapped value** might be in an **inconsistent** state.<br>
Once lock is **poisoned** then *all future acquisitions* will return `PoisonError<T>`.<br>
Both `lock()` and `try_lock()` return `PoisonError<T>` if the **mutex** was **poisoned**.<br>

<br>

## `Arc<Mutex<T>>`
Wrapping a `Mutex<T>` in an `Arc` is a common pattern to **share mutable data across threads**.


<br>

# Examples
```Rust
use std::sync::{Arc, Mutex, MutexGuard};

fn main() {
    let m: Mutex<u64> = Mutex::new(10u64);
    let mut l: MutexGuard<'_, u64> = m.lock().unwrap();
    *l += 10;
    println!("new value {}", l);
}
```

<br>

```Rust
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

fn main() {
    let shared_vector = Arc::new(Mutex::new(vec![100, 200, 300]));
    let ids = [1, 2, 3];
    let mut threads = Vec::with_capacity(10);
    for id in ids {
        let a = shared_vector.clone();
        threads.push(
            thread::spawn(move || {
                let a = &a;
                let lock = a.lock();
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
