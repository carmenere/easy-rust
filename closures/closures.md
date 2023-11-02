# Closures
**Closure** aka **anonymous function** or **lambda**.<br>

We create a **closure** using the `|...| {...}` syntax, and then we create a `let` **binding** so we can use it later.<br>
Note that we call the closure using the **binding name** and **parentheses**, just like we would for a **named function**.<br>

**Closure has access to values in the scope where it defined**. In other words, **closure captures** any **values** it uses from **scope** where it is **defined**.<br>

**Syntax** for **closure** (braces are optional) and **named function**:
<table>
<tr>
<td>

**Closure**

</td>


<td>

```Rust
let add_one = |x: i64| -> i64 { 1 + x };
let add_one = |x: i64|        { 1 + x };
let add_one = |x: i64|          1 + x  ;
```

</td>
</tr>

<tr></tr>
<tr>
<td>

**Closure without args**

</td>

<td>

```Rust
let x = 1;

|| -> i64 { 1 + x } 
||        { 1 + x }
||          1 + x
```

</td>
</tr>

<tr></tr>
<tr>
<td>

**Function**

</td>

<td>

```Rust
fn addone (x: i64) -> i64 { 1 + x }
```

</td>
</tr>

</table>

<br>

#### Example
```Rust
fn main() {
    let x = 10;
    let capture_x = || println!("{x}");
    capture_x();
}
```

here **closure** captures variable `x`.

<br>

# Closure traits and closure capture modes
There are **3 capture modes**:
- by **taking ownership**;
- by **mutable borrowing**;
- by **immutable borrowing**;

<br>

**Mapping** between **capture modes** and **traits**:
|**Trait**|**Declaration**|**Capture mode**|
|:--------|:--------------|:---------------|
|`FnOnce`|`pub trait FnOnce<Args>`|**Taking ownership**|
|`FnMut`|`pub trait FnMut<Args>: FnOnce<Args>`|**Mutable borrowing**|
|`Fn`|`pub trait Fn<Args>: FnMut<Args>`|**Immutable borrowing**|

<br>

> **Note**:<br>
> `FnOnce` is **supertrait** for `FnMut`.<br>
> `FnMut` is **supertrait** for `Fn`.

<br>

What trait is implemented is decided by **what the closure does with the captured variable**:
1. Compiler choose `impl Fn()` for those closures which
  - **don't mutate** the captured variables **inside** closure; 
  - **don't move** the captured variables **out** of the closure;
  - **don't capture any variables** at all;
2. Compiler choose `impl FnMut()` for those closures which
   - **mutate at least 1 captured variable** but **don't move any of captured variables out** of the closure;
3. Compiler choose `impl FnOnce()` for those closures which 
   - **move at least 1 captured variable out** of the closure, for example by **dropping** them;

<br>

> **Note**:<br>
> 1. Closure of `impl Fn()` type can be called **multiple times**, even in **parallel** on **multiple threads**.
> 2. Closure of `impl FnMut()` type can be called **multiple times** but **not** in **parallel** on **multiple threads**.
> 3. Closure of `impl FnOnce()` type can be called **only once** because after calling it the first time it **no longer owns the captured variable**.

<br>

# Closure type
The concrete type of a closure **can't** be written **explicitly**:
```Rust
let closure /*:No way to write the type of closure here*/ = || {};
```

<br>

**Each closure** has an **unique anonymous type** assigned to it by the compiler.<br>
**No two closures**, even if **identical**, have the **same type**:
```Rust
fn main() {
    let mut closure1 = || {};
    let closure2 = || {};
    closure1 = closure2; // ERROR mismatched types
}
```

<br>

But is is possible to refer to **closure type** in finction signature trough `Fn()`/`FnMut()`/`FnOnce()` traits:
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

Also it is possible to use reference to closures:
```Rust
fn main() {
    let foo = "foo".to_string();
    let capture_foo: &dyn Fn() = &|| {
        println!("foo = {foo}");
    };
    println!("foo = {foo}");
    capture_foo();
}
```

<br>


## impl Fn()
Closure `capture_foo` is of `impl Fn()` type:
```Rust
fn main() {
    let mut foo = "foo".to_string();
    let mut capture_foo = || {
        println!("foo = {foo}");
    };
    println!("foo = {foo}");
    capture_foo();
}
```

<br>

## impl FnMut()
Closure `capture_foo` is of `impl FnMut()` type:
```Rust
fn main() {
    let mut foo = "foo".to_string();
    let mut capture_foo = || {
        foo.push_str("bar");
        println!("foo = {foo}");
    };
    // println!("foo = {foo}"); // ERROR: cannot borrow `foo` as immutable because it is also borrowed as mutable
    capture_foo();
}
```

