# Table of contents
- [Table of contents](#table-of-contents)
- [`stdin`](#stdin)
- [`args`](#args)
- [`envs`](#envs)

<br>

# `stdin`
The function `std::io::stdin()` returns `std::io::Stdin` struct, which handles **user input** and has a various methods:
- `.read_line()` reads the input to a `&mut String`;

<br>

**Example**:
```rust
use std::io;

fn main() {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();
    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        ;
    }
    println!(r#"You wrote "{}", exit"#, input_string.trim());
}
```

**Output**:
```bash
Please type something, or x to escape:
aa
bb
x
You wrote "x", exit
```

# `args`
The function `std::env::args()` returns `std::env::Args` struct. This `Args` struct holds what the user types when starting the program, known as **command-line arguments**.<br>

**Example**:
```rust
use std::env::args;

fn main() {
    let input = args();
    for (i, entry) in input.enumerate() {
        println!("arg #{}: {}", i, entry);
    }
}
```

**Output**:
```bash
cargo run a b c
arg #0: target/debug/example
arg #1: a
arg #2: b
arg #3: c
```

The **first argument** (**zero index**) is always the **path** to the binary file.<br>
The main crate used by Rust users to work with command-line arguments is known as `clap` (**CLAP** = Command Line Argument Parser).<br>

<br>

# `envs`
The function `std::env::vars()` returns `std::env::Vars` struct. This `Vars` struct holds all environment variables.<br>

Example - list all environment variables:
```rust
fn main() {
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }
}
```

<br>

**Functions**:
- `std::env::set_var(env, value)` sets value `value` for environment variables `env`;
- `std::env::var(env)` reads value of environment variables `env`;

<br>

Most crates in Rust use the `RUST_LOG` environment variable to set **severity** for logging:

```rust
fn main() {
    match std::env::var("RUST_LOG") {
        Ok(severity) => println!("log severity level is {severity}"),
        Err(_) => {
            unsafe {std::env::set_var("RUST_LOG", "DEBUG");}
            println!("default log severity level is {}", std::env::var("RUST_LOG").unwrap());
        },
    }
}
```

<br>

