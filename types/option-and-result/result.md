# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Result alias](#result-alias)
  - [Example](#example)
- [Combinators](#combinators)
  - [Predicates for checking the contained value](#predicates-for-checking-the-contained-value)
  - [Methods for working with references](#methods-for-working-with-references)
  - [Methods for extracting the contained value](#methods-for-extracting-the-contained-value)
    - [Diagram](#diagram)
    - [Examples](#examples)
      - [`unwrap()`](#unwrap)
      - [`expect()`](#expect)
      - [`unwrap_err()`](#unwrap_err)
      - [`expect_err()`](#expect_err)
  - [Methods for transforming the contained value](#methods-for-transforming-the-contained-value)
    - [Diagram](#diagram-1)
  - [Methods acting as `boolean` operators](#methods-acting-as-boolean-operators)
    - [Diagram](#diagram-2)

<br>

# URLs
|URL|
|:------------|
[**std::result::Result**](https://doc.rust-lang.org/std/result/enum.Result.html)|
|[**Combinators**](https://doc.rust-lang.org/std/result/#method-overview)|

<br>

# Declaration
```Rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` – type of **value**. `T` is wrapped in `Ok` variant.<br>
`E` – type of **error**. `E` is wrapped in `Err` variant.<br>

<br>

# In a nutshell
`Result` express **ability** of **error** through `enum`. `Result` is **more general** than `Option`. <br>

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

## Example
If we have a lot of functions that could return `ParseIntError`, then it’s much more convenient to define an **alias** that always uses `ParseIntError`.<br>

```Rust
use std::num::ParseIntError;
use std::result;

type Result<T> = result::Result<T, ParseIntError>;

fn double_number(number_str: &str) -> Result<i32> {
    unimplemented!();
}
```

<br>

# Combinators
## Predicates for checking the contained value
- [**is_ok()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_ok):
  - if the `self` is `Err` it returns `false`;
  - if the `self` is `Ok` it returns `true`;
- [**is_err()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.is_err):
  - if the `self` is `Err` it returns `true`;
  - if the `self` is `Ok` it returns `false`;

<br>

## Methods for working with references
- [**as_ref()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_ref):
  - converts from `&Result<T, E>` to `Result<&T, &E>`;
- [**as_mut()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.as_mut):
  - converts from `&mut Result<T, E>` to `Result<&mut T, &mut E>`;

<br>

## Methods for extracting the contained value
### Diagram
![Result-unwrap](/img/Result-unwrap.png)

<br>

**Methods**:
- [**unwrap_err()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_err);
- [**expect_err()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect_err);
- [**unwrap()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap);
- [**expect()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect);
- [**unwrap_or()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or);
- [**unwrap_or_else()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else);
- [**unwrap_or_default()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default);

<br>

### Examples
#### `unwrap()`
```rust
Err::<u32, u32>(1).unwrap();
```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
called `Result::unwrap()` on an `Err` value: 1
```

<br>

#### `expect()`
```rust
Err::<u32, u32>(1).expect("some_message");
```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
some_message: 1
```

<br>

#### `unwrap_err()`
```rust
Ok::<u32, u32>(1).unwrap_err();

```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
called `Result::unwrap_err()` on an `Ok` value: 1
```

<br>

#### `expect_err()`
```rust
Ok::<u32, u32>(1).expect_err("some_message");
```

**Output**:
```bash
thread 'main' (354353) panicked at src/main.rs:216:20:
some_message: 1
```

<br>

## Methods for transforming the contained value
### Diagram
![Result-or-and-map](/img/Result-or-and-2.png)

<br>

![Result-or-and-map](/img/Result-or-and-map.png)

<br>

**Methods**:
- **transforms** `Result<T, E>` into `Option<T>`:
  - [**ok()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok);
  - [**err()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.err);
- **transforms** `Result<T, E>` into `Result<T, F>`:
  - [**or()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.or);
  - [**or_else()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.or_else);
  - [**map_err(f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err);
- **transforms** `Result<T, E>` into `Result<U, E>`:
  - [**and()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.and);
  - [**and_then()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then);
  - [**map(f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map);
- it **returns value** of type `U`:
  - [**map_or(default, f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or);
  - [**map_or_else(d, f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or_else);

<br>

- [**transpose()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.transpose):
  - **transforms** a `Result` of an `Option` into an `Option` of a `Result`: `Result<Option<T>, E>` => `Option<Result<T, E>>`:
    - `Ok(None)` => `None`;
    - `Ok(Some(v))` => `Some(Ok(v))`;
    - `Err(e)` => `Some(Err(e))`;
- [**flatten()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.flatten):
  - converts from `Result<Result<T, E>, E>` to `Result<T, E>`;

<br>

## Methods acting as `boolean` operators
[**Boolean operators doc**](https://doc.rust-lang.org/std/result/index.html#boolean-operators).<br>

<br>

Methods `and()`/`and_then()`/`or()`/`or_else()` treat the `Result` as a `boolean` value: `Ok` is like **1** and `Err` is like **0**:
- `and()` and `or()` methods take another `Result` as **input**, and produce a `Result` as **output**;
- `and_then()` and `or_else()` methods take a **function** `f` as **input**, and produce a `Result` as **output**;
- `and()` and `and_then()` methods produce a `Result` with **different inner type** `U`: `Result<U,E>`;
- `or()` and `or_else()` methods produce a `Result` with **different inner type** `F`: `Result<T,F>`;

<br>

### Diagram
![Result-or-vs-and](/img/Result-or-vs-and.png)

<br>

**Example**:
```rust
let ok_1_u32_str: Result<u32, &str> = Ok(1);
let ok_1_u32_u8: Result<u32, u8> = Ok(1);
let ok_2_u32_u8: Result<u32, u8> = Ok(2);
let ok_a_str_str: Result<&str, &str> = Ok("A");

let err_e1_u32_str: Result<u32, &str> = Err("e1");
let err_e1_str_str: Result<&str, &str> = Err("e1");
let err_e2_str_str: Result<&str, &str> = Err("e2");
let err_2_u32_u8: Result<u32, u8> = Err(2);

// or
assert_eq!(ok_1_u32_str.or(ok_2_u32_u8), ok_1_u32_u8);
assert_eq!(ok_1_u32_str.or(err_2_u32_u8), ok_1_u32_u8);

assert_eq!(err_e1_u32_str.or(ok_2_u32_u8), ok_2_u32_u8);
assert_eq!(err_e1_u32_str.or(err_2_u32_u8), err_2_u32_u8);

// and
assert_eq!(ok_1_u32_str.and(ok_a_str_str), ok_a_str_str);
assert_eq!(ok_1_u32_str.and(err_e2_str_str), err_e2_str_str);

assert_eq!(err_e1_u32_str.and(ok_a_str_str), err_e1_str_str);
assert_eq!(err_e1_u32_str.and(err_e2_str_str), err_e1_str_str);
```

<br>