<br>

## impl FnOnce()
Closure `capture_foo` is of `impl FnOnce()` type:
```Rust
fn main() {
    let foo = "foo".to_string();
    let capture_foo = || {
        println!("foo = {foo}");
        drop(foo);
    };
    capture_foo();
    // println!("foo = {foo}"); // ERROR: borrow of moved value: `foo`
}
```

<br>

# Returning closures
```Rust
fn return_closure(a: i32) -> impl Fn(u32) -> u32 {
    if a > 0 {
        |i| i+i
    }
    else {
        |i| i*i
    }
}

fn return_boxed_closure(a: i32) -> Box<dyn Fn(u32) -> u32> {
    if a > 0 {
        Box::new(|i| i+i)
    }
    else {
        Box::new(|i| i*i)
    }
}

fn main () {
    let a = return_closure(-1);
    let b = return_closure(1);
    dbg!(a(4));
    dbg!(b(4));
    
    let a = return_boxed_closure(-1);
    let b = return_boxed_closure(1);
    dbg!(a(4));
    dbg!(b(4));
}
```

<br>

# `move` keyword
Example:
```Rust
fn main() {
    let foo = "foo".to_string();
    let capture_foo = move || {
        println!("foo = {foo}");
    };
    // println!("foo = {foo}"); // ERROR: borrow of moved value: `foo`
    capture_foo();
}
```

<br>

The `move` keyword **doesn't mean** that the closure will implement the `FnOnce` trait.<br>
**Which trait is implemented** is **decided** by **what the closure does with** the *captured variable*, **not how it is captured**.<br>
`move` **only forces** the *captured variable* **to be moved** into the closure **while** `FnOnce` is implemented if the **closure moves** the *captured variable* **out**.<br>
`move` keyword made no difference to which trait was implemented.

<br>

### Example
```Rust
fn main() {        
    let mut v: Vec<u64> = vec![1; 3];
    let mut print_v = || { v.push(1); println!("v = {:?}", v); };

    print_v();
    print_v();

    v.push(1);

    println!("v = {:?}", v);
}

Output:
v = [1, 1, 1, 1]
v = [1, 1, 1, 1, 1]
v = [1, 1, 1, 1, 1, 1]
```

<br>

### Example
```Rust
fn main() {        
    let mut v: Vec<u64> = vec![1; 3];
    let mut print_v = move || { v.push(1); println!("v = {:?}", v); };

    print_v();
    print_v();

    v.push(1);

    println!("v = {:?}", v);
}
```

**Output**:
```bash
Error:
error[E0382]: borrow of moved value: `v`
 --> src/main.rs:8:5
  |
2 |     let mut v: Vec<u64> = vec![1; 3];
  |         ----- move occurs because `v` has type `Vec<u64>`, which does not implement the `Copy` trait
3 |     let mut print_v = move || { v.push(1); println!("v = {:?}", v); };
  |                       -------   - variable moved due to use in closure
  |                       |
  |                       value moved into closure here
...
8 |     v.push(1);
  |     ^ value borrowed here after move
```

<br>

<br>

# How the compiler implements closures
Where does the compiler store captured variables? The compiler uses a **struct** to store the captured variables.<br>
Consider example:
```Rust
fn main() {
    let foo = "foo".to_string();
    let capture_foo = || {
        println!("foo = {foo}");
    };
    capture_foo();
}
```

It can be **desugared** as:
```Rust
struct Closure<'a> {
    foo: &'a String
}

impl<'a> Fn<()> for Closure<'a> {
    type Output = ();
    fn call(&self) {
        println!("foo = {self.foo}");
    }
}

let foo = "foo".to_string();
let capture_foo = Closure { foo: &foo };
capture_foo.call();
```

<br>

# Another examples
### Function that accepts one closures
```Rust
fn twice<F: Fn(i32) -> i32>(x: i32, f: F) -> i32 {
    f(x) + f(x)
}

fn square(x: i32) -> i32 { x * x }

fn main() {
    twice(5, square); // evaluates to 50
}
```

<br>

### Function that accepts two closures
```Rust
fn compose<F, G>(x: i32, f: F, g: G) -> i32
    where F: Fn(i32) -> i32, 
          G: Fn(i32) -> i32 
{
    g(f(x))
}

fn main() {
    compose(5,
            |n: i32| { n + 42 },
            |n: i32| { n * 2 }); // evaluates to 94
}
```
