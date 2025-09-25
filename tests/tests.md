# Table of contents
- [Table of contents](#table-of-contents)
- [Test organization](#test-organization)
  - [Fail conditions](#fail-conditions)
- [Unit Tests](#unit-tests)
- [Integration Tests](#integration-tests)
- [Test Attributes](#test-attributes)
- [Doc Tests](#doc-tests)
- [Ways to run tests](#ways-to-run-tests)
  - [Running tests in parallel or consecutively](#running-tests-in-parallel-or-consecutively)
  - [Showing function outputit](#showing-function-outputit)
  - [Run one specific test by exact name](#run-one-specific-test-by-exact-name)
  - [Run tests by pattern](#run-tests-by-pattern)
  - [Run only the ignored tests](#run-only-the-ignored-tests)
  - [Run all tests whether they’re ignored or not](#run-all-tests-whether-theyre-ignored-or-not)
- [Example project](#example-project)

<br>

# Test organization
A test in Rust is a **function** that’s **annotated** with the `#[test]` **attribute**.<br>

There 3 types of tests:
1. **Unit tests**.
2. **Integration tests**.
3. **Doc tests**.

<br>

|Command| Description |
|:------|:------------|
|`cargo test --bins --lib`| Runs **only** _unit tests_ from binary crates and library crate|
|`cargo test --lib`| Runs **only** _unit tests_ from library crate|
|`cargo test --lib <test_name>`| Runs **only** _unit tests_ from library crate with **common prefix** in their names `<test_name>`|
|`cargo test --lib tests::<test_name> -- --exact`| Runs **only one** _unit test_ from library crate with exact name `<test_name>`|
|`cargo test --test '*'`| Runs **only** _integration tests_|
|`cargo test <integration_test_file_name>`| Runs **only** _integration tests_ from file `tests/<integration_test_file_name>.rs`|
|`cargo test <integration_test_file_name> -- <test_function_name>`| Runs **only** _integration tests_ from file `tests/<integration_test_file_name>.rs` with **common prefix** in their names `<test_function_name>`|
|`cargo test <integration_test_file_name> -- <test_function_name> --exact`| Runs **only one** _integration test_ from file `tests/<integration_test_file_name>.rs` with exact name `<test_function_name>`|
|`cargo test --tests`| Runs _unit tests_ + _integration tests_|
|`cargo test --doc`| Runs **only** _doc tests_|
|`cargo test`| Runs **all**: _unit tests_ + _integration tests_ + _doc tests_|

<br>

## Fail conditions
The test is considered **failed** if:
- code inside test **panics**;
- test function returns `Result<(), String>` type and **test returns** `Err(String::from("Some error message here."))`;

<br>

To run tests there is `cargo test` command.<br>
Organization of **output** of `cargo test`:
- all **unit tests** are grouped in **one separate section**;
- all **integration tests** are grouped **per .rs file** `src/tests` directory in **separate section**;
- all **doc tests** are grouped in **one separate section**;

<br>

**Output** of `cargo test`:
```bash
     Running unittests src/lib.rs

running 1 test
test tests::internal ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/integration_test.rs

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests adder

running 0 tests

test result: ok. 0 passed; 
```

<br>

# Unit Tests
The purpose of **unit tests** is to test each unit of code in isolation from the rest of the code.<br>
The convention is to create a **module** named `tests` in **each .rs file** and to annotate the module with `cfg(test)`:
```Rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

Here `use super::*` brings all of the test module’s parent’s items into scope.<br>

<br>

# Integration Tests
In Rust, **integration tests** are **separated** from your **library code** and live in a `tests` directory in your **project root** (aka **package root)**.<br>
They can only call functions that are part of your library’s **public API**.<br>
Their purpose is to test whether many parts of your library **work together correctly**.<br>

Cargo knows to look for `tests/*.rs` files in **package root** directory.<br>
**Each file** in the `tests` directory is a **separate crate**, so we need to bring our library into each test crate’s scope.<br>

We can still run a **particular integration test function** by specifying the **test function’s name** as an argument to `cargo test`.<br>

To run all the tests in a **particular integration test file**, use the `--test` argument of `cargo test` followed by the name of the file:
```bash
cargo test --test person
```
here `person` is a `tests/person.rs` file.

<br>

# Test Attributes
- `#[test]`
  - Indicates a function is a test to be run. This function takes no arguments.
- `#[bench]`
  - Indicates a function is a benchmark to be run. This function takes one argument `test::Bencher`.
- `#[should_panic]`
  - Indicates that function will only pass if the code causes a **panic**. To **filter** panic by its message there is `expected` parameter.
```Rust
#[test]
#[should_panic(expected = "less than or equal to")]
fn greater_than_100() {
    Guess::new(200);
}
```
- `#[ignore]`
  - **Excludes** function from **normal** test runs.

<br>

# Doc Tests
**Doc tests** are injected into documentation inside **code block** ``````:
```Rust
/// Function `add` take two u64 integers and add them and return result of u64.
/// 
/// # Examples
/// ```
/// use my::add;
/// assert_eq!(3, add(1,2))
/// ```
pub fn add(x:u64, y:u64) -> u64 {
    x + y
}
```

<br>

# Ways to run tests
## Running tests in parallel or consecutively
By default tests are run in parallel using threads. Because the tests are running at the same time, you must make sure your tests don’t depend on each other or on any shared state.<br>

```bash
cargo test -- --test-threads=1
```

<br>

## Showing function outputit
If we call `println!` in a test and the test passes, we **won’t** see the `println!` **output** in the terminal.<br>
Anything printed to **standard output** is captured for **passed** test.<br>
If a test **fails**, we’ll see whatever was printed to **standard output** with the rest of the **failure message**.<br>
There is option `--show-output` to see printed values for **passing** tests as well:
```bash
cargo test -- --show-output
```

<br>

## Run one specific test by exact name
```bash
cargo test add_3_0
```

<br>

## Run tests by pattern
```bash
cargo test add
```

<br>

## Run only the ignored tests
```bash
cargo test -- --ignored
```

<br>

## Run all tests whether they’re ignored or not
```bash
cargo test -- --include-ignored
```

<br>

# Example project
1. `Cargo.toml`
```Rust
[package]
name = "my"
version = "0.1.0"
edition = "2021"
[lib]
name = "my"

[[bin]]
name = "my"
```
2. `main.rs`
```Rust
use my;

fn main() {
    my::add(1, 2);
}
```
3. `lib.rs`
```Rust
pub mod example;

/// Function `add` take two u64 integers and add them and return result of u64.
/// 
/// # Examples
/// ```
/// use my::add;
/// assert_eq!(3, add(1,2))
/// ```
pub fn add(x:u64, y:u64) -> u64 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1_2() {
        assert_eq!(3, add(1,2))
    }

    #[test]
    fn add_3_0() {
        assert_eq!(3, add(3,0))
    }
}
```
4. `example.rs`
```Rust
/// Function `add` take two u64 integers and add them and return result of u64.
/// 
/// # Examples
/// ```
/// use my::{self, example::multiply};
/// assert_eq!(2, multiply(1,2))
/// ```
pub fn multiply(x:u64, y:u64) -> u64 {
    x * y
}
```
5. `tests/test_add.rs`
```Rust
use my::{self, example::multiply};

#[test]
fn add_and_multiply() {
    assert_eq!(5, my::add(1, 2) + multiply(1, 2));
}
```

