# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Condvars](#condvars)
  - [Example](#example)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Condition variables`|[**std::sync::Condvar**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html)|

<br>

# Condvars
While a **mutex** does allow threads to wait until it becomes unlocked, it **doesn't** provide functionality for **waiting for** any other **conditions**.<br>
There are 2 ways to wait for a notification from another thread:
- **thread parking**;
- **condition variables**;

<br>

**Usecase**: we can create a **condition variable** for specific events or conditions we're interested in, such as the queue is empty and wait on that condition.<br>
Any thread that causes that event or **condition** then notifies the **condition variable**.<br>

**Condition variable** has two basic operations: **wait** and **notify**.<br>
**Condition variables** are typically associated with a **boolean predicate** (a **condition**) and a **mutex**. The predicate is always verified inside of the mutex before determining that a thread must block.<br>

Rust provides `std::sync::Condvar` type. Its `wait()` method takes `MutexGuard` that proves we have locked the mutex. The `wait()` method must **unlock** mutex and **put** thread to sleep in one **atomic** operation.<br>

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

## Example
```rust
use std::{sync::{Mutex, Condvar}, thread};
use std::collections::VecDeque;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            loop {
                if let Some(item) = q.pop_front() {
                    dbg!(item);
                } else {
                    q = not_empty.wait(q).unwrap();
                }
            };
        });

        for i in 1..=10 {
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
```

In the above example **producer** thread **needn't** store the return value from `spawn()` because the **producer** thread **needn't** to know which thread to wake up.<br>