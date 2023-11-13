# Support for `async` in Rust
What **async runtime** does? **Async runtime**:
- **execute** async code;
- **processes** on IO events;
- **spawns** tasks;

<br>

But, **Rust doesn’t provide any async runtime**.<br>

Rust **only** provides:
- `async`/`.await` syntax (it is built-in to language);
- the **fundamental types** (provided by `std` crate):
  - `enum Poll`;
  - `trait Future`;
  - `struct Context<'a>`;

<br>

There are several popular crates that implement **async runtime** for Rust:
- `tokio`
- `async-std`
- `smol`

<br>

## Enum `Poll`
Future’s `poll()` method returns **enum** `Poll`, whose variants are
- `Ready<T>`  if `Future` is **ready** to return value, once `Future` has returned variant `Ready(T)` it will **never** be polled again.
- `Pending` if `Future` is **not** ready yet;

<br>

```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

<br>

## Trait `Future`
`Future` type represents **deferred computation**, i.e., result is ready at some point in the future.<br>

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

<br>

`Output` is associate type of Future’s result.<br>
Future’s `poll()` method always returns **immediately** one of `Poll` variant:
- `Poll::Ready(T)`
- `Poll::Pending`

<br>

## Struct `Context`
`Context` is a wrapper for `Waker`.<br>
`Waker` is a type that has a `wake()` method that will be called by the **Reactor** to notify the **Executor** that particular `Future` is ready to be polled.<br>

```rust
pub struct Context<'a>{
    waker: &'a Waker,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>,
}
```

<br>

## `async` keyword
`async` keyword defines **async block** or **async function**:
- **async function**:
```rust
async fn async_function() -> String {
    "ABCDEF".to_string()
}
```
- **async block**:
```rust
async {
    "ABCDEF".to_string()
}

async move {
    "ABCDEF".to_string()
}
```

This code:
```rust
async fn async_function() -> String {
    "ABCDEF".to_string()
}
```

is equal to:
```rust
use std::future::Future;

fn async_function() -> impl Future<Output = String> {
    async {
        "ABCDEF".to_string()
    }
}

#[tokio::main]
async fn main() {
    let result = async_function().await;
    println!("{}", result);
}
```

`async` keyword transforms block of code into **state machine** that implement `trait Future`.<br>

This code:
```rust
async fn my_future(i: i32) -> i32 {
    i
}

#[tokio::main]
async fn main() {
    let result = my_future(10).await;
    println!("{}", result);
}
```

<br>

will be desugared to:
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyFuture {
    i: i32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<i32> {
        Poll::Ready(self.i)
    }
}

#[tokio::main]
async fn main() {
    let my_future = MyFuture{ i: 100};
    // let result = block_on(my_fut);
    let result = my_future.await;
    println!("{}", result);
}
```

So, Rust implicitly treats `async fn f() { } -> T` as function that returns `Future` type: `impl Future<Output = T>`.<br>
The future’s specific type `impl Future<Output = T>` is generated **automatically** by the compiler, based on the function’s body and arguments.<br>

<br>

## `.await` keyword
`.await` is special syntax called **await expression**. `.await` **polls** `Future`.<br>
Switching from one async task to another happens only at `.await` when awaited `Future` returns `Pending`.<br>
Pending `Future` **yields control** back, allowing other `Futures` to make progress.<br>

`.await` is added to every `async` function call:
```rust
async fn async_function() -> String {
    "ABCDEF".to_string()
}

#[tokio::main]
async fn main() {
    let result = async_function().await;
    println!("{}", result);
}
```

<br>

# Executor/Reactor pattern
At the **top** of the program is the **Executor**. The **Executor** is just a **scheduling algorithm** that executes the `Futures` by calling `poll()` on them.<br>
- **Executer** provides special API called **spawner**: spawner produces new tasks and puts them into *Executor's* **task queue**;
- **Executor** provides the runtime that iterates over its **task queue** and calls `poll()` on `Futures` until `Futures` return the `Ready` state;

<br>

At the **bottom** of the program is **Reactor** (aka **source of system IO events**).<br>
The **Reactor** notifies the **Executor** which task is ready to continue executing.<br>
**Reactor** is an **interface** between **Executor** and **OS**.<br>

**Reactor** provides **subscription API** for **external events**:
- IO events;
- IPC;
- timers;
- interrupts;

<br>

In async runtime, **subscribers** are `Futures` requesting **low level IO operations**, i.e., **read from socket**, **write to socket** and so on.<br>

<br>

