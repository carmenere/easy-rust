# Table of contents
- [Table of contents](#table-of-contents)
- [Just add `#[test]`](#just-add-test)
- [Grouping tests in modules](#grouping-tests-in-modules)
- [Test-driven development](#test-driven-development)

<br>

# Just add `#[test]`
The easiest way to start testing is to add `#[test]` above a function:
```rust
#[test]
fn two_is_two() {
    assert_eq!(2, 2);
}
```

<br>

The command `cargo test` runs **all tests** and it **doesn't need** `main()` function for tests. You can **delete** the `main()` function and `cargo test` **still works**.<br>

The names for test function are usually **quite descriptive**, like `one_minus_two_is_minus_one`. That is because as your code grows, the **number of tests grows too**, and **descriptive test names** let you understand which tests have failed.<br>

> *Note*: **test functions can’t take any arguments**.<br>

So, how does the compiler know that the test passed? It’s pretty simple:
- if a test function **doesn't** *panic*, then it is a **pass**;
- if a test function **panics**, then it’s a **failure**;

<br>

The `assert_eq!(left, right)` and `assert!(bool)` are probably **the most common ways** *to test a function* in Rust.<br>

For `assert_eq!`, if the `left` and `right` sides **don’t match**, it will **panic** and show that the **values are different**:
```rust
assertion `left == right` failed
left: 2
right: 3
```

<br>

The output for the `assert!` macro is almost the same:
```rust
assertion failed: 2 == 3
```

<br>

When a test **fails**, you get a lot information:
```rust
cargo test --bin example
    Finished `test` profile [optimized + debuginfo] target(s) in 0.01s
     Running unittests src/main.rs (target/debug/deps/example-59186309a0341b26)

running 2 tests
test one_is_one ... ok
test two_is_two ... FAILED

failures:

---- two_is_two stdout ----

thread 'two_is_two' panicked at example/src/main.rs:3:5:
assertion `left == right` failed
  left: 2
 right: 3
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    two_is_two

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `--bin example`
```

<br>

The `RUST_BACKTRACE` **environment variable** controls verbosity of **stack backtrace**
- `RUST_BACKTRACE=1`
- `RUST_BACKTRACE=full`

<br>

Work with **envs**:
- the function `std::env::var("FOO")` returns value of **environment variable** `FOO`;
- the function `std::env::set_var("FOO", "1");` assign value `1` to **environment variable** `FOO`;

<br>

**By default**, the env `RUST_BACKTRACE` is not set:
```rust
fn main() {
    println!("{:?}", std::env::var("RUST_BACKTRACE"));
}
```

**Output**:
```rust
Err(NotPresent)
```

<br>

You don’t need to use a backtrace unless you really can’t find where the problem is.<br>

<br>

# Grouping tests in modules
Use the `mod` keyword to create a new **test module** `tests` and add `#[cfg(test)]` above it. Also, **don’t forget** to write `use super::*;` because the **test module** needs access to the functions above it:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_is_two() {
        assert_eq!(2, 3);
    }

    #[test]
    fn one_is_one() {
        assert_eq!(1, 1);
    }
}
```

<br>

# Test-driven development
**TDD** means writing tests first, **all of which will fail**! Only then you start writing the code. Then you start writing the code and keep doing that until all the tests pass.<br>
