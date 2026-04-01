# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Closures](#closures)
  - [Syntax](#syntax)
    - [Notation](#notation)
    - [Closure with args](#closure-with-args)
    - [Closure without args](#closure-without-args)
    - [Compare to functions](#compare-to-functions)
- [`|_|` in a closure](#_-in-a-closure)
- [Types of closures](#types-of-closures)
  - [The relationship between `FnOnce`, `FnMut`, and `Fn`](#the-relationship-between-fnonce-fnmut-and-fn)
  - [Examples of how Rust infers type of closure](#examples-of-how-rust-infers-type-of-closure)
- [Closure type](#closure-type)
- [Returning closures](#returning-closures)
- [`move` keyword](#move-keyword)
    - [Example](#example)
    - [Example](#example-1)
- [How the compiler implements closures](#how-the-compiler-implements-closures)
- [Another examples](#another-examples)
    - [Function that accepts one closures](#function-that-accepts-one-closures)
    - [Function that accepts two closures](#function-that-accepts-two-closures)
<!-- TOC -->

<br>

# Closures
**Closure** aka **anonymous function** or **lambda**.<br>

We create a **closure** using the `|...| {...}` syntax, and then we create a `let` **binding** so we can use it later.<br>
Note that we call the closure using the **binding name** and **parentheses**, just like we would for a **named function**.<br>

**Closure has access to values in the scope where it defined**. In other words, **closure captures** any **values** it uses from **scope** where it is **defined**.<br>

**Example**:
```rust
fn main() {
    let x = 10;
    let capture_x = || println!("{x}");
    capture_x();
}
```
here **closure** captures variable `x`.

<br>

## Syntax
### Notation
`|| -> { ... }`:
- `||` defines **arguments**, **mandatory**;
- `->` defines **returning type**, **optional**;
- `{}` defines **body**, **optional**;

<br>

**Examples**:
```rust
let x: i32 = || -> i32 { … };
let x: ()  = || {};
let x: ()  = |a, b| { … };
let x: i32 = |a, b| a + b;
```

<br>

### Closure with args
```rust
let add_one = |x: i64| -> i64 { 1 + x };
let add_one = |x: i64|        { 1 + x };
let add_one = |x: i64|          1 + x  ;
```

### Closure without args
```rust
let x = 1;

|| -> i64 { 1 + x } 
||        { 1 + x }
||          1 + x
```

### Compare to functions
```rust
fn addone (x: i64) -> i64 { 1 + x }
```

<br>

# `|_|` in a closure
The `|_|` in a closure means that the closure needs to take an argument, but you don't want to use it.<br>

<br>

# Types of closures
A **closure** is just **sugar** for defining a *struct to contain the environment* (aka **closure's struct** or **closure object**) and *implementing one of the* `Fn*` *traits on it*.<br>

A *closure* can **capture** variables in **3 ways** (**3 capture modes**):
- **by value** (by **taking ownership**, **move**);
- **by mutable reference** (**&mut**);
- **by immutable reference** (**&**);

<br>

**Mapping** between **capture modes** and **traits**:
|**Trait**|**Declaration**|**Capture mode**|
|:--------|:--------------|:---------------|
|`FnOnce`|`pub trait FnOnce<Args>`|**Taking ownership**|
|`FnMut`|`pub trait FnMut<Args>: FnOnce<Args>`|**Mutable borrowing**|
|`Fn`|`pub trait Fn<Args>: FnMut<Args>`|**Immutable borrowing**|

<br>

> **Note**:<br>
> 1. Closure of `impl Fn()` type can be called **multiple times**, even in **parallel** on **multiple threads**.
> 2. Closure of `impl FnMut()` type can be called **multiple times** but **not** in **parallel** on **multiple threads**.
> 3. Closure of `impl FnOnce()` type can be called **only once** because after calling it the first time it **no longer owns the captured variable**.

<br>

Rust’s compiler automatically **determines** which `Fn*` **trait** to implements based on **how captured variables are used inside the closure**:
- `Fn` if *all* captured values ​​are for **read-only** access; 
- `FnMut` if *at least one* captured value is **changed**;
- `FnOnce` if *at least one* captured value is **moved out** or **dropped**;

<br>

In other words, what trait is implemented is decided by **what the closure does with the captured variable**:
1. Compiler choose `impl Fn()` for those closures which
  - **don't mutate** the captured variables **inside** closure; 
  - **don't move** the captured variables **out** of the closure;
  - **don't capture any variables** at all;
2. Compiler choose `impl FnMut()` for those closures which
   - **mutate at least 1 captured variable** but **don't move any of captured variables out** of the closure;
3. Compiler choose `impl FnOnce()` for those closures which 
   - **move at least 1 captured variable out** of the closure, for example by **dropping** them;

<br>

**Closure's struct** that **captures variables by value** looks like:
```rust
struct EnvOwn {
    variable1: Type1,
    variable2: Type2,
}
```

<br>

**Closure's struct** that **captures variables by reference** looks like:
```rust
struct EnvRef<'a> {
    variable1: &'a Type1,
    variable2: &'a mut Type2, // Note: if an `&mut` reference to `variable2` is used in the closure.
}
```

<br>

Take a look at the **signature** of the `call` method in the `Fn*` traits:
- `FnOnce` -> `FnOnce::call_once(self, ...)`;
- `Fn` -> `Fn::call(&self, ...)`;
- `FnMut` -> `FnMut::call_mut(&mut self, ...)`;

<br>

The `self` is the *closure's struct* that contains **captured variables** from closure's **environment**.<br>

<br>

The `move` keword **moves variables** from the stack where the closure is defined into the *closure's struct*.<br>
When invoked, `FnOnce` moves the **closure object** into the closure's call stack and thus consumes it. So, **closure can be used only once**.<br>
Both `Fn` and `FnMut` put a reference to **closure object** on the call stack.<br>

<br>

## The relationship between `FnOnce`, `FnMut`, and `Fn`
```rust
pub trait FnOnce
pub trait FnMut: FnOnce
pub trait Fn: FnMut
```

`FnOnce` is a **supertrait** of `FnMut` and `FnMut` is a **supertrait** of `Fn`.<br>

<br>

To sum up:
- `Fn` **must** implement `FnMut` and `FnOnce`;
- `FnMut` **must** implement **only** `FnOnce`;
- `FnOnce` **doesn’t need any** other traits to be implemented;

<br>

Note, **all closures implement** `FnOnce`.<br>

<br>

Also it means that:
- if a **function takes** an `F: FnOnce()` as an argument `f`:
  - it can accept **any closure** (`Fn`, `FnMut` or `FnOnce`) matching the signature;
  - but it **can call** `f()` **only once**;
```rust
fn foo<F>(f: F)
where
    F: FnOnce(),
{
    f();
    // f(); // ❌ ERROR: use of moved value: `f`
}
```
- if a **function takes** an `F: FnMut()` as an argument `f`:
  - it can **also** accept `Fn` closure matching the signature (because `Fn` implements `FnMut`);
  - `FnMut` requires `mut` before argument `mut f`:
```rust
fn foo<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
}
```
- if a **function takes** an `F: Fn()` as an argument `f`: it **only** accepts `Fn`:
```rust
fn foo<F>(f: F)
where
    F: Fn(),
{
    f();
    f();
}
```

<br>

Also it means that:
- closure implementing `Fn` can be used **anywhere**;
- closure implementing `FnMut` can be used where `FnOnce` or `FnMut` is expected;
- closure implementing `FnOnce` can be used where **only** `FnOnce` is expected;

<br>

## Examples of how Rust infers type of closure
**FnOnce** vs. **FnMut**:
![no_move_and_FnOnce](/img/fn_once_vs_fn_mut.png)

<br>

**Fn**:<br>
![Fn](/img/Fn.png)

<br>

**FnMut**:<br>
![FnMut](/img/FnMut.png)

<br>

**FnMut** - using `move` keyword:<br>
![move_and_FnMut](/img/move_and_FnMut.png)

<br>

**FnOnce** - *drop*:<br>
![FnOnce](/img/FnOnce.png)

<br>

**FnOnce** - *mutation* + *drop*:<br>
![FnOnce_and_mutation](/img/FnOnce_and_mutation.png)

<br>

**FnOnce** - *transferring ownership out*:<br>
![no_move_and_FnOnce](/img/no_move_and_FnOnce.png)

<br>

```rust
fn do_something<F>(f: F)
where
    F: FnOnce(),
{
    f();
}
```

<br>

# Closure type
The concrete type of a closure **can't** be written **explicitly**:
```rust
let closure /*:No way to write the type of closure here*/ = || {};
```

<br>

**Each closure** has an **unique anonymous type** assigned to it by the compiler.<br>
**No two closures**, even if **identical**, have the **same type**:
```rust
fn main() {
    let mut closure1 = || {};
    let closure2 = || {};
    closure1 = closure2; // ERROR mismatched types
}
```

<br>

But is is possible to refer to **closure type** in finction signature trough `Fn()`/`FnMut()`/`FnOnce()` traits:
```rust
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
```rust
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

# Returning closures
```rust
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
```rust
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
```rust
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
```rust
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
```rust
fn main() {
    let foo = "foo".to_string();
    let capture_foo = || {
        println!("foo = {foo}");
    };
    capture_foo();
}
```

It can be **desugared** as:
```rust
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
```rust
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
```rust
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
