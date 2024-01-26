# Thiserror and Anyhow
There are 2 useful crates to handle errors in Rust:
- **Anyhow**
- **Thiserror**

<br>

**Anyhow** just **wraps any** *error type* `E` with `Box<dyn Error + Send + Sync>`, i.e., **anyhow** abstracts from any kind of returned error and simplifies writing of code.<br>
**Thiserror** is a **macro** to derive `std::error::Error`, `std::fmt::Display`, `std::convert::From` traits.<br>
Anything that you write with **thiserror** could sensibly be written by hand.<br>

<br>

Recommendations:
- use **anyhow** if you **don't care** what error type your functions return. This is common in application code. 
- use **thiserror** if you **writes a library** that wants to design your **own dedicated** error type(s) so that on failures the caller gets exactly the information that you choose.

<br>

# Anyhow
The most useful feature of `anyhow` is its `Result<T>` type.<br>
Also it provides `anyhow!` macro. It will convert your `&str` or **any type** implementing `Debug` and `Display` to an **error**.<br>
`anyhow` provides a `Context` trait that gives access to a `context` method you can use on `Result` (and `Option`) types. This method will add the information you specify in the error.<br>

```rust
use anyhow::{Result, anyhow};

enum Kind {
    String,
    IO,
    Any
}

fn string_error() -> Result<()> {
    Err(anyhow!("string_error").context("string_error"))
}

fn io_error() -> Result<()> {
    Err(anyhow!("io_error").context("io_error"))
}

fn any_error(kind: Kind) -> Result<()> {
    match kind {
        Kind::String => Ok(string_error()?),
        Kind::IO => Ok(io_error()?),
        Kind::Any => Err(anyhow!("fizzbazz").context("any_error")),
    }
}

fn main() -> Result<()> {
    Ok(any_error(Kind::Any)?)
}
```