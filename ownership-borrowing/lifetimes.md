# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Lifetimes](#lifetimes)
  * [Lifetime elision](#lifetime-elision)
  * [Lifetimes in structs](#lifetimes-in-structs)
  * [Lifetimes in impl blocks](#lifetimes-in-impl-blocks)
* [Lifetimes and scopes](#lifetimes-and-scopes)
* [Iterator invalidation](#iterator-invalidation)
* [Lifetime subtyping](#lifetime-subtyping)
* [Lifetimes bounds](#lifetimes-bounds)
* ['static lifetime](#static-lifetime)
  * [Rules for `'static` lifetime bound](#rules-for-static-lifetime-bound)
  * [`&str`](#str)
  * [`std::thread::spawn`](#stdthreadspawn)
  * [Returning references from functions](#returning-references-from-functions)
* [Anonymous lifetimes](#anonymous-lifetimes)
* [Higher-Rank Trait Bounds (HRTBs)](#higher-rank-trait-bounds-hrtbs)
  * [Example 1](#example-1)
  * [Example 2](#example-2)
<!-- TOC -->

<br>

# Lifetimes
Every **reference** (aka **borrow** or **borrower**) must be **valid** until the **lender** (aka **referent**) is **destroyed**.<br>
A **lifetime** is the **scope** within which a **reference** must be **valid**.<br>
*Lifetimes* are **denoted** with an **apostrophe**: `'a`, `'b`.<br>
In the expression `x: &'a T`, instead of saying `'a` is the **lifetime** of `x`, we should say: `'a` is a **lifetime parameter associated** with the type of `x`.<br>
The **lifetimes** help Rust find **dangling pointers**.<br>

<br>

There are 2 kind of **lifetime parameters**:
- **input lifetime parameter** is a lifetime associated with a **parameter** of a function;
- **output lifetime parameter** is a lifetime associated with the **return value** of a function;

<br>

## Lifetime elision
Technically, **every** reference has some **lifetime** associated with it, but the compiler lets you **elide** them in common cases.<br>
It is called **lifetime elision** or **implicit lifetime annotation**. It is because the Rust compiler is smart enough to infer lifetimes in many cases.<br>

<br>

**Elision rules**:
1. **Each** function’s parameter that is **reference** gets its **own** *lifetime parameter* (aka **elided lifetime**).
2. If there is exactly **one input** *lifetime parameter*, it is assigned to **all output** *lifetime parameters*.
3. If there are **multiple input** *lifetime parameters*, but one of them is `&self` or `&mut self`, the **lifetime** of `self` is assigned to **all output** *lifetime parameters*.

<br>

**Example**:<br>
According to elision rules:
```rust
fn run<'a>(&self, x: &'a Foo) -> &i32
```
the compiler will assign the `'b` to `&self` and `-> &i32` itself:
```rust
fn run<'a, 'b>(&'b self, x: &'a Foo) -> &'b i32
```

<br>

But sometimes it is needed to specify _lifetimes_ **explicitly**. From Rust point of view, signature:
```rust
fn f (s1: &str, s2: &str) → &str
```
is **equal** to signature:
```rust
fn f<'a, 'b> (s1: &'a str, s2: &'b str) → &'??? str
```

So, `compiler` sets to `s1` and `s2` **different** _lifetimes_ and **doesn't** know what _lifetime_ to assign to **returning value**.<br>
That is why compiler return **error**. So we must **explicitly** set _lifetimes_ for **input** and **output** parameters:
```rust
fn f<'a> (s1: &'a str, s2: &'a str) → &'a str
```

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

# Lifetimes and scopes
In everyday speech, the word **lifetime** can be used in two distinct – but similar – ways:
- the **lifetime of a reference**;
- the **lifetime of a value** aka (**liveness scope**);

To distinguish these cases, we refer to *lifetime of a value* as **scope**.<br>

A **scope of value** means **lexical lifetime** (**LL**) which **begins** when value is **created** and **ends** when it is **destroyed** (closing curly bracket `}` or `drop()`).<br>
A **lifetime of a reference** means **Non-Lexical Lifetime** (**NLL**) which **begins** when **reference** is **created** by `let` keyword and **ends** when it is **used last time**. Each `let` statement **implicitly** introduces a **scope**.<br>

<br>

**Lifetimes** and **scopes** are linked to one another: if you make a reference to a value, the lifetime of that reference **cannot outlive** that value. Otherwise, your reference would be pointing into freed memory.<br>

<br>

# Iterator invalidation
**NLL** prevents a common error called **iterator invalidation**, where the program modifies a collection while iterating over it.<br>

Rust rejects following code, because it borrows `v` both **immutably** and **mutably**:
```Rust
let mut v = vec![1, 2];

// Borrows `v` immutably
for i in &v {
    // Error: borrows `v` mutably, but `v` was already borrowed.
    v.push(*i);
}
```

<br>

# Lifetime subtyping
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

It **doesn't** compile:
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

Notation `'long: 'short` (or `'l: 's`) means `'l` **outlives** `'s`. It sometimes called an **outlive constraints**.<br>

<br>

**Properties**:
- constraints `'a: 'b` and `'b: 'a` imply that `'a == 'b`;
- types `&'a str` and `&'b str` are **not** the same type, unless `'a == 'b`;


<br>

# Lifetimes bounds
**Syntactic forms**:
- `S<T> where T: 'a` **lifetime bound**: it means if `T` has **references**, they all **must outlive** `'a`;
- `S<T> where 'b: 'a` **lifetime** `'b` **must live** *at least as long as* (**outlive**) `'a` **bound**;

<br>

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
Static objects are **not** located in **stack** or **heap**. They are **located** in **data segments** or **code segments** that are mapped to the process memory.<br>
`'static` lifetime **outlives** any lifetime `'a`: `'static: 'a`.<br>

There are 2 different cases:
1. As a **reference lifetime**: `&'static` indicates that the value reference points to lives **at least as long as** the **entire program**, in other words, `&'static` indicates that the value reference points to can **live forever**.
2. As a **trait bound**: `T: 'static` means that `T` **can't** have **any dynamic** references, in other words, **any references** inside the type `T` must have a `'static` lifetime.

<br>

## Rules for `'static` lifetime bound
1. Any **owned type**, e.g. `i32`, `String`, `Vec`, satisfies the bound `T: 'static`.<br>
2. Any **struct** **without** lifetime parameters satisfies the bound `T: 'static`.<br>
3. If type has a lifetime parameter `<'a>` it **doesn't** satisfy the bound `T: 'static`.<br>

We are using the bound `T: 'static` to restrict `SomeType<'a>` to `SomeType<'static>`.<br>

<br>

## `&str`
**String literal** has the type `&str`, but under the hood, `&str` is `&'static str` because the **reference** is **always alive**: it's **hardcoded into the data segment of the final binary**.

<br>

## `std::thread::spawn`
To move values between threads with `std::thread::spawn` thier types need to implement `Send`, but they also need to **not** contain any **dynamic references** (the `'static` lifetime bound).

<br>

## Returning references from functions
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

They both compile without errors. But **second** is **clearer** because it tells us that `MyType` **contains** at least one **reference**.

<br>

# Higher-Rank Trait Bounds (HRTBs)
## Example 1
Consider example:
```rust
use std::fmt::Display;

trait Processor {
    fn process<T: Display>(&self, value: T) -> String;
}

struct MyProcessor;
 
impl Processor for MyProcessor {
    fn process<T: Display>(&self, value: T) -> String {
        format!("{}", value)
    }
}

fn get_processor<P>(processor: P) -> impl Fn(&str) -> String
where
    P: Processor,
{
    move |value| processor.process(value)
}

fn main(){
    let processor = MyProcessor;
    let process_closure = get_processor(processor);
 
    let item_1 = "10".to_string();
    let item_2 = "20".to_string();
 
    println!("{}", process_closure(&item_1));
    println!("{}", process_closure(&item_2));
}
```

It compiles.<br>

Consider specifying lifetimes explicitly:
```rust
fn get_processor<'a, P>(processor: P) -> impl Fn(&'a str) -> String
where
    P: Processor,
{
    move |value| processor.process(value)
}
```

<br>

We have specified the lifetime `'a`, but it's **not** used in the function arguments, instead it's used in the argument of the closure returned by the function.<br>
This syntax means that *references* that are *passed to the closure* returned by `get_processor()` must **outlive** *closure* itself.<br>

This syntax causes to `error[E0597]: item_1 does not live long enough`.<br>

**Explanation**:
- the closure is stored in the `process_closure` variable, which lives until the end of the main function;
- values in Rust are dropped in reverse order, meaning `item_2` is dropped, then `item_1`, and then closure stored in `process_closure`;
- this means that the strings `item_1` and `item_2` will be dropped **before** the closure stored in `process_closure`;
- once `item_1` is dropped, all the references to `item_1` will be invalid;
- the problem is that the closure expects a reference that lives as long as the closure itself, but the string `item_1` is dropped before the closure;

<br>

We can **increase the lifetime** of the `item_1` and `item_2` by moving them **above** the `process_closure`:
```rust
fn main(){
    let processor = MyProcessor;
    let item_2 = "20".to_string();
    let item_1 = "10".to_string();
 
    let process_closure = get_processor(processor);
 
    println!("{}", process_closure(&item_1));
    println!("{}", process_closure(&item_2));
}
```

But this is not a good approach.<br>

The solution is to use `for<'a>` syntax which is called **H**igher-**R**anked **T**rait **B**ounds (**HRTB**).<br>
The `for<'a>` syntax means that the **reference can be valid for any lifetime** (hence a **smaller** lifetime **can** be used).<br>
In other words, the **returned closure** can be called with references of **any** lifetime.<br>

<br>

**Example** of closure with **HRTB**:
```rust
fn get_processor<P>(processor: P) -> impl for<'a> Fn(&'a str) -> String
where
    P: Processor,
{
    move |value| processor.process(value)
}
```

<br>

## Example 2
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
- The **second** one says: there is a lifetime `'a` for which `f` takes a reference with that lifetime and also returns a reference with that lifetime. Lifetime `'a` is connected to function `foo` and `'a` outlives **scope of function** `foo`, but the lifetime of **local variable** `y` passed to `f` is **shorter** than `'a` in this case, but it must be at least as long as `'a`.<br>
