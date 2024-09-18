# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Condvars](#condvars)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Condition variables`|[**std::sync::Condvar**](https://doc.rust-lang.org/stable/std/sync/struct.Condvar.html)|

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