# Table of contents

- [Table of contents](#table-of-contents)
- [Lifetimes](#lifetimes)
  - [Lifetimes in functions](#lifetimes-in-functions)
  - [Lifetimes rules](#lifetimes-rules)
  - [Lifetimes in structs](#lifetimes-in-structs)
  - [Lifetimes in impl blocks](#lifetimes-in-impl-blocks)
- [Interpretation of lifetimes](#interpretation-of-lifetimes)
  - [Lifetime subtyping](#lifetime-subtyping)
  - [Example](#example)
- [Lifetimes bounds](#lifetimes-bounds)
- ['static lifetime](#static-lifetime)
  - [Rules for 'static lifetime bound](#rules-for-static-lifetime-bound)
  - [Usage](#usage)
    - [`&str`](#str)
    - [`std::thread::spawn`](#stdthreadspawn)
    - [Return reference to value from function](#return-reference-to-value-from-function)
- [Anonymous lifetimes](#anonymous-lifetimes)
- [Higher-Rank Trait Bounds (HRTBs)](#higher-rank-trait-bounds-hrtbs)

<br>

# Lifetimes
Every **reference** must be **valid** until the **lender** is **destroyed**.<br>
A **lifetime** is the **scope** within which a **reference** must be **valid**.<br>
*Lifetimes* are **denoted** with an **apostrophe**: `'a`, `'b`.<br>

Technically, **every reference has** some **lifetime** associated with it, but the compiler lets you **elide** them in common cases.<br>
It is called **lifetime elision** or **implicit lifetime annotation**. It is because the Rust compiler is smart enough to infer lifetimes in many cases.<br>
But sometimes it is needed to specify lifetimes **explicitly**. That’s because of how the lifetime elision works.<br>
When a function accepts **multiple references**, they’re **each** given **their own lifetime**.<br>

From Rust point of view, signature:`fn f (s1: &str, s2: &str) → &str` is **equal** to signature:`fn f<'a, 'b> (s1: &'a str, s2: &'b str) → &'??? str`<br>
So, `rustc` sets to `s1` and `s2` **different** lifetimes and `rustc` **doesn't** know what lifetime to assign to **returning value**.<br>
That is why compiler return error. So we must **explicitly** set lifetimes for input and output parameters.<br>
Example: `fn f<'a> (s1: &'a str, s2: &'a str) → &'a str`.<br>

The **lifetimes** help Rust find **dangling pointers**.<br>

## Lifetimes in functions
For function there are 2 kind of **lifetime parameters**:
- **Input lifetime parameter** is a lifetime associated with a **parameter** of a function. 
- **Output lifetime parameter** is a lifetime associated with the **return value** of a function.

<br>

## Lifetimes rules
**Lifetimes rules**:
1. Each function’s parameter that is **reference** gets its own **lifetime parameter** (aka **elided lifetime**).
2. If there is exactly **one input** *lifetime parameter*, it is assigned to **all output** *lifetime parameters*.
3. If there are **multiple input** *lifetime parameters*, but one of them is `&self` or `&mut self`, the **lifetime** of `self` is assigned to **all output** *lifetime parameters*.

<br>

## Lifetimes in structs
```Rust
struct Foo<'a> {
    x: &'a i64,
}

fn main() {
    let nref = &1;
    let f = Foo { x: nref };
    println!("{}", f.x);
}
```

<br>

## Lifetimes in impl blocks
```Rust
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let nref = &1;
    let f = Foo { x: nref };
    println!("x is: {}", f.x());
}
```

<br>

# Interpretation of lifetimes
Consider function:
```Rust
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    }
    else {
        s2
    }
}
```

<br>

When **concrete references** is passed to `longest` the **generic** lifetime `'a` will get the **concrete** lifetime that is equal to the **smaller** of the lifetimes of `s1` and `s2`.
In general `s1` and `s2` both can have **different** lifetimes and compiler choose the smallest one.<br>

This signature tells that for some lifetime `'a`, the function takes two parameters, both of which **live at least as long as lifetime** `'a`.<br>
This signature tells also tells Rust that the value returned from the function will **live at least as long as lifetime** `'a`.<br>

It means that **lenders** of `s1` and `s2` **can't** be **destroyed** until function execution ends, i.e., **lenders** of `s1` and `s2` must live at least as long as function call.<br>
It means that **lenders** of `s1` and `s2` **can't** be **destroyed** while **returned value** of function is in use, in other words, **lenders** of `s1` and `s2` must live at least as long as **return value** of function.

<br>

Consider code:
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let s1 = String::from("ABC");
    let result: &str;
    {
        let s2 = String::from("XYZ");
        result = longest_string(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}", result);

}


fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    }
    else {
        s2
    }
}
```

```bash
error[E0597]: `s2` does not live long enough
 --> src/main.rs:6:46
  |
5 |         let s2 = String::from("XYZ");
  |             -- binding `s2` declared here
6 |         result = longest_string(s1.as_str(), s2.as_str());
  |                                              ^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `s2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
```

<br>

We get error because compiler assigns the **lifetime** of the `result` variable to the **smallest lifetime** of passed arguments.<br>
Then compiler assigns the **lifetime** of the **returned value** to the **smallest lifetime** of passed parameters.<br>
It means that the **lifetime** of varibale `result` can last until the end of scope where argument with **smallest lifetime** goes **out of scope**.

<br>

## Lifetime subtyping
Consider following code:
```Rust
fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        x
    } else {
        y
    }
}
```

```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0623]: lifetime mismatch
  --> src/main.rs:10:9
   |
6  | fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
   |                               -------     -------
   |                               |
   |                               this parameter and the return type are declared with different lifetimes...
...
10 |         y
   |         ^ ...but data from `y` is returned here

For more information about this error, try `rustc --explain E0623`.
error: could not compile `playrs` due to previous error
```

This program **doesn't compile**, because the lifetimes `'a` and `'b` are **independent**.<br>

Rust allows you to declare that lifetime `a` contains another lifetime. It is called **lifetime subtyping**.<br>
Notations of **lifetime subtyping**:<br>
- `fn max<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32`;
- `fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 where 'b: 'a`;

<br>

Notation `'left: 'right` means `'left` **outlives** `'right`, and `'left` is a **subtype** of `'right`, i.e., `'right` <= `'left`.

<br>

## Example
```rust
#[derive(Debug)]
struct Movie<'a> {
    title: &'a str,
    rating: u8,
}

#[derive(Debug)]
struct Reviewer<'a, 'b: 'a> {
    movie: &'a Movie<'b>,
    name: &'a str,
}

impl<'a, 'b> Reviewer<'a, 'b> {
    fn new(name: &'a str, movie: &'b Movie) -> Self {
        Reviewer { movie: movie, name: name }
    }
}

fn main() {
    let movie = Movie {
        title: "Foo",
        rating: 10,
    };

    println!("{:?}", Reviewer::new("Bar", &movie));
}
```

Here `'b` specifies that lifetimes of the `Movie` struct **must live as long** or **longer** than the `Reviewer` struct.

<br>

# Lifetimes bounds
*Lifetimes* can be used as **trait bounds** and called **lifetimes bounds**.<br>
Consider following type:<br>
```Rust
struct Ref<'a, T: 'a>(&'a T);
```

Note that the type `T` might itself contains references with some **inner** lifetime `'b`.<br>
If `T`'s **inner** lifetime `'b` **smaller** than the **exterior** lifetime `'a` then `Ref` holds a reference to some value that owns **dangling pointers**.<br>

To prevent this, we need `'b` to be **larger** than `'a`, i.e. `'b: 'a`, but instead we can write `T: 'a`.<br>

`T: 'a` means any references in the type `T` must have a lifetime `'b` that **outlives** `'a`, and that makes `Ref` safe: while **exterior** lifetime `'a` is valid, then any reference inside `T` is valid too.

<br>

Consider following example:
```rust
use std::fmt::Display;

#[derive(Debug)]
struct Movie<'a, T> {
    title: &'a str,
    rating: T,
}

impl<'a, T: 'a + Display + PartialOrd> Movie<'a, T> {
    fn new(title: &'a str, rating: T) -> Self {
        Movie {
            title,
            rating,
        }
    }
}

fn main() {
    let movie = Movie::new("Foo", 100);
    println!("{:#?}", movie);
}
```

<br>

# 'static lifetime
There are 2 different cases:
1. As a **reference lifetime**: `&'static` indicates that the value reference points to lives **at least as long as** the **entire program**, in other words, `&'static` indicates that the value reference points to can **live forever**.
2. As a **trait bound**: `T: 'static` means that `T` **can't** have **any dynamic** references, in other words, **any references** inside the type `T` must have a `'static` lifetime.

<br>

## Rules for 'static lifetime bound
1. Any **owned type**, e.g. `i32`, `String`, `Vec`, satisfies the bound `T: 'static`.<br>
2. Any **struct** **without** lifetime parameters satisfies the bound `T: 'static`.<br>
3. If type has a lifetime parameter `<'a>` it **doesn't** satisfy the bound `T: 'static`.<br>

We are using the bound `T: 'static` to restrict `SomeType<'a>` to `SomeType<'static>`.<br>

<br>

## Usage
### `&str`
**String literal** has the type `&str`, but under the hood, `&str` is `&'static str` because the **reference** is **always alive**: it's **hardcoded into the data segment of the final binary**.

<br>

### `std::thread::spawn`
One common place this shows up is when you try to move values between threads with `std::thread::spawn`.<br>
The moved values need to be of types that implement `Send`, indicating that they're safe to move between threads, but they also need to **not** contain any **dynamic references** (the `'static` lifetime bound).<br>
This makes sense when you realize that a reference to something on the stack now raises the question: **which stack**? Each thread's stack is independent, and so lifetimes **can't** be tracked between them.<br>

<br>

### Return reference to value from function
`'static` *lifetime* **allows return reference to value from function**:
```Rust
fn create_string() -> &'static str {
    let s = String::from("abc");
    &s
}
```

<br>

# Anonymous lifetimes
Notation `<'_>` is called **anonymous lifetime** or **implicit lifetime**.<br>
The **implicit lifetime** `<'_>` tells Rust **to figure out the lifetime itself** and it is used to **simplify** `impl` blocks.

Consider following example:
```Rust
struct Foo<'a> {
    f: &'a str
}

impl<'a> Foo<'a> {
    fn foo(&self) {
        println!("{}", self.f)
    }
}

fn main() {
    let foo = Foo{f: "abc"};
    foo.foo();
}
```

The same code, but with **anonymous lifetime** is much simpler:
```Rust
struct Foo<'a> {
    f: &'a str
}

impl Foo<'_> {
    fn foo(&self) {
        println!("{}", self.f)
    }
}

fn main() {
    let foo = Foo{f: "abc"};
    foo.foo();
}
```

<br>

Consider following 2 examples:<br>
1. `make_wrapper` returns `MyType`:
```rust
struct MyType<'a>(&'a str);

fn make_wrapper(string: &str) -> MyType {
    MyType(string)
}
```
2. `make_wrapper` returns `-> MyType<'_>`:
```rust
struct MyType<'a>(&'a str);

fn make_wrapper(string: &str) -> MyType<'_> {
    MyType(string)
}
```

They are both compiled. But second is clearer because it tells us that `MyType` **contains** a reference.

<br>

# Higher-Rank Trait Bounds (HRTBs)
**HRTBs** are most commonly used with the `Fn*` traits.<br>
This compiles:
```rust
pub fn foo(f: impl for<'a> Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = f(&y);
    *z
}

fn main() {
    foo(|_| &5);
}
```

<br>

But this code does **not** compile:
```rust
pub fn bar<'a>(f: impl Fn(&'a i32) -> &'a i32) -> i32 {
    let y = 42;
    let z = f(&y);
    *z
}

fn main() {

}
```
Error:
```bash
y does not live long enough
```

<br>

- The **first** one says: the lifetime `'a` of the returned reference from `f` is the same as for the passed argument of `f` and it's **not** **directly related** to the **input** or **output** lifetimes of function `foo`.<br>
- The **second** one says: there is a lifetime `'a` for which `f` takes a reference with that lifetime and also returns a reference with that lifetime. The lifetime of **local variable** `y` passed to `f` is **shorter** than `'a` in this case, but it must be at least as long as `'a`.<br>

<br>

`for<'a>` means that the **reference can be valid for any lifetime** (hence a smaller lifetime can be used).<br>