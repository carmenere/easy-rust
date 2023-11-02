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

# Function pointer vs. Closures
**Closures** which **don't capture any variable** at all are **like functions** and such closures can be **coerced** into **function pointers**:
```Rust
let fptr: fn(i32, i32) -> i32 = |a, b| a + b;
```

<br>

But if the closure **captures a variable**, it **can no longer be coerced** and code below **will not compile**:
```Rust
fn main() {
    let c = 10;
    let fptr: fn(i32, i32) -> i32 = |a, b| a + b + c;
}
```

<br>

Since **functions can't capture any environment**, they are **like** an `Fn` closure that **captures nothing**.<br>
Because `Fn` is the lowest in the trait hierarchy, **all function pointers implement all the closure traits**:
```Rust
fn call_fn_once<C: FnOnce(i32, i32) -> i32>(c: C) -> i64 {
    c(1, 2).into()
}

fn call_fn_mut<C: FnMut(i32, i32) -> i32>(mut c: C) -> i64 {
    i64::from(c(3, 4))
}

fn call_fn<C: Fn(i32, i32) -> i32>(c: C) -> i64 {
    c(5, 6).into()
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = call_fn_once(add);
    let b = call_fn_mut(add);
    let c = call_fn(add);
    let d = call_fn(|a,b| a + b);

    println!("a = {a}; b = {b}; c = {c}; d = {d}.");
}
```

<br>

When should you use a **function pointer** and when a **closure**?<br>
Your **first choice should be a closure** because it gives the caller more freedom in what to pass.<br>
There are times when using a function pointer makes sense. For example, when you are calling **C code** through **FFI** it's ok to use function pointers because **C doesn't understand Rust closures**.