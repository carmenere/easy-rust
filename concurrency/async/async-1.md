# Table of contents
- [Table of contents](#table-of-contents)
- [Async basics](#async-basics)
- [`join!`, `select!` and `try_join!`](#join-select-and-try_join)
    - [`join!`](#join)
    - [`select!`](#select)
    - [`try_join!`](#try_join)

<br>

# Async basics
**async Rust** is possible through a trait called `Future`. The `Future` trait is well named as it refers to a **value** that **will be available** *at some time in the future*:
```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

**async functions** begin with `async fn` instead of `fn`:
```rust
fn give_8() -> u8 {
    10
}

async fn async_give_8() -> u8 {
    10
}
```

Both functions return a `u8` but in different ways. The **regular function** returns one right away, but the `async fn` returns **something that will be** a `u8` when it’s done.<br>
And because it’s **async**, if it’s **not** done yet, your code **can do other work** as it waits.<br>

So, `async fn async_give_8()` actually returns `impl Future<Output = u8>`, not `u8`.<br>

The main way to **poll a future** in Rust is by adding the `.await` keyword to future:
```rust
let some_number = async_give_8().await;
```

<br>

But `.await` is **only** allowed inside `async` **functions** and **blocks**:
- *regular functions* **can’t await** async functions, so if you have a regular function that needs to call an async function, it will become async, too;
- `async` functions **can** call *regular functions*;
  - this is usually no problem, but **remember** that *regular functions* **may block the thread** until they are done;

<br>

You can make `main` into an `async main` through **Tokio** by adding `#[tokio::main]` above it. Also, you need to enable at least 2 **features**:
- `macros` to bring in the macro above main;
- `rt-multi-thread` to enable Tokio’s **multithreaded run time**;

<br>

**Example**:
```rust
use tokio;

async fn async_give_8() -> u8 {
    8
}

#[tokio::main]
async fn main() {
    let some_number = async_give_8().await;
    let second_number = async_give_8().await;
    println!("{some_number}, {second_number}");
}
```

<br>

And **expanded code**:
```rust
fn main() {
    let body = async {
        let some_number = async_give_8().await;
        let second_number = async_give_8().await;
        {
            ::std::io::_print(format_args!("{0}, {1}\n", some_number, second_number));
        };
    };
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
```

<br>

# `join!`, `select!` and `try_join!`
**Example**:
```rust
use std::time::Duration;
use rand::{rng};
use rand::prelude::*;
use tokio;
use tokio::time::sleep;

async fn wait_and_give_u8(num: u8) -> u8 {
    let mut wait_time = rng().random_range(0..100);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}

#[tokio::main]
async fn main() {
    let num1 = wait_and_give_u8(1).await;
    let num2 = wait_and_give_u8(2).await;
    let num3 = wait_and_give_u8(3).await;
    println!("{num1}, {num2}, {num3}");
}
```

**Output**:
```bash
Got a number! 1
Got a number! 2
Got a number! 3
1, 2, 3
```

In the above we await futures **sequentially**: the result **will always be** `1`, then `2`, and then `3`.<br>

<br>

### `join!`
Instead of `.await` on each, we’ll use `join!`, which will **poll them all** *at the same time*
**Example**:
```rust
use std::time::Duration;
use rand::{rng};
use rand::prelude::*;
use tokio;
use tokio::time::sleep;
use tokio::join;

async fn wait_and_give_u8(num: u8) -> u8 {
    let wait_time = rng().random_range(100..500);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}

#[tokio::main]
async fn main() {
    let nums = join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );
    println!("{:#?}", nums);
}
```

**Output**:
```bash
Got a number! 2
Got a number! 3
Got a number! 1
(
    1,
    2,
    3,
)
```

<br>

### `select!`
The `select!` polls all futures *at the same time* and returns the first one that is completed.<br>

This macro uses its own syntax that looks like this: `name_of_variable = future => handle_variable`. This is particularly useful when polling futures that **don’t** return the *same type* and you can **modify the output** to return the *same type*.<br>

**Example**:
```rust
use std::time::Duration;
use rand::{rng};
use rand::prelude::*;
use tokio;
use tokio::time::sleep;
use tokio::select;

async fn wait_and_give_u8(num: u8) -> u8 {
    let wait_time = rng().random_range(100..500);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    num
}

async fn wait_and_give_string(num: u8) -> String {
    let wait_time = rng().random_range(100..500);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    format!("{num}")
}

#[tokio::main]
async fn main() {
    let nums = select!(
        n1 = wait_and_give_string(1) => n1,
        n2 = wait_and_give_string(2) => n2,
        n3 = wait_and_give_u8(3) => format!("{n3}"),
        timeout = sleep(Duration::from_millis(200)) => format!("timeout")
    );
    println!("{:#?}", nums);
}
```

Finally, we’ll add a `timeout` to the `select!`. If neither of the first three return before **200** milliseconds have passed, the `select!` will **finish** with a *timeout message*.<br>

**Output 1**:
```bash
Got a number! 1
"1"
```

**Output 2**:
```bash
"timeout"
```

<br>

### `try_join!`
The `try_join!`
- returns `Ok` if **all** futures are completed **successfuly**;
- returns `Err` **immediately** when future **fails**;

<br>

**Example**:
```rust
use std::time::Duration;
use rand::{rng};
use rand::prelude::*;
use tokio;
use tokio::time::sleep;
use tokio::try_join;

async fn wait_and_give_u8(num: u8) -> Result<u8, Box<dyn std::error::Error>> {
    let wait_time = rng().random_range(100..500);
    sleep(Duration::from_millis(wait_time)).await;
    println!("Got a number! {num}");
    if num < 10 {Ok(num)} else {Err(format!("num={num}").into())}
}

#[tokio::main]
async fn main() {
    let nums = try_join!(
        wait_and_give_u8(1),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );
    println!("{:#?}", nums);

    let nums = try_join!(
        wait_and_give_u8(10),
        wait_and_give_u8(2),
        wait_and_give_u8(3)
    );
    println!("{:#?}", nums);
}
```

**Output**:
```bash
Got a number! 2
Got a number! 1
Got a number! 3
Ok(
    (
        1,
        2,
        3,
    ),
)

Got a number! 2
Got a number! 10
Err(
    "num=10",
)
```