So:
- *Executor* **schedules** `Futures` that are **ready to be polled**.
- *Reactor* **waits** **IO events** and **wakes** `Futures` that are bound to events **when events happen**.
- **Event loop** = **Executor** + **Reactor**.

<br>

# `Future` life cycle
Every `Future` transits through different phases during its life cycle.<br>

### Spawning
**Spawning** is registering a **top-level** `Future` at the **Executor**.<br>
**Top-level** `Future` contains `.await` calls to **nested** `Futures` and such **nested** `.await` calls form **chain** of `Futures`.<br>
The **last** `Future` in this chain (aka **leaf** `Future`) is the `Future` that acts like **subscriber** on some system **IO operation**.<br>

### Polling
**Executor** fetches `Future` from its **task queue** and call `poll(cx)` method on it where `cx` is `Context`.<br>
`Context` is **wrapper** for `Waker` and just contains a reference to a `Waker`.<br>
The result of the `poll(cx)` method represents the the state of the `Future`.<br>

### Waiting
When the **Executor** calls `poll()` on a `Future`, that `Future` will return either `Ready` or `Pending`:
- If `Future` returns `Ready(T)` then the `.await` will return `T` and the **Executor** removes it from the **task queue**.
- If `Future` returns `Pending` then the **Executor** removes it from the **task queue**, but **Reactor** will notify **Executor** when particular `Future` will become ready to be polled again. This is where the **Waker API** comes in.

### Waking
When event happens, **Reactor** calls `wake()` method on `Waker`, which puts `Future` that is bound to this event into *Executor's* **task queue**.<br>

<br>

# Waker API
The **Waker API** connects *Executor* and *Reactor*.<br>
Every time **Executor** calls `poll(cx)` method it passes a `Context` to it. `Context` provides access to a `Waker`, i.e., it wraps `Waker`.<br>
The reason `poll()` takes `Context` instead `Waker` is to has ability add other things to `Context` in future.<br>

Requirements to `Waker` type:
- the `Waker` type cannot be Generic because it is need to be passed through arbitrary `Futures`;
- the `Waker` type must implement `.wake()` method;
- the `Waker` type must implement `Clone` trait;

<br>

`Futures` can be **nested** and `Waker` object is passed along chain of nested `Futures` until it reaches the **source of event** (**Reactor**), then `Waker` is being registered in **Reactor**.<br>

If `Future` returns `Poll::Pending` then `Waker`, that was passed inside `Context`, is registered in **Reactor** and bound to **event id** (e.g. **file descriptor**).<br>
When event occurs **Reactor** calls `wake()`.

To poll `Futures` it is necessary to create a `Waker`. `Waker` is responsible for scheduling a task to be polled again once `wake()` is called.<br>
The easiest way to create a new `Waker` is by implementing the `ArcWake` trait and then using the `waker_ref()` or `into_waker()` functions to turn an `Arc<impl ArcWake>` into a `Waker`.<br>

<br>

## Ways to implement `wake()`
### Using task id
In this approach the `Waker` is **Task id** and the *Executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
Also Executor stores set of Tasks as `HashMap<Task_id, Task>`.<br>
When event occurs, **Reactor** calls `wake()` and it appends **Task** id to *Executor’s* **task queue**.<br>

### Using reference counter
In this approach the `Waker` is `Arc<Task>` and the *Executor’s* **task queue** is `Vec<Arc<Task>>`.<br>
When event occurs, **Reactor** calls `wake()` and it push `Arc<Task>` to *Executor’s* **task queue**.<br>

<br>

# Pinning
If **pointer** is wrapped into `Pin<P>`, it means the value pointer points to will **no longer move**.<br>
`Pin` allows to create **immovable** `Futures`.<br>
Also there is marker trait `Unpin` that **disable** such restirction.<br>

The `poll()` method requires the future be passed as `Pin<&mut Self>` value.<br>
So, you cannot poll future until you’ve constructed a `Pin` wrapper for it, and once you have done that, the future can’t be moved.<br>
This restrictions for `Pin` type are implemented in code-generated `Future` implementation.

Pin type:
```rust
pub struct Pin<P> {
    pointer: P,
}
```

<br>

There is `Box::pin(value: T)` constructor to **make reference pinned**: it takes ownership of value of type `T` and returns `Pin<Box<T>>`.<br>
`Pin<Box<T>>` implements `From<Box<T>>`, so `Pin::from(value: T)` takes ownership of value of type `T` and returns `Pin<Box<T>>`.

