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
  - [Methods for transforming the contained value](#methods-for-transforming-the-contained-value)
    - [Transform `Result<T, E>` to `Option<E>` or `Option<T>`](#transform-resultt-e-to-optione-or-optiont)
    - [Transform `Result` to `Result`:](#transform-result-to-result)
    - [Transform an `Result<T, E>` into a value of a **possibly** different type `U`:](#transform-an-resultt-e-into-a-value-of-a-possibly-different-type-u)
- [Methods acting as `boolean` operators](#methods-acting-as-boolean-operators)

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
- [**unwrap()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap):
  - if the result is `Ok(v)` returns **inner value** `v` of type `T`;
  - if the result is `Err(e)` **panics** with a **generic message**;
- [**expect()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.expect):
  - if the result is `Ok(v)` returns **inner value** `v` of type `T`;
  - if the result is `Err(e)` **panics** with a **custom message** provided by `msg`;
- [**unwrap_or()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or):
  - if the result is `Ok(v)` returns **inner value** `v` of type `T`;
  - if the result is `Err(e)` returns the **default value** of type `T` provided by `default`;
- [**unwrap_or_else()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_else):
  - if the result is `Ok(v)` returns **inner value** `v` of type `T`;
  - if the result is `Err(e)` calls **closure** `f()` and returns **its result** of type `T`;
- [**unwrap_or_default()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap_or_default):
  - if the result is `Ok(v)` returns **inner value** `v` of type `T`;
  - if the result is `Err(e)` returns the **default value** tor type `T`. Type `T` must implement `Default` trait;

<br>

## Methods for transforming the contained value
### Transform `Result<T, E>` to `Option<E>` or `Option<T>`
- [**err()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.err):
  - transforms `Result<T, E>` into `Option<E>`
    - `Err(e)` => `Some(e)`;
    - `Ok(v)` => `None`;
- [**ok()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.ok):
  - transforms `Result<T, E>` into `Option<T>`
    - `Ok(v)` => `Some(v)`;
    - `Err(e)` => `None`;
- [**transpose()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.transpose):
  - transposes a `Result` of an `Option` into an `Option` of a `Result`: `Result<Option<T>, E>` => `Option<Result<T, E>>`:
    - `Ok(None)` => `None`;
    - `Ok(Some(v))` => `Some(Ok(v))`;
    - `Err(e)` => `Some(Err(e))`.

<br>

### Transform `Result` to `Result`:
- [**map(f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map):
  - transforms `Result<T, E>` into `Result<T2, E>`
    - if the result is `Err(e)` it leaves the value `e` of the `Err` variant unchanged;
    - if the result is `Ok(v)` it transforms `T` into `U` by applying the provided function `f` to the value `v` of the `Ok` variant.
- [**map_err(f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err):
  - transforms `Result<T, E>` into `Result<T, E2>`
    - if the result is `Ok(v)` it leaves the value `v` of the `Ok` variant unchanged;
    - if the result is `Err(e)` it transforms `E` into `U` by applying the provided function `f` to the value e of the `Err` variant.


<br>

### Transform an `Result<T, E>` into a value of a **possibly** different type `U`:
- [**map_or(default, f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or):
  - it **returns** **value** of type `U`;
    - if the result is `Ok(v)` it applies the provided function `f` to the value `v` of the `Ok` variant;
    - if the result is `Err(e)` it returns the provided **default value** by default.
- [**map_or_else(d, f)**](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_or_else):
  - it **returns** **value** of type `U`;
    - if the result is `Ok(v)` it applies the provided function `f` to the value `v` of the `Ok` variant;
    - if the result is `Err(e)` it applies the provided default fallback function `d` to the value `e` of the `Err` variant.


<br>

# Methods acting as `boolean` operators
[**Boolean operators doc**](https://doc.rust-lang.org/std/result/index.html#boolean-operators):
- [**and()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.and);
- [**or()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.or);
- [**and_then()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.and_then);
- [**or_else()**](https://doc.rust-lang.org/std/result/enum.Result.html#method.or_else);

<br>

All above methods treat the `Result` as a `boolean` value.
- The `and()` and `or()` methods take another `Result` as **input**, and produce a `Result` as **output**.
  - The `and` method can produce a `Result<U, E>` value having a different inner type `U` than `Result<T, E>`.
  - The `or` method can produce a `Result<T, E2>` value having a different error type `E2` than `Result<T, E>`.
- The `and_then()` and `or_else()` methods take a **function** `f` as **input**, and produce a **Result** as **output**.
  - The `and_then()` method can produce a `Result<U, E>` value having a different inner type `U` than `Result<T, E>`.
  - The `or_else()` method can produce a `Result<T, E2>` value having a different error type `E2` than `Result<T, E>`.

