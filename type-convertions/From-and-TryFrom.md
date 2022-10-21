# Trait ``From``
Path in **std** is ``std::convert::From``.<br>
Trait ``From`` is used to convert value **from** type ``S`` **to** type ``D`` and **must** be implemented on ``D`` type.<br>
Trait ``From`` **must** **not** **fail**. If the conversion **can** **fail**, use ``TryFrom``.<br>

**Declaration** of ``From``:
```Rust
pub trait From<T> {
    fn from(T) -> Self;
}
```

Method ``from()`` performs the conversion.<br>
Traits ``From`` and ``Into`` are **connected**: implementing ``impl From<S> for D`` **automatically** implements ``impl Into<D> for S``, but not vise versa. The compiler is **unable** to **infer** destination type ``D`` for ``S`` when ``.into()`` is used.

<br>

### Example
```Rust
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num1 = Number::from(30);
    println!("num: {:?}", num1);

    let num2 = 5.into();
    println!("num: {:?}", num2);
}
```

**Output**:
```bash
cargo run 
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0282]: type annotations needed
  --> src/main.rs:22:9
   |
22 |     let num = 5.into();
   |         ^^^ consider giving `num` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playrs` due to previous error
```

<br>

**BUT**:
```Rust
fn main() {
    let num2: Number = 5.into();
    println!("num: {:?}", num2);
}
```

**Output**:
```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/playrs`
num: Number { value: 30 }
num: Number { value: 5 }
```

<br>

# Error handling
The ``From`` is also very useful when performing **error handling**.<br>
By converting underlying error types to our own **custom** error type that encapsulates the underlying error type, we can return a single error type without losing information on the underlying cause.<br>
The ``?`` operator **automatically** converts the underlying error type **to** our **custom** error type.<br>

<br>

### Example
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

# Trait ``TryFrom``
Path in **std** is ``std::convert::TryFrom``.<br>
``TryFrom<T>`` returns ``Result<T, E>``.<br>

**Declaration** of ``TryFrom``:
```Rust
pub trait TryFrom<T> {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}
```

<br>

### Example
```Rust
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts value superior than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}
```
