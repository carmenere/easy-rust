# Table of contents
- [Table of contents](#table-of-contents)
- [Stackles coroutines and generators](#stackles-coroutines-and-generators)
  - [In a nutshell](#in-a-nutshell)
  - [Underlying state machime](#underlying-state-machime)
- [Async runtime in Rust](#async-runtime-in-rust)
- [Rust API for async runtimes](#rust-api-for-async-runtimes)
  - [Trait `Future`](#trait-future)
  - [Enum `Poll`](#enum-poll)
  - [Struct `Context`](#struct-context)
  - [Leaf futures](#leaf-futures)
  - [Non-leaf futures](#non-leaf-futures)
  - [`async` keyword](#async-keyword)
  - [`.await` keyword](#await-keyword)

<br>

# Stackles coroutines and generators
## In a nutshell
An **asynchronous task** represents **deferred computation**, i.e. some operation that will be completed in the future.<br>

An **asynchronous task** can be implemented in a 2 different ways:
- as a **stackful coroutine**;
- as a **stackles coroutine**;

<br>

**Stackful coroutines** (aka **fibers**/**green threads**) is a way of representing a **resumable tasks** without any limitations: they **can be interrupted at any arbitrary points**. It is very similar how OS schedules threads. It is possible because they all have **stack** where they **store** its **state**.<br>

**Stackles coroutines** is a way of representing a **resumable tasks**, but **with limitations**.<br>
A *stackles coroutine* **doesn't** store its state on **stack**, instead it stores its state in some **data structue** and this data structure has a **set of states** it can be. Each state represents a possible yield/resume point.<br>
In other words, *stackles coroutines* is a **state machine** that **store** its **states** on some **data structure**.<br>
That's why *stackles coroutines* can be interrupted **only** at the **pre-defined points** (**yield points**) and **only** when they **explicitly yield control** to the **caller** (*another coroutine* or *scheduler*) **on they own**.<br>

**Generators** are very similar to *stackles coroutine*, but they also allow to **receive values** at the r**esume points**.<br>

Both **stackles coroutines** and **generators** represent the same underlying mechanism for creating **resumable tasks**.<br>

<br>

## Underlying state machime

<br>

# Async runtime in Rust
Rust uses **stackles coroutines** to implement **asynchronous tasks**.<br>
In Rust **stackles coroutine** is called **future**.<br>

What **async runtime** does?<br>

**Async runtime** in Rust uses **poll-based approach** in which a **future**  has **3 phases**:
- **poll phase**: **future** makes progress until it completes or reaches a point where it can no longer make progress;
- **wait phase**: reactor registers a **future** and maps it with *event source* to be sure that it can wake the **future** when that event is ready;
- **wake phase**: the **future** is woken up when the event happens, executor schedule the **future** to be polled again to make further progress;

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
- `tokio`;
- `async-std`;
- `smol`;

<br>

A fully working **async runtime** in Rust consists of:
- **reactor** (responsible for notifying about **I/O events**);
- **executor** (aka **scheduler**);
- **future** (a **resumable task**);

<br>

# Rust API for async runtimes
In Rust **future** is anything that implements a `Future` trait.<br>

<br>

## Trait `Future`
```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

<br>

Future’s `poll()` method always returns **immediately** one of `Poll` variant:
- `Poll::Ready(T)`: means `Future` is **ready** to return value, once `Future` has returned variant `Ready(T)` it will **never** be polled again;
- `Poll::Pending`: means `Future` is **not** ready yet;

<br>

## Enum `Poll`
```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

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

## Leaf futures
**Runtimes** create **leaf futures**.<br>
**Leaf future** represents a **resource** such as a **socket**, in other words it acts like **subscriber** on some system **I/O operation**.<br>

Example:
```rust
let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080");
```

<br>

## Non-leaf futures
**Non-leaf futures** are futures that we as users of a runtime write ourselves using `async` keyword.<br>
**Non-leaf futures** represent a **task** that can be run on the **executor**, they **don't** represent an I/O resource.<br>

**Non-leaf** `Future` contains `.await` calls to **nested non-leaf** `Futures`. The **last** `Future` in this chain is a **leaf** `Future`.<br>

The code between `await` runs in the **same thread** where **executor** runs. Any **CPU-intensive** tasks can **block** executor from handling new requests.<br>
More executors provide `spawn_blocking` to solve this problem. These method send the task to a **thread pool** created by the runtime where you can run **CPU-intensive** tasks.

<br>

## `async` keyword
`async` keyword transforms *function* or *block of code* into some **data structure** which implements `Future` trait.<br>
This **data structure** actually implements **state machine** inside `poll` method of `Future` trait.<br>

<br>

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

<br>

**This code**:
```rust
async fn afunc(i: i32) -> i32 {
    i
}

#[tokio::main]
async fn main() {
    let result = afunc(10).await;
    println!("{}", result);
}
```
**will be desugared to**:
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct AFunc {
    i: i32,
}

impl Future for AFunc {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<i32> {
        Poll::Ready(self.i)
    }
}

#[tokio::main]
async fn main() {
    let afunc = AFunc{ i: 100};
    // let result = block_on(my_fut);
    let result = afunc.await;
    println!("{}", result);
}
```

Under the hood `afunc` is represented as **data structure** `AFunc`.<br>

<br>

So, Rust implicitly converts `async fn f() { } -> T` in `Future` type: `impl Future<Output = T>`.<br>
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
