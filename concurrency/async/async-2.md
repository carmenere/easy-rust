# Table of contents
- [Table of contents](#table-of-contents)
- [Stackles coroutines and generators](#stackles-coroutines-and-generators)
- [Async runtime in Rust](#async-runtime-in-rust)
- [Rust API for async runtimes](#rust-api-for-async-runtimes)
  - [Trait `Future`](#trait-future)
  - [Enum `Poll`](#enum-poll)
  - [Struct `Context`](#struct-context)
  - [Leaf futures](#leaf-futures)
  - [Non-leaf futures](#non-leaf-futures)
  - [Top-level futures](#top-level-futures)
  - [`async` keyword](#async-keyword)
  - [`.await` keyword](#await-keyword)
- [Future as a state machine](#future-as-a-state-machine)
  - [Hand made leaf futures](#hand-made-leaf-futures)
  - [Compiler-generated state machines](#compiler-generated-state-machines)
  - [Top-level futures (aka tasks)](#top-level-futures-aka-tasks)

<br>

# Stackles coroutines and generators
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

# Async runtime in Rust
Rust uses **stackles coroutines** to implement **asynchronous tasks**.<br>
In Rust **stackles coroutine** is called **future**.<br>

What **async runtime** does?<br>

**Async runtime** in Rust uses **poll-based approach** in which a **future**  has **3 phases**:
- **poll phase**: **future** makes progress until it completes or reaches a point where it can no longer make progress;
- **wait phase**: **reactor** registers a **future** and maps it with *event source* to be sure that it can wake the **future** when that event is ready;
- **wake phase**: the **future** is woken up when the event happens, **executor** schedules the **future** to be polled again to make further progress;

<br>

But, **Rust doesn’t provide any async runtime**.<br>

Rust **only** provides:
- `async`/`.await` syntax (it is built-in to language);
- the **fundamental types** (provided by `std` crate):
  - `enum Poll`;
  - `trait Future`;
  - **Waker API** and `struct Context<'a>`;

<br>

There are several popular crates that implement **async runtime** for Rust:
- `tokio`;
- `async-std`;
- `smol`;

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

## Top-level futures
**Top-level futures** are actually **tasks** that **executor schedules**.<br>

<br>

## `async` keyword
**Async functions** can call **normal functions**, but **normal functions** **can't** call **async function**.<br>
The `async` keyword transforms *function* or *block of code* into some **data structure** which implements `Future` trait.<br>
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
    let result = afunc(100).await;
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
The `.await` is special syntax called **await expression**.<br>
Only **futures** (instances of types that implement `Future` trait) can be polled with `.await`.<br>

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

# Future as a state machine
## Hand made leaf futures
Consider example.

**Http**:
```rust
pub struct Http;

impl Http {
    pub fn get(req: Request) -> impl Future<Output = String> {
        HttpFuture::new(req)
    }
}
```

The `Http` is a **http client**. Its `get()` methods returns `HttpFuture` which implements `Future`.

<br>

**HttpFuture**:
```rust
struct HttpFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    req: Request,
}

impl HttpFuture {
    fn new(req: Request) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            req: req,
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect(self.req.host.clone()).unwrap();
        stream.set_nonblocking(true);
        let mut stream = mio::net::TcpStream::from_std(stream);
        stream.write_all(self.req.get().as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}
```

<br>

The `HttpFuture` is an example of a **leaf future**.<br>
The `HttpFuture` has 2 methods:
- `new()` which sets the initial state;
- `write_request()` which sends the **GET** request to the server;

<br>

**impl Future for HttpFuture**:
```rust
impl Future for HttpFuture {
    type Output = String;
    
    fn poll(&mut self) -> Poll<Self::Output> {
        if self.stream.is_none() {
            self.write_request();
            self.stream.as_ref().map(|s| {
                let _ = s.peer_addr().map(|v| {
                    println!("Sending request to: {:?}", v)
                });
            });

            return Poll::Pending;
        }

        let mut buff = vec![0u8; 4096];

        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break Poll::Ready(s.to_string())
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break Poll::Pending;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) => panic!("{e:?}")
            }
        }
    }
    
}
```

<br>

On the **first poll** the `poll()` method returns `Poll::Pending`, so `HttpFuture` will be polled again at least one time.

We can see that it is a very simple **state machine** with **3 states**:
- **not started**, indicated by `self.stream` being `None`;
- **pending**, indicated by `self.stream` being `Some` and `stream.read()` returning `WouldBlock`;
- **resolved**, indicated by `self.stream` being `Some` and `stream.read()` returning `0` bytes;

<br>

## Compiler-generated state machines
**Async function** is a function prefixed with `async` keyword.<br>
Every **async function** will be **rewritten** *by compiler* to a **state machine**. The return type `T` of **async function** will be rewritten to `impl Future<Output = T>`.

<br>

Let's write some **async function**:
```rust
async fn main() {
    println!("Starting");
    let req = Request::new("/", "HTTP/1.1","ya.ru:80", "close");
    let resp = Http::get(req).await;
    println!("Response 1: {}", resp);
    let req = Request::new("/", "HTTP/1.1","google.com:80", "close");
    let resp = Http::get(req).await;
    println!("Response 2: {}", resp);
}
```

<br>

The compiler will **rewrite** it into something like this:
```rust
struct MainCoroutine {
    state: State,
    req1: Request,
    req2: Request,
}

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl MainCoroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
            req1: Request::new("/", "HTTP/1.1","ya.ru:80", "close"),
            req2: Request::new("/", "HTTP/1.1","google.com:80", "close")
        }
    }
}

impl Future for MainCoroutine {
    type Output = ();
    
    fn poll(&mut self) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Coroutine starting");
                    let fut = Box::new(Http::get(self.req1.clone()));
                    self.state = State::Wait1(fut);
                },
                State::Wait1(ref mut fut) => {
                    match fut.poll() {
                        Poll::Ready(resp) => {
                            println!("Response 1: {}", resp);
                            let fut = Box::new(Http::get(self.req2.clone()));
                            self.state = State::Wait2(fut);
                        },
                        Poll::Pending => break Poll::Pending,
                    }
                },
                State::Wait2(ref mut fut) => {
                    match fut.poll() {
                        Poll::Ready(resp) => {
                            println!("Response 2: {}", resp);
                            self.state = State::Resolved;
                            break Poll::Ready(())
                        },
                        Poll::Pending => break Poll::Pending,
                    }
                },
                State::Resolved => {
                    panic!("The resolved future is polled!")
                },
            }
        }
    }
}
```

<br>

Every **async function** can be defined as a **struct** to store some **values** and **enum** to store **state**.<br>
Compiler sets initial state for such state machine `state: State::Start`.<br>
If the **future** returns `Poll::Pending` or `Poll::Ready` we bubble that up to the caller by breaking out of the loop.<br>

<br>

## Top-level futures (aka tasks)
It is **not** possible to make `main` function `asyc`. The `main` function **must** be decorated by **runtime**: 
```rust
#[tokio::main]
async fn main() {

}
```

Under the hood the **runtime** rewrites the `main` function like this:
```rust
pub fn main() {
    let mut future = MainCoroutine::new();

    loop {
        match future.poll() {
            Poll::Ready(_) => break,
            Poll::Pending => {
                println!("Schedule other task");
            },
        }

        println!("Sleep ... ");
        thread::sleep(Duration::from_millis(100));
        println!("Wake up ... ");
    }
}
```

<br>

**To run futures concurrently** there is `join_all` function. It takes a **collection of futures** and polls them all **simultaneously**:
```rust
pub struct JoinAll<F: Future> {
    futures: Vec<(bool, F)>,
    finished_count: usize,
}

pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, f)).collect();
    JoinAll{
        futures,
        finished_count: 0
    }
}

impl<F> Future for JoinAll<F>
where F: Future {
    type Output = ();

    fn poll(&mut self) -> Poll<Self::Output> {
        for (finished, fut) in self.futures.iter_mut() {
            if *finished {
                continue;
            }

            match fut.poll() {
                Poll::Ready(_) => {
                    *finished = true;
                    self.finished_count += 1;
                },
                Poll::Pending => continue,
            }
        }

        if self.finished_count == self.futures.len() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
```

<br>

In the example above we simply throw the value away.<br>
Tokio's `join_all` implementation puts all the returned values in a `Vec<T>` and returns them all when the `JoinAll` futures is resolved.