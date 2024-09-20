# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Atomics](#atomics)
- [Memory ordering](#memory-ordering)
- [Opeartions](#opeartions)
  - [Load and store](#load-and-store)
  - [Fetch-and-Modify](#fetch-and-modify)
  - [Compare-and-Exchange](#compare-and-exchange)
    - [Overflow check](#overflow-check)
    - [Compare-and-Exchange based increment](#compare-and-exchange-based-increment)
- [Examples](#examples)
  - [Progress bar](#progress-bar)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Atomic types`|[**std::sync::atomic**](https://doc.rust-lang.org/stable/std/sync/atomic/index.html)|

<br>

# Atomics
The word **atomic** means **indivisible**, something that **cannot** be cut into smaller pieces. **Atomic operations** are the main building blocks for anything involving multiple threads. All the other sync primitives, such as **mutexes** and **condition variables**, are implemented using atomic operations.<br>

In Rust, **atomic operations** are avaliable as **methods** on the **atomic types** in in `std::sync::atomic`.<br>

The **atomic types** represent the concurrent version of a `Cell`. They allow modification through a shared reference.<br>
Like `Cell`, they avoid UB by making us copy values in and out as whole, without letting us borrow the content directly.<br>
Unlike a `Cell` they **cannot** be of **arbitrary size**. Because of this, there is **no** generic `Atomic<T>` type for any type `T`, but there are **only specific atomic types** such as `AtomicU32` and `AtomicPtr<T>`.<br>

Because of limited size **atomics** don't directly contain the data that needs to be shared between threads. Instead, **atomics** are often used as a tool **to build synchronization primitives**.<br>

Constructors for all **atomic types** are all `const` functions: `const fn abc() { … }`.<br>
It is possible to use **global atomic variable**. But **atomic globals** are limited to `integers` and `bool`.<br>

Which **atomic** types are avaliable **depends on the platform**, since they **require support from the processor** to avoid data races. [**About portability of atomic types here**](https://doc.rust-lang.org/stable/std/sync/atomic/index.html#portability).<br>

<br>

# Memory ordering
Every atomic operation takes an argument of type `std::sync::atomic::Ordering`, which determines memory ordering guarantees.<br>
```rust
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

The simplest variant is `Relaxed`, it **guarantees consistency** on a **single atomic variable**, but **doesn't** promise anything about the relative order of operations **between different variables**. It means that if **first thread** writes to variable `foo` and then writes to variable `bar`, **second thread** might see that happen in the opposite order, because these atomic variables are updated **separetly**. So, **second thread** can read **old** value of variable `foo` and **new** value of variable `bar`.<br>

<br>

# Opeartions
## Load and store
|Operation|Description|
|`load()`|**loads** the value stored in the atomic variable|
|`store()`|**stores** the value in the atomic varibale|

<br>

## Fetch-and-Modify
All **fetch-and-modify** operations **modify** the **atomic variable**, but also **return** the **original value**, all **atomically** as a single operation.<br>

In the following example `v.fetch_add(55, Relaxed)` increments `v` from `100` to `155`, but returns the **old** value of `100`:
```rust
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn main() {
    let v = AtomicU64::new(100);
    let a = v.fetch_add(55, Relaxed);
    let b = v.load(Relaxed);

    assert_eq!(a, 100);
    assert_eq!(b, 155);
}
```
<br>

The **fetch-and-modify** operations are **not** suitable for reliable overflow checking in concurrent environment, because they don't prevent from overflow.<br>

<br>

## Compare-and-Exchange
The **compare-and-exchange** operation checks if the atomic value is **equal** to a **expected** value and **only** if that is the case **replaces** it with the **new** value, but also **returns** the **original value**, all **atomically** as a single operation.<br>

For `std::sync::atomic::AtomicI8` it is looks like this:
```rust
pub fn compare_exchange(
    &self,
    current: i8, // expected
    new: i8,
    success: Ordering,
    failure: Ordering,
) -> Result<i8, i8> {
  let v = self.load();
  if v == current {
    self.store(new);
    Ok(v)
  } else {
    Err(v)
  }
}
```

<br>

### Overflow check
The **compare-and-exchange** operation allow implement reliable overflow checking in concurrent environment, example:
```rust
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn main() {
    println!("id = {}", allocate_id());
    println!("id = {}", allocate_id());
}

fn allocate_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < u64::MAX, "overflow of u64!");
        match NEXT_ID.compare_exchange(id, id+1, Relaxed, Relaxed) {
            Ok(v) => return id,
            Err(v) => id = v,
        }
    }
}
```

Consider following execution order:
1. The actual value of `NEXT_ID` is `u64::MAX - 1`;
2. **First thread** calls `assert!(id < u64::MAX, "overflow of u64!")`;
3. **Second thread** calls `assert!(id < u64::MAX, "overflow of u64!")`;
4. **First thread** calls `NEXT_ID.compare_exchange(id, id+1, Relaxed, Relaxed)`;
5. **Second thread** calls `NEXT_ID.compare_exchange(id, id+1, Relaxed, Relaxed)` -> failed, because the **first thread** has already changed the value;
6. **Second thread** goes to secound round and calls `assert!(id < u64::MAX, "overflow of u64!")` again and **paincs**, because **condition is no longer met**;

<br>

If we used `fetch_add()` instead `compare_exchange()` we would get **overflow** on the **step 5**. Because both threads have the same loaded value and both pass the `assert!` checks.<br>

<br>

### Compare-and-Exchange based increment
```rust
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

fn main() {
    println!("id = {}", {increment(&NEXT_ID); NEXT_ID.load(Relaxed)});
    println!("id = {}", {increment(&NEXT_ID); NEXT_ID.load(Relaxed)});
}

fn increment(a: &AtomicU64) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(v) => return,
            Err(v) => current = v,
        }
    }
}
```
If `a` is still the same as previously loaded it is replaced by `new` value.<br>
If another thread has changed `a` and it is not the same as previously loaded, `compare_exchange` returns **updated** value and we will **try again** using that **updated** value.<br>
The **brief moment** between *loading* and *updating* is **so short** that it's **unlikely** for this to loop *more than a few iterations*.<br>

<br>

# Examples
## Progress bar
```rust
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // std::process::exit(0);
    let iteration = &AtomicU64::new(0);
    let mut times = Vec::with_capacity(128);

    thread::scope(|s| {
        let mut times = &mut times;
        let t = s.spawn(|| {
            loop {
                let n = iteration.load(Relaxed);
                println!("Working... {n}% done");
                if n == 100 { break; }
                thread::park_timeout(Duration::from_secs(1));
            }
        });

        s.spawn(move || {
            let total_time = Instant::now();
            for i in 0..=100 {
                println!("i: {}, iteration: {}", i, iteration.load(Relaxed));
                iteration.store(i, Relaxed);
                let duration = Instant::now();
                process();
                times.push(duration.elapsed().as_micros());
                t.thread().unpark();
            }
            println!("Total real time, microsec: {}", total_time.elapsed().as_micros() as u64);
        });
    });

    println!("Total user time, microsec: {}", times.iter().sum::<u128>());
    println!("Done!");
    dbg!(times);
}

fn process() {
    thread::sleep(Duration::from_millis(1)); // emulate processing
}
```