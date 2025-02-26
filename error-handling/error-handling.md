# Table of contents
- [Table of contents](#table-of-contents)
- [Ways of errors handling](#ways-of-errors-handling)
  - [Macros panic!](#macros-panic)
- [Boxing errors](#boxing-errors)
- [Custom error type](#custom-error-type)
    - [Example](#example)
- [Macros `try!` and `?` operator](#macros-try-and--operator)
  - [Example](#example-1)

<br>

# Ways of errors handling
Generally speaking, there 2 ways of **error handling**: 
- **exceptions**; 
- **return values**. 

Rust uses **return values** approach. 

There are 2 kind of errors in Rust: 
- **recoverable**;
- **unrecoverable**. 

**Unrecoverable** errors are always symptoms of bugs.

Rust has the type `Result<T, E>` for **recoverable** errors and the `panic!` macro for **unrecoverable** error.

<br>

## Macros panic!
`panic!` macro should be used when a **program reaches** an **unrecoverable state**.<br>
This allows a program to **terminate** immediately and provide feedback to the caller of the program.<br>

```Rust
if n < 1 || n > 100 {
    panic!("Incorrect number: {}", n);
}
```

<br>

# Boxing errors
Specifying `Result<(), Box<dyn Error>>` as the return type allows pass to `Err()` **any type**, that can be converted to `Box<dyn Error>`.<br>

For example, `Box<dyn Error>` implements `From<&str> `:
```rust
impl From<&str> for Box<dyn Error> {
    fn from(err: &str) -> Box<dyn Error> {
        From::from(String::from(err))
    }
}
```

<br>

```rust
use std::{error::Error, io::ErrorKind};

// Create alise for Result<T, Box<dyn Error>>
type MyResult<T> = Result<T, Box<dyn Error>>;

enum Kind {
    String,
    IO,
    Any
}

fn string_error() -> Result<(), String> {
    Err(String::from("foo"))
}

fn io_error() -> Result<(), std::io::Error> {
    Err(std::io::Error::new(ErrorKind::InvalidData, "bar"))
}

fn any_error(kind: Kind) -> MyResult<()> {
    match kind {
        Kind::String => Ok(string_error()?),
        Kind::IO => Ok(io_error()?),
        Kind::Any => Err("fizzbazz".into()),
    }
    // Err("sdfdsf") // This leads to following error: 
    //     mismatched types
    //     expected struct `Box<dyn std::error::Error>`
    //     found reference `&'static str`
}

fn main() -> MyResult<()> {
    Ok(any_error(Kind::Any)?)
}
```

<br>

# Custom error type
Rust allows to define **custom** *error type* `E` in `Result<T, E>`.<br>

Custom error type `E`:
- **must** implement `std::fmt::Display` trait;
- **must** implement `std::error::Error` trait;
- *may* implement `std::fmt::Debug` trait;
- *may* implement `std::convert::From` trait or `std::convert::TryFrom` trait.

<br>

Through the `Display` and `Debug` traits **errors** describe themselves.

<br>

### Example
```Rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

// a test function that returns our error result
fn example(yes: bool) -> Result<(),MyError> {
    if yes {
        Err(MyError::new("ABC"))
    } else {
        Ok(())
    }
}
```

<br>

# Macros `try!` and `?` operator
The `?` operator is equivalent to `try!`. Now, `try!` is **deprecated**.<br>

`?` **unwrap** `Result` **or** perform **prematurely** /premətʃʊəʳli/ **return** from function.<br>

<br>

`expression?` **unfolds to**:
```Rust
match expression {
    Ok(value) => value,
    Err(err)  => return Err(From::from(err)),
}
```
<br>

To use `?`, **calling** and **called** functions must use `Result<T, E>` as return type.<br>
The `?` operator **automatically** converts the **error** to `Err` variant of `Result` type.<br>

<br>

## Example
```Rust
use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}
```

<br>

The `fs::read_to_string(&file_name)?` under the hood converts the `io::Error` to the type `CliError` returned by the function `open_and_parse_file`.<br>
