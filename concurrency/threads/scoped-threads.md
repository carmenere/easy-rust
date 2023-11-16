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
**Scoped threads** are a mechanism to guarantee to the compiler that **spawned threads will be joined before the scope ends**.<br>
Whenever a scope spawns a thread, it promises to join the thread **before the scope ends**.<br>
scope guarantees all threads will be joined at the end of the scope, i.e., **scoped threads only live within the scope** and **can safely access variables outside it**.<br>
Unlike *non-scoped threads*, scoped threads **can borrow non**-`'static` data.
All threads spawned within the scope that haven’t been manually joined will be **automatically joined** before this function returns.

<br>

### Scoped threads example
```Rust
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
```

<br>


## `scope` function
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

Type `Scope`:
```rust
pub struct Scope<'scope, 'env: 'scope> {
    data: Arc<ScopeData>,
    scope: PhantomData<&'scope mut &'scope ()>,
    env: PhantomData<&'env mut &'env ()>,
}
```

<br>

### Scoped threads lifetimes
Scoped threads involve two lifetimes: `'scope` and `'env`:
- the `'scope` lifetime represents the **lifetime of the scoped threads**, once this **lifetime ends**, all **scoped threads are joined**;
- the `'env` lifetime represents the **lifetime of whatever is borrowed** by the scoped threads;

<br>

Also there is boundary: `'env: 'scope`, it means means `'env` outlives `'scope`.<br>