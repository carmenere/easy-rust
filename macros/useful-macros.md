# Table of contents
- [Table of contents](#table-of-contents)
- [The `todo!` and `unimplemented!` macro](#the-todo-and-unimplemented-macro)
- [`column!`, `line!`, `file!` and `module_path!`](#column-line-file-and-module_path)
- [`matches!`](#matches)
- [`thread_local!`](#thread_local)
- [`cfg!`](#cfg)
- [`dbg!`](#dbg)
  - [Suppress `dbg!` in release builds](#suppress-dbg-in-release-builds)
    - [Attribute `debug_assertions`](#attribute-debug_assertions)
    - [Defining a custom macro](#defining-a-custom-macro)

<br>

# The `todo!` and `unimplemented!` macro
Sometimes, you want to write the **general structure of your code** to help you imagine your project’s final form. Writing the general structure of your code is called **prototyping**.<br>
There are 2 macro that help achieve this:
- `todo!`
  - indicates functionality you **intend to implement later**;
  - **panics** with a message like **not yet implemented**;
- `unimplemented!`
  - indicates functionality that is **not currently implemented**, **possibly permanently**;
  - **panics** with a message like **not implemented**;

<br>

# `column!`, `line!`, `file!` and `module_path!`
- `column!` gives you **column number** where this macro is called;
- `file!` gives the **filename** in which this macro is called;
- `line!` gives the **line number** in which this macro is called;
- `module_path!` gives the **path to the module** in which this macro is called;

<br>

# `matches!`
The `matches!` macro converts result of `match` to `bool`, the `matches!` lets us add an `if` clause and an `expression`:
```rust
fn main() {
  let var = 333;
  println!("1: {}", matches!(var, 9));
  println!("2: {}", matches!(var, 0..=10));
  println!("3: {}", matches!(var, 100..=1000));

  println!("4: {}", matches!(var, 10..=1000 if var % 3 == 0));
  println!("4: {}", matches!(var, 10..=1000 if var % 3 != 0));
}
```
**Output**:
```bash
1: false
2: false
3: true
4: true
4: false
```

<br>

# `thread_local!`
The `thread_local!` macro creates `static` that is **local to each thread**, so no need for a `Mutex`!.<br>
The value can then be accessed with a method called `.with()` that gives access to the value inside within a closure.<br>

**Example**:
```rust
use std::cell::RefCell;


thread_local! {
    static LOCAL_VALUE: RefCell<i32> = RefCell::new(10);
}

#[test]
fn one() {
    LOCAL_VALUE.with(|cell| {
        let mut lock = cell.borrow_mut();
        println!("Test 1. Local value is {lock:?}");
        *lock += 1;
        println!("Test 1. Local value is now {lock:?}\n");
    });
}

#[test]
fn two() {
    LOCAL_VALUE.with(|cell| {
        let mut lock = cell.borrow_mut();
        println!("Test 2. Local value is {lock:?}");
        *lock += 1;
        println!("Test 2. Local value is now {lock:?}\n");
    });
}

fn main() {

}
```

**Output**:
```bash
running 2 tests
test one ... ok
test two ... ok

successes:

---- one stdout ----
Test 1. Local value is 10
Test 1. Local value is now 11


---- two stdout ----
Test 2. Local value is 10
Test 2. Local value is now 11



successes:
    one
    two

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<br>

# `cfg!`
```rust
#[test]
fn one() {
    println!("{}", foo());
}

fn foo() -> String {
    if cfg!(test) {
        ">>>> inside test".to_owned()
    } else {
        ">>>> inside main()".to_owned()
    }
}

fn main() {
    println!("{}", foo());
}
```

**Output** **`cargo run`**:
```bash
>>>> inside main()
```

**Output** **`cargo test -- --show-output`**:
```bash
running 1 test
test one ... ok

successes:

---- one stdout ----
>>>> inside test


successes:
    one

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

<br>

# `dbg!`
The `dbg!` prints to **stderr**.<br>
The `dbg!(val)` **moves** `val` if `val` is **not** `Copy`.<br>

You can put `dbg!` in the middle of expression:
```rust
fn main() {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x *2).collect::<Vec<i32>>());
    dbg!(double_vec);
}
```
**Output**:
```python
[src/main.rs:2:25] 9 = 9
[src/main.rs:3:5] my_number += 10 = ()
[src/main.rs:4:19] vec![8, 9, 10] = [
    8,
    9,
    10,
]
[src/main.rs:5:22] new_vec.iter().map(|x| x *2).collect::<Vec<i32>>() = [
    16,
    18,
    20,
]
[src/main.rs:6:5] double_vec = [
    16,
    18,
    20,
]
```

<br>

## Suppress `dbg!` in release builds
The standard Rust `dbg!` macro is **not** automatically disabled in **release** builds.<br>

<br>

### Attribute `debug_assertions`
```rust
fn main() {
    #[cfg(debug_assertions)]
    let v =  dbg!(10);
}
```

- the `debug_assertions` flag is *automatically* **enabled** for **debug profile**;
- the `debug_assertions` flag is *automatically* **disabled** for **release profile**, i.e. when using `cargo build --release`;

<br>

### Defining a custom macro
```rust
macro_rules! dbg {
    () => {
        #[cfg(debug_assertions)]
        std::dbg!();
    };
    ($val:expr) => {
        #[cfg(debug_assertions)]
        std::dbg!($val);
        #[cfg(not(debug_assertions))]
        {
            // In release, the expression is still evaluated but nothing is printed.
            // This is useful if the expression has side effects.
            ($val)
        }
    };
    // Add more cases for multiple arguments if needed.
    ($($val:expr),+) => {
        #[cfg(debug_assertions)]
        std::dbg!($($val),+);
        #[cfg(not(debug_assertions))]
        {
            // Evaluate all expressions in release mode to avoid dead code warnings
            // if they have side effects.
            ($(($val)),*)
        }
    };
}
```

<br>