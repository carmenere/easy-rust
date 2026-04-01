# Table of contents
- [Table of contents](#table-of-contents)
- [Ways of errors handling](#ways-of-errors-handling)
  - [Macros panic!](#macros-panic)
- [Boxing errors](#boxing-errors)
- [Custom error type](#custom-error-type)
    - [Example](#example)
- [Macros `try!` and `?` operator](#macros-try-and--operator)
- [Examples](#examples)
  - [Example 1](#example-1)
  - [Example 2](#example-2)
  - [Example 3](#example-3)

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
Consider function that returns `Result<String, Box<dyn Error>>`. By returning a `Box<dyn Error>`, we can return a `Box` that **holds anything** that implements the `Error` trait.<br>

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

If we didn’t have a `Box<dyn Error>` and wrote just `Result<String, Error>` the compiler gives us error: `Result<String, Error> doesn't have a size known at compile-time`. Indeed, a trait `Error` can be implemented on many types and compiler doesn't know exact size of type.<br>

Consider a function with **two types of possible errors**, for example, an error when parsing into an `i32` and an error when parsing into an `f64`. But we want to use `?` operator inside this function and thus we can use only **one** `Error` inside returning `Result`.<br>
We can use `Result<f64, Box<dynError>>` as returning type:
```rust
use std::error::Error;

fn parse_numbers(int: &str, float: &str) -> Result<f64, Box<dyn Error>> {
    let num_1 = int.parse::<i32>()?;
    let num_2 = float.parse::<f64>()?;
    Ok(num_1 as f64 + num_2)
}
fn main() {
    let n = parse_numbers("8", "ninepointnine");
    println!("{:?}", n);
    let n = parse_numbers("8", "9.");
    println!("{:?}", n);
}
```

**Output**:
```rust
Err(ParseFloatError { kind: Invalid })
Ok(17.0)
```

<br>

**Example**:
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

After anything that returns a `Result` or `Option`, you can add `?`. This will:
- automatically **pulls out** the `Ok` value from a `Result`;
- if the value inside `Result` is `Err` it will **exit the function early** (**early return**) and return `Err` of the `Result` of function's returning type

<br>

We **don’t** need to write `std::result::Result` because `Result` is **always in scope**.<br>

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

A function that returns `Result<T, ErrorOuter>` can only use `?` on a value of type `Result<U, ErrorInner>` if `ErrorOuter` and `ErrorInner` are the same type or if `ErrorOuter` implements `From<ErrorInner>`.<br>
A common alternative to a `From` implementation is `Result::map_err`, especially when the conversion only happens in one place.<br>

<br>

# Examples
## Example 1
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

<br>

## Example 2
```rust
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<u32, ParseIntError> 
{
    let parsed_number = input.parse::<u32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn main() {
    let input = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in input {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}
```

<br>

## Example 3
Imagine that you want to *take some bytes*, turn *them into* a `String`, and then *parse* it into a *number*. **First**, you need to successfully create a `String` from the bytes using a method called `String::from_utf8()`. **And then** it needs to successfully parse into a number.<br>

**The problem is the return type**:
- if `String::from_utf8()` **fails**, it will return `Err(FromUtf8Error)`;
- and if `string.parse()` **fails**, it will return an `Err(ParseIntError)`;
- but we **can’t return** a `Result<i32, ParseIntError or FromUtf8Error>`;

What must be in the place of `????`:
```rust
use std::num::ParseIntError;
use std::string::FromUtf8Error;

fn turn_into_string_and_parse(bytes: Vec<u8>) -> Result<i32, ????> {
    let num = String::from_utf8(bytes)?.parse::<i32>()?; // Two possible errors can be returned here
    Ok(num)
}
```

<br>

The `?` operator **automatically** converts the **error** to `Err` variant of `Result` type.<br>
