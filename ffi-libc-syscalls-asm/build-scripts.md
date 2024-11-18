# Example of using build scripts
The `Cargo.toml` has section `[build-dependencies]` to specify dependencies for **build script**.<br>
A **build script** is used for building native code.<br>
The **default path** to build scripts is `build.rs` in the root of the package.<br>

<br>

- **Cargo.toml**
```toml
[package]
name = "hello-world-c"
version = "0.1.0"
edition = "2021"

[build-dependencies]
cc = "1.0"

[dependencies]
```

- **build.rs**
```rust
fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
```

- **src/hello.c**
```c
#include <stdio.h>

void hello() {
    printf("Hello, World!\n");
}
```

- **src/main.rs**
```rust
extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}
```
