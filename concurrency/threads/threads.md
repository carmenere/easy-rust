# Table of contents
- [Table of contents](#table-of-contents)
- [Threads](#threads)
  - [`Thread` type](#thread-type)
  - [`JoinHandle` type](#joinhandle-type)
  - [thread::spawn](#threadspawn)
  - [thread::Builder](#threadbuilder)
  - [Examples](#examples)
  - [Constraints](#constraints)
      - [`'static` constraint](#static-constraint)
      - [`Send` constraint](#send-constraint)
- [`park` and `unpark`](#park-and-unpark)
- [Examples](#examples-1)
  - [Just one thread](#just-one-thread)
  - [Multiple threads](#multiple-threads)
- [Sharing immutable data across threads](#sharing-immutable-data-across-threads)

<br>

# Threads
An executing Rust program consists of a collection of **native OS threads**, each with their **own stack** and **local state**.<br>
When the **main thread** *terminates*, the **whole process** is *terminated*, even if other threads are *still running*.<br>
So, the spawned thread **may outlive** the caller *unless the caller thread is the main thread*.

<br>

## `Thread` type
The `Thread` type is a **handle** to a **thread**, i.e., it represents **thread**.<br> 
There are 2 ways to obtain instance of `Thread` type:
- by **spawning a new thread**, e.g., using the `thread::spawn` function or `thread::Builder::spawn` method, and calling `thread` method on the `JoinHandle`.
  - the `thread::spawn` returns `JoinHandle<T>`;
  - the `thread::Builder::spawn` method returns `Result<JoinHandle<T>>`.
- by **requesting the current thread**, using the `thread::current` function.

<br>

All threads have `id`:
```Rust
use std::thread::{self, JoinHandle};

fn main() {
    let other_thread: JoinHandle<u64> = thread::spawn(|| {
        dbg!(thread::current().id());
        1
    });
    
    let result: u64 = other_thread.join().unwrap();
    dbg!(result);
}
```

<br>

By default, *spawned threads* are **unnamed**.<br>
To **specify** a name for a thread, build the thread with `Builder` and pass the desired thread name to `Builder::name`.<br>
To **retrieve** the thread name from within the thread, use `Thread::name`.<br>

<br>

## `JoinHandle` type
Instance of `JoinHandle` is created by the `thread::spawn` function and the `thread::Builder::spawn` method.<br>

The `JoinHandle` provides a `join` method that can be used to **join the spawned thread**, i.e., to **wait for thread to finish** and obtain its **result**.<br>
If the *spawned thread* panics, `join` will return an `Err` containing the argument given to `panic!`.<br>
Due to platform restrictions, it is not possible to `Clone` this `JoinHandle` and the ability to **join a thread** is a uniquely-owned permission.<br>

<br>

## thread::spawn
```Rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

This call will create a thread using default parameters of `Builder`, if you want to specify the **stack size** or the **name** of the thread, use `thread::Builder`.<br>
If the **join handle** is not used, the *spawned thread* will implicitly be **detached**. In this case, the *spawned thread* may no longer be joined.<br>


<br>

## thread::Builder
```Rust
pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

`Builder` is a **thread factory**, which can be used in order to **configure** the *properties of a new thread*.

<br>

## Examples
```Rust
use std::thread;

fn main() {
    let computation: thread::JoinHandle<i32> = thread::spawn(|| {
        // Some expensive computation.
        42
    });

    let result: i32 = computation.join().unwrap();
    println!("{result}");
}
```

<br>

```Rust
use std::thread;

fn main() {
    let builder: thread::Builder = thread::Builder::new();

    let handler: thread::JoinHandle<()> = builder.spawn(|| {
        // thread code
    }).unwrap();

    handler.join().unwrap();
}
```

<br>

## Constraints
#### `'static` constraint
The `'static` constraint means that the **closure** and its **return value** must have a **lifetime** of the **whole program execution**.<br>
The reason for this is that threads **can outlive** the lifetime they have been created in.<br>
Indeed if the **thread** and its **return value** can **outlive** their **caller**, we need to make sure that they will be **valid** afterwards, and since we **can’t know** when it will return we need to have them valid **as long as possible**, that is **until the end of the program**, hence the `'static` **lifetime**.<br>

Rust has no way of knowing how long the child thread will run, so it **assumes** the **worst**: it assumes the **child thread** can **outlive** **parent thread**.<br>
So when you pass reference to `thread::spawn` closure, it assumes this **reference can outlive original value it points to**.<br>

<br>

#### `Send` constraint
The `Send` constraint is because the closure is passed **by value** from the thread where it is spawned to the new thread.

<br>

# `park` and `unpark`
`std::thread::park` blocks the **current thread**, which can then be *resumed* **from another thread** by calling the `unpark` method on the blocked **thread’s handle**.

<br>

```Rust
use std::thread;
use std::time::Duration;

fn main () {
    let parked_thread: thread::JoinHandle<()> = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    // Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    println!("Unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}
```

<br>

# Examples
## Just one thread
```Rust
use std::{thread, thread::JoinHandle, time::Duration};
use rand::{Rng};

fn main() {
    let mut v = (1..=100).collect::<Vec<u32>>();

    let handle: JoinHandle<u64> = thread::spawn(|| {
        let mut r = rand::thread_rng();
        let id = thread::current().id();
        let delay = r.gen_range(1..=5);
        println!("Thread id: {:?}", id);
        thread::sleep(Duration::from_secs(delay));
        println!("Thread id: {:?}", id);
        delay
    });
    
    let r = handle.join();
    if let Ok(r) = r {
        println!("Result: {}", r);
    }
}
```

<br>

## Multiple threads
```Rust
use std::{thread, thread::JoinHandle, time::Duration};
use rand::{Rng};

fn main() {
    let mut v = (1..=10).collect::<Vec<u32>>();

    let handles: Vec<JoinHandle<u64>> = v.iter().map(|i| {
        thread::spawn(|| {
            let mut r = rand::thread_rng();
            let id = thread::current().id();
            let delay = r.gen_range(1..=5);
            println!("Thread id: {:?}, will sleep {} sec. ", id, delay);
            thread::sleep(Duration::from_secs(delay));
            println!("Thread id: {:?}, waked up, continue execution.", id);
            delay * delay
        })
    }).collect();

    for h in handles {
        let id = h.thread().id();
        if let Ok(r) = h.join() {
            println!("Thread id: {:?}, result: {}", id, r);
        }
    }
}
```

<br>

# Sharing immutable data across threads
To share data between threads there is `Arc` type.<br>

```Rust
use std::thread;

use std::sync::Arc;

fn main() {
    let shared_vector = Arc::new(vec![100, 200, 300]);
    let ids = [1, 2, 3];
    let mut threads = Vec::with_capacity(10);
    for id in ids {
        let shared_vector_per_thread = shared_vector.clone();
        threads.push(
            thread::spawn(move || {
                let v = &shared_vector_per_thread;
                println!("My thread id is {}. v.len() = {}", id, v.len())}
            )
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```
