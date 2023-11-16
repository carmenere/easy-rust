# Primitive Type never
The `!` type is called **never type**.<br>

<br>

```Rust
const FOO: bool = true;

fn main () {
    let bar = None;
    while FOO {
        let v = match bar {
            Some(v) => v,
            None => continue,
        };
    };
}
```

In the above example, variable `v` inside `while {}` is of **never type**, because `None => continue` arm of `match` will never return any value.

<br>

```Rust
const FOO: bool = true;

fn main () {
    let bar = Some(10);
    while FOO {
        let v = match bar {
            Some(v) => v,
            None => continue,
        };
    };
}
```

In the above example, variable `v` inside `while {}` is of `i32` type, because `Some(v) => v` arm of `match` returns value `10`.

<br>

So,
- `panic!` macro returns **never type**;
- `loop` returns **never type**, e.g., `let r: ! = loop {};`;
- keyword `continue` inside loop returns **never type**;
