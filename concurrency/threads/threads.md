# Table of contents
- [Table of contents](#table-of-contents)
- [Intro](#intro)
- [Threads in Rust](#threads-in-rust)
  - [`Thread` type](#thread-type)
  - [`ThreadId` type](#threadid-type)
  - [Thread names](#thread-names)
  - [Example](#example)
- [Joining threads](#joining-threads)
  - [Output locking](#output-locking)
  - [Example](#example-1)
- [Closures](#closures)
  - [Example](#example-2)
- [Constraints](#constraints)
  - [std::thread::spawn](#stdthreadspawn)
  - [std::thread::Builder](#stdthreadbuilder)
  - [`'static` constraint](#static-constraint)
  - [`Send` constraint](#send-constraint)

<br>

# Intro
Operating systems allow to run many programs **concurrently**. This is achieved by rapidly switching between processes, allowing each to repeatedly make a little bit of progress, one by one.<br>
Operating systems **completely isolate** processes from each other. A process **cannot** accidentally access the memory of another process without asking the kernel first.<br>
However, a programm can **spawn** extra **threads of execution** (aka **native OS threads**), **within** the **same process**. **Threads** within the same process are **not** isolated from each other. Threads **share memory** and can interact with each other through that memory. But **each thread** has its **own stack** and some **local state**.<br>

<br>

# Threads in Rust
An executing Rust program consists of a collection of **native OS threads**.<br>
Every program starts with exactly **one** thread: the **main thread**. The **main thread** executes the `main()` function and can be used to **spawn other threads**.<br>
When the **main thread** *terminates*, the **whole process** is *terminated*, even if other threads are *still running*.<br>
**Any** spawned thread can also **spawn** other threads. So, the spawned thread **may outlive** the *caller thread* **unless** the *caller thread* is the **main thread**.<br>

In Rust, there are **2 ways** to **spawn** new thread:
- by using the `std::thread::spawn()` function, which **returns** `JoinHandle<T>`;
```rust
let t = thread::spawn(f);
```
- by using the `std::thread::Builder::new().spawn()` method, which **returns** `Result<JoinHandle<T>>`;
```rust
let t  = builder.spawn(f2); // Here 'builder' is an instance of 'std::thread::Builder'
```

<br>

The `std::thread::Builder` allows you to set some settings for the new thread before spawning it, for example, the **stack size** or/and **thread name**:
```rust
let builder = thread::Builder::new()
    .name("foo".into())
    .stack_size(32 * 1024);
```

<br>

Both `spawn()` functions take the **single argument**: the function the new thread will execute. The thread **stops** once this function **returns**.<br>
But also they have some **differences**:
- the `std::thread::spawn()` function is actually a convenient shorthand for `std::thread::Builder::new().spawn().unwrap()` with default parameters of `Builder`;
- the `std::thread::spawn()` function simply **panics** if it is unable to spawn a new thread, it returns `JoinHandle<T>` on success;
- the `Builder`'s `spawn()` function returns `Result<JoinHandle<T>>`, allowing you to handle situations where spawning a new thread fails, this might happen if the process reaches resource limits;

<br>

## `Thread` type
The `Thread` type represents the **thread**.<br> 
There are 2 ways to obtain instance of `Thread` type:
- by calling `.thread()` method on the `JoinHandle<T>` instance;
- by calling `thread::current()` function inside thread;

<br>

## `ThreadId` type
The Rust standard library assigns every thread a **unique identifier**. This identifier is of type `ThreadId` and accessible throgh `.id()` method on the `Thread` instance.

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

## Thread names
By default, *spawned threads* are **unnamed**.<br>
To **specify** a **thread name**, build the thread with `Builder` and pass the desired name to `Builder::new().name()`.<br>
To **retrieve** a **thread name** use `.name()` method on `Thread` instance.<br>

<br>

## Example
```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let t1  = builder.spawn(f);
    let t2 = thread::spawn(f);
    let t3 = thread::spawn(f);

    println!("Hello from main thread!");
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("My thread id is {:?}", id);
}
```

<br>

# Joining threads
If we want to make sure the threads are finished **before** we return from **main thread**, we must **join** all such threads.
To do so, there is the `JoinHandle<T>` retruned by the `thread::spawn()` function or the `Result<JoinHandle<T>>` retruned by the `thread::Builder::new().spawn()` method.<br>

The `JoinHandle<T>` provides a `join()` method to **join** particular spawned thread. The `join()` method **waits until thread has terminates** and returns `std::thread::Result`. <br>

If the *spawned thread* panics, `join()` will return an `Err` containing the `panic!` message. We can handle that situation or call `.unwrap()` to **propagate panic** when joining panicked thread.
Due to platform restrictions, it is not possible to `Clone` this `JoinHandle` and the ability to **join a thread** is a uniquely-owned permission.<br>

<br>

## Output locking
The `println!` macro uses `std::io::Stdout::lock()` to make sure its output does not get interrupted. A println! expression will wait until any concurrently running one is finished before writing any output. If this was not the case, the output of different threads was interleaved.

<br>

## Example
```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let t1  = builder.spawn(f);
    let t2 = thread::spawn(f);
    let t3 = thread::spawn(f);

    println!("Hello from main thread!");

    let r = match t1 {
        Ok(t1) => t1.join(),
        Err(e) => todo!()
    };

    let r = t2.join();
    let r = t3.join();
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("My thread id is {:?}", id);
}
```

<br>

# Closures
Rather then passing the name of function to `spawn()` it is more common to pass it a **closure**. This allows to **capture** values for the new thread.
Use `move` keyword for closures that are passed to threads. **By default**, closures capture values by **reference**. But spawned thread **can outlive** the caller thread (if its **not** the *main thread*) and this will result to compiler error.

<br>

## Example
```rust
use std::thread;

fn main() {
    let numbers = vec![1,2,3,4,5];

    let t = thread::spawn(move || {
        let len = numbers.len() as u64;
        let sum = numbers.into_iter().sum::<u64>();
        sum/len
    });

    let avg = t.join().unwrap();

    println!("average: {avg}");
}
```

<br>

# Constraints
## std::thread::spawn
```Rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

<br>

## std::thread::Builder
```Rust
pub fn spawn<F, T>(self, f: F) -> Result<JoinHandle<T>>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

## `'static` constraint
The `'static` constraint means that the **closure** and its **return value** must have a **lifetime** of the **whole program execution**, in other words, a value passed to a closure must live **until the end of the program**, hence the `'static` **lifetime**. The reason for this is that *spawned thread* **can outlive** scope in which it was originated.<br>
Rust has no way of knowing how long the *child thread* will run, so it **assumes** the **worst**: it assumes that *child thread* **outlives** *parent thread*.<br>
So when you pass reference to `thread::spawn` closure, it assumes this **reference can outlive original value it points to**.<br>

<br>

## `Send` constraint
The `Send` constraint is because the closure is passed **by value** from the thread where it is spawned to the new thread.
