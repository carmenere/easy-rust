# Function pointer
**Function pointers** are used to pass function into another function.<br>

```Rust
fn foo(x:i32, y: i32, f: fn(i32) -> u32) -> u32 {
    f(x) + f(y)
}

fn abs(v: i32) -> u32 {
   v.abs() as u32
}

fn main () {
    let x = -10;
    let y = 30;
    dbg!(foo(x, y, abs));
}
```

<br>

**Closure** is **more general type**: closure type notation `Fn(i32) -> i32` means it can be either **closure** or **function**.