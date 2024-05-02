# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Atomics](#atomics)
- [Condvars](#condvars)
- [Examples](#examples)
  - [Atomics](#atomics-1)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Atomic types`|[**std::sync::atomic**](https://doc.rust-lang.org/stable/std/sync/atomic/index.html)|
|`Condition variables`|[**std::sync::Condvar**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html)|

<br>

# Atomics
[**About portability of atomic types here**](https://doc.rust-lang.org/stable/std/sync/atomic/index.html#portability).<br>
Constructors for all **atomic types** are all `const` functions: `const fn abc() { … }`.<br>
It is possible to use **global atomic variable**. But **atomic globals** are limited to `integers` and `bool`.<br>

<br>

# Condvars
**Condition variables** represent the ability to **block a thread** such that it consumes no CPU time while **waiting** for an **event** to occur.<br>
**Condition variables** are typically associated with a **boolean predicate** (a **condition**) and a **mutex**. The predicate is always verified inside of the mutex before determining that a thread must block.<br>

Some usefull methods of `Condvar`:
- [**wait(guard: MutexGuard<'a, T>)**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html#method.wait):
  - **blocks calling thread** until some **other thread** calls `notify_one()` or `notify_all()`;
- [**wait_timeout(dur: Duration)**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html#method.wait_timeout):
  - **blocks calling thread** until some **other thread** calls `notify_one()`/`notify_all()` or `dur` is **expired**;
- [**notify_all()**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html#method.notify_all)
  - **wakes up all** blocked threads on this condvar;
- [**notify_one()**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html#method.notify_one)
  - **wakes up one** blocked thread on this condvar;

<br>

# Examples
## Atomics
```Rust
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

fn main() {
    let counter = Arc::new(AtomicI64::new(10));
    let ids = [1, 2, 3, 4, 5];
    let mut threads = Vec::with_capacity(10);

    for id in ids {
        let c = counter.clone();
        threads.push(
            thread::spawn(move || {
                let c = c;
                c.fetch_add(id, Ordering::SeqCst);
                println!("My thread id is {}. Counter = {}", id, c.load(Ordering::SeqCst))}
            )
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```