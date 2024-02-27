# Table of contents
- [Table of contents](#table-of-contents)
- [Result](#result)
- [Result alias](#result-alias)
    - [Example](#example)

<br>

# Result
`Result` express **ability** of **error** through `enum`.<br>
`Result` is **more general** than `Option`. <br>

Path to `Result` in **std**: `std::result::Result`.<br>

```Rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` – type of **value**. `T` is wrapped in `Ok` variant.<br>
`E` – type of **error**. `E` is wrapped in `Err` variant.<br>

Value of type `T` or `E` can only be obtained via `match`:
```Rust
fn example(s: Option<i32>) -> Result<i32, &'static str> {
    match s {
        None => Err("invalid header length"),
        Some(val) => Ok(val) 
    }
}

let r1 = example(Some(1));
let r2 = example(None);
```

<br>

# Result alias
In the **std**, you may frequently see types like `Result<i32>`.<br>
Rust allows to define a `Result` **type alias** that **fixes** **one** of the *type parameters* to a **particular type**.<br>
Usually the **fixed type** is the **error type**.<br>

Standard libraries define their own `Result` **aliases**.<br>

|**Library**|**Path to** `Result`|**Definition**|
|:----------|:---------------------|:-------------|
|`std::io`|`std::io::Result`|`type Result = Result<(), std::io::Error>;`|
|`std::fmt`|`std::fmt::Result`|`type Result = Result<(), std::fmt::Error>;`|

<br>

### Example
If we have a lot of functions that could return `ParseIntError`, then it’s much more convenient to define an **alias** that always uses `ParseIntError`.<br>

```Rust
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    unimplemented!();
}
```
