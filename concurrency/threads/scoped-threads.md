# Table of contents
- [Table of contents](#table-of-contents)
- [Non-scoped threads](#non-scoped-threads)
    - [Example: borrow variables with the `'static` lifetime](#example-borrow-variables-with-the-static-lifetime)
- [Scoped threads](#scoped-threads)
- [Constraints](#constraints)
  - [std::thread::scope](#stdthreadscope)
  - [`Scope` type](#scope-type)

<br>

# Non-scoped threads
The code below **doesn’t work** because the borrow checker complains about **spawned thread may outlive borrowed value** `s`:
```Rust
use std::thread;

fn main() {
    let s = String::from("Hello");
    thread::spawn(|| {
        println!("Length: {}", s.len());
    });
}
```

<br>

The problem here is that spawned threads are not allowed to borrow variables on **stack** because the compiler **cannot prove** they will be joined before `s` is destroyed.<br>
If a variable is borrowed by a thread, the **thread must complete before the variable is destroyed**.<br>
Threads spawned using `std::thread::spawn` can only borrow variables with the `'static` lifetime because the borrow checker cannot be sure when the thread will complete.

<br>

### Example: borrow variables with the `'static` lifetime
```rust
#![allow(non_upper_case_globals)]

use std::thread;

static mut people: Vec<String> = vec![];

fn main() {
    unsafe {
        people = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Carol".to_string(),
    ];
    }
    
    let mut threads = Vec::new();

    let p: &'static Vec<String> = unsafe { &people };
    
    for person in p {
        threads.push(thread::spawn(move || {
            println!("Hello, {}!", person);
        }));
    }
    
    for thread in threads {
        thread.join().unwrap();
    }
}
```

<br>

# Scoped threads
Consider example:
```Rust
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::scope(|s| {
        s.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
```

The Rust stndard library provides the `std::thread::scope` function. We call `std::thread::scope` function with a closure. This closure gets an instance of `Scope`: `s`, representing the **scope**. This argument `s` of closure is used further to spawn threads.<br>
The `scope` guarantees that **none** of the threads spawned inside the `std::thread::scope` **can outlive** the `scope`, in other words, **all** threads will be joined at the end of the scope. Scoped `.spawn()` method doesn't have a `'static` bound, allowing to reference anything that outlives the scope.<br>
Unlike *non-scoped threads*, scoped threads **can borrow non**-`'static` data.<br>
When `scope` ends, all threads that haven't been joined yet are automatically joined.<br>

So, *scoped threads* allow us **borrow** data from **longer living** *parent threads*.<br>

<br>

# Constraints
## std::thread::scope
Signature of `std::thread::scope` function:
```rust
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T
{ 
    ...
}
```

<br>

## `Scope` type
```rust
pub struct Scope<'scope, 'env: 'scope> {
    data: Arc<ScopeData>,
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}
```
<br>

Scoped threads involve two lifetimes: `'scope` and `'env`:
- the `'scope` lifetime represents the **lifetime of the scoped threads**, once this **lifetime ends**, all **scoped threads are joined**;
- the `'env` lifetime represents the **lifetime of whatever is borrowed** by the scoped threads;

<br>

The `'env: 'scope` boundary means means `'env` outlives `'scope`.<br>
