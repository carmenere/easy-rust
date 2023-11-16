# Constants
## Notes
- **Uppercase** by convention.
- Data type is **mandatory**.
- Values can not be changed.
- **Global** or **Local** scope.

<br>

## Examples
```Rust
const URL: &str = "google.com";
```

<br>

# Static
**Static variables** are **global variables**.<br>
*Static variable* **must** have **static lifetime**.<br>
*Static variables* can be **mutable** or **immutable**.<br>
**Mutable** *static variables* can only be **read** and **modified** inside `unsafe` **block**.<br>
*Static variables* have **fixed address** in the memory.

<br>

## Examples
```Rust
static mut COUNTER: u64 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

fn main () {
    // access to modify static variable
    unsafe {
        increment();
    }

    // access to read static variable
    unsafe {
        println!("Counter is {}.", COUNTER);
    }
}
```

