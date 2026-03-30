# Table of contents
- [Table of contents](#table-of-contents)
- [Thiserror and Anyhow](#thiserror-and-anyhow)
- [Anyhow](#anyhow)
  - [`anyhow`](#anyhow-1)
  - [`thiserror`](#thiserror)

<br>

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

<br>

## `anyhow`
It would be nice to have a **single error type** that’s easy to use. This is what `anyhow` is used for.<br>
Another common way to do this is to use `Box<dyn Error>`.<br>
Anyhow works with **any error type** that has an impl of `std::error::Error`.<br>

<br>

Use `fn foo () -> Result<T, anyhow::Error> {}` or **equivalently** `fn foo () -> anyhow::Result<T> {}`, as the return type of any **fallible function**:
```rust
pub fn parse_u16_1(input: &[u8]) -> anyhow::Result<u16> {
    Ok(std::str::from_utf8(input)?
        .parse::<u16>()?)
}

pub fn parse_u16_2(input: &[u8]) -> Result<u16, anyhow::Error> {
    Ok(std::str::from_utf8(input)?
        .parse::<u16>()?)
}

fn main() {
    println!("{:?}", parse_u16_1("abc".as_bytes()));
    println!("{:?}", parse_u16_1("444".as_bytes()));
    println!("{:?}", parse_u16_2("444444".as_bytes()));
}
```

**Output**:
```rust
Err(invalid digit found in string)
Ok(444)
Err(number too large to fit in target type)
```

<br>

We can also bring in the `anyhow!` **macro**, which makes a quick `anyhow::Error` from a **string** or an **error type**:
```rust
#[derive(Debug)]
pub enum MyError {
    A,
    B
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::A => write!(f, "MyError::A error"),
            MyError::B => write!(f, "MyError::B error"),
        }
    }
}

pub fn ret_myerr(err: MyError) -> anyhow::Result<u16> {
    Err(anyhow::anyhow!(err))
}

pub fn ret_str_err(msg: &str) -> anyhow::Result<u16> {
    Err(anyhow::anyhow!(msg.to_owned()))
}

fn main() {
    println!("{:?}", ret_myerr(MyError::A));
    println!("{:?}", ret_str_err("Some error string"));
}
```

<br>

## `thiserror`
You use `#[derive(Error)]` on top and then another `#[error]` attribute above each variant if we want a message. This will **automatically implement** `Display`.<br>

**Note**, the `error` attribute has the same format as when you use the `format!` macro.<br>

**Example**:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum SystemError {
    #[error("Got error: {0}")]
    A(String),
    #[error("Wrong number: {0}")]
    B(u8),
}

fn main() {
    println!("{}", SystemError::B(8));
    println!("{}", SystemError::A("foo".to_owned()));
}
```

**Output**:
```bash
Wrong number: 8
Got error: foo
```

<br>