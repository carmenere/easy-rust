# Threads
An executing Rust program consists of a collection of **native OS threads**, each with their **own stack** and **local state**.<br>
When the **main thread** of a Rust program **terminates**, the **entire program shuts down**, even if other threads are still running.<br>
So, the spawned thread **may outlive** the caller *unless the caller thread is the main thread*. The **whole process** is terminated when the **main thread finishes**.

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
use std::thread;

let other_thread = thread::spawn(|| {
    thread::current().id()
});

let other_thread_id = other_thread.join().unwrap();
assert!(thread::current().id() != other_thread_id);
```

<br>

By default, *spawned threads* are **unnamed**.<br>
To **specify** a name for a thread, build the thread with `Builder` and pass the desired thread name to `Builder::name`.<br>
To **retrieve** the thread name from within the thread, use `Thread::name`.<br>

<br>

## `JoinHandle` type
Due to platform restrictions, it is not possible to `Clone` this handle: the ability to **join a thread** is a uniquely-owned permission.<br>
This struct is created by the `thread::spawn` function and the `thread::Builder::spawn` method.<br>

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
The **join handle** provides a `join` method that can be used to **join the spawned thread**.<br>
If the *spawned thread* panics, `join` will return an `Err` containing the argument given to `panic!`.<br>

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
use thread;

let computation = thread::spawn(|| {
    // Some expensive computation.
    42
});

let result = computation.join().unwrap();
println!("{result}");
```

<br>

```Rust
use std::thread;

let builder = thread::Builder::new();

let handler = builder.spawn(|| {
    // thread code
}).unwrap();

handler.join().unwrap();
```

<br>

## Constraints
#### `'static` constraint
The `'static` constraint means that the **closure** and its **return value** must have a **lifetime** of the **whole program execution**.<br>
The reason for this is that threads **can outlive** the lifetime they have been created in.<br>
Indeed if the thread and its return value can outlive their caller, we need to make sure that they will be valid afterwards, and since we **can’t know** when it will return we need to have them valid **as long as possible**, that is **until the end of the program**, hence the `'static` **lifetime**.

<br>

#### `Send` constraint
The `Send` constraint is because the closure will need to be passed **by value** from the thread where it is spawned to the new thread.

<br>

# `park` and `unpark`
`std::thread::park` blocks the **current thread**, which can then be *resumed* **from another thread** by calling the `unpark` method on the blocked **thread’s handle**.

<br>

## Example
```Rust
use std::thread;
use std::time::Duration;

let parked_thread = thread::Builder::new()
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
```
