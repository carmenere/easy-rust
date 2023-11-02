# Scoped threads
**Normal threads** **cannot** borrow from their environment:
```Rust
use std::thread;

fn foo() {
    let s = String::from("Hello");
    thread::spawn(|| {
        println!("Length: {}", s.len());
    });
}

fn main() {
    foo();
}
```

<br>

However, there are a **scoped threads** `thread::scope(|scope| { ... })` for this:
```Rust
use std::thread;

fn main() {
    let s = String::from("Hello");

    thread::scope(|scope| {
        scope.spawn(|| {
            println!("Length: {}", s.len());
        });
    });
}
```
