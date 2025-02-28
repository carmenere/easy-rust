# Table of contents

- [Table of contents](#table-of-contents)
- [Lifetimes](#lifetimes)
  - [Lifetimes in functions](#lifetimes-in-functions)
  - [Lifetimes in structs](#lifetimes-in-structs)
  - [Lifetimes in impl blocks](#lifetimes-in-impl-blocks)
- [Lifetimes and scopes](#lifetimes-and-scopes)
  - [NLL and iterator invalidation](#nll-and-iterator-invalidation)
  - [Desugar lifetimes](#desugar-lifetimes)
    - [Example 1](#example-1)
    - [Example 2](#example-2)
    - [Example 3: references that outlive referents](#example-3-references-that-outlive-referents)
- [Lifetime subtyping](#lifetime-subtyping)
  - [Example 1](#example-1-1)
  - [Example 2: `&mut T` is invariant over `T`](#example-2-mut-t-is-invariant-over-t)
  - [Example 3](#example-3)
- [Lifetime arithmetic](#lifetime-arithmetic)
    - [Type declaration](#type-declaration)
    - [Borrowing](#borrowing)
    - [Assigning reference to reference](#assigning-reference-to-reference)
    - [Struct's lifetimes](#structs-lifetimes)
    - [Double references](#double-references)
    - [Static scope](#static-scope)
  - [Examples](#examples)
    - [Example 1](#example-1-2)
    - [Example 2](#example-2-1)
    - [Example 3](#example-3-1)
    - [Example 4](#example-4)
- [Interpretation of lifetimes](#interpretation-of-lifetimes)
- [Lifetimes bounds](#lifetimes-bounds)
- ['static lifetime](#static-lifetime)
  - [Rules for 'static lifetime bound](#rules-for-static-lifetime-bound)
  - [Usage](#usage)
    - [`&str`](#str)
    - [`std::thread::spawn`](#stdthreadspawn)
    - [Return reference to value from function](#return-reference-to-value-from-function)
- [Anonymous lifetimes](#anonymous-lifetimes)
- [Higher-Rank Trait Bounds (HRTBs)](#higher-rank-trait-bounds-hrtbs)
  - [Example 1](#example-1-3)
  - [Example 2](#example-2-2)

<br>

# Lifetimes
Every **reference** (aka **borrow** or **borrower**) must be **valid** until the **lender** (aka **referent**) is **destroyed**.<br>
A **lifetime** is the **scope** within which a **reference** must be **valid**.<br>
*Lifetimes* are **denoted** with an **apostrophe**: `'a`, `'b`.<br>
The **lifetimes** help Rust find **dangling pointers**.<br>

Technically, **every** reference has some **lifetime** associated with it, but the compiler lets you **elide** them in common cases.<br>
It is called **lifetime elision** or **implicit lifetime annotation**. It is because the Rust compiler is smart enough to infer lifetimes in many cases.<br>
But sometimes it is needed to specify lifetimes **explicitly**.<br>

**Lifetimes rules**:
1. **Each** function’s parameter that is **reference** gets its **own** *lifetime parameter* (aka **elided lifetime**).
2. If there is exactly **one input** *lifetime parameter*, it is assigned to **all output** *lifetime parameters*.
3. If there are **multiple input** *lifetime parameters*, but one of them is `&self` or `&mut self`, the **lifetime** of `self` is assigned to **all output** *lifetime parameters*.

<br>

From Rust point of view, signature:`fn f (s1: &str, s2: &str) → &str` is **equal** to signature:`fn f<'a, 'b> (s1: &'a str, s2: &'b str) → &'??? str`<br>
So, `rustc` sets to `s1` and `s2` **different** lifetimes and `rustc` **doesn't** know what lifetime to assign to **returning value**.<br>
That is why compiler return error. So we must **explicitly** set lifetimes for input and output parameters.<br>
Example: `fn f<'a> (s1: &'a str, s2: &'a str) → &'a str`.<br>

<br>

From Rust point of view, this code:
```rust
fn run<'a>(&self, x: &'a Foo) -> &i32
```
is equivalent to:
```rust
fn run<'a, 'b>(&'b self, x: &'a Foo) -> &'b i32
```

<br>

## Lifetimes in functions
For function there are 2 kind of **lifetime parameters**:
- **Input lifetime parameter** is a lifetime associated with a **parameter** of a function. 
- **Output lifetime parameter** is a lifetime associated with the **return value** of a function.

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
- the **lifetime of a reference**, corresponding to the span of time in which that reference is **used** and **valid**;
- the **lifetime of a value**, corresponding to the span of time before that value is **destroyed**;

To distinguish these cases, we refer to *lifetime of a value* as **scope**.<br>

A **scope of value** means **Lexical Lifetime** (**LL**) which **begins** when value is **created** by `let var = value;` assigning to variable and **ends** when it is **destroyed** (closing curly bracket `}` or `drop()`).<br>
A **lifetime of a reference** means **Non-Lexical Lifetime** (**NLL**) which **begins** when **reference** is **created** by `let` keyword and **ends** when it is **used last time**.<br>

A reference is alive from the place it is created to its last use.<br>

**Lifetimes** and **scopes** are linked to one another. A **lifetime** is the **scope** within which a **reference** must be **valid**. If you make a reference to a value, the lifetime of that reference **cannot outlive** the scope of that value. Otherwise, your reference would be pointing into freed memory.<br>

<br>

## NLL and iterator invalidation
**NLL** prevents a common error called **iterator invalidation**, where the program modifies a collection while iterating over it.<br>

Rust rejects following code, because it borrows ``v`` both **immutably** and **mutably**:
```Rust
let mut v = vec![1, 2];

// Borrows `v` immutably
for i in &v {
    // Error: borrows `v` mutably, but `v` was already borrowed.
    v.push(*i);
}
```

<br>

## Desugar lifetimes
Each `let` statement **implicitly** introduces a **scope**.<br>

<br>

### Example 1
Rust desugar this simple piece of code:
```rust
let x: i32 = 0;
let y: &i32 = &x;
let z: &&i32 = &y;
```
to the following:
```rust
'a: {
    let x: i32 = 0;
    'b: {
        let y: &'b i32 = &'b x;
        'c: {
            let z: &'c &'b i32 = &'c y;
        }
    }
}
```

<br>

### Example 2
Rust desugar this simple piece of code:
```rust
let x: i32 = 0;
let z: &i32;
let y: &i32 = &x;
z = y;
```
to the following:
```rust
'a: {
    let x: i32 = 0;
    'b: {
        let z: &'b i32;
        'c: {
            // Must use 'b here because the reference to x is being passed to the scope 'b.
            let y: &'b i32 = &'b x;
            z = y;
        }
    }
}
```

<br>

### Example 3: references that outlive referents
Consider example:
```rust
fn as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data);
        return &'a s
    }
}

fn main() {
    'c: {
        let x: u32 = 0;
        'd: {
            println!("{}", as_str::<'d>(&'d x));
        }
    }
}
```

<br>

The contract of `as_str` says that the reference `&str` must outlive `'a`.<br>
Unfortunately, `s` was defined in the scope `'b`, so the **only** way this is **sound** is if `'b` contains `'a`, but it is **false**.<br>
We have therefore created a reference whose lifetime outlives its referent.<br>

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

Notation `'long: 'short` means `'long` **outlives** `'short`.<br>

<br>

**Subtyping** is the idea that one type (called **subtype**) can be used in place of another type.<br>

Given two types `Sub` and `Super`, where `Sub` is a **subtype** of `Super`. The **variance** over generic `T` defines **relationships** between **generic types** `F<T>`. <br>

There are **three** kinds of **variance**:
- `F<T>` is **covariant over** `T` if `T` is a **subtype** of `U` then `F<T>` is a **subtype** of `F<U>`;
- `F<T>` is **contravariant over** `T` if `T` is a **subtype** of `U` then `F<U>` is a **subtype** of `F<T>`;
- `F<T>` is **invariant over** `T` otherwise (**no subtyping relation can be derived**);

<br>

**In Rust**:
|Type|Variance in `'a`|Variance in `T`|
|:---|:-------------|:------------|
|`&'a T`|covariant|covariant|
|`&'a mut T`|covariant|**invariant**|
|`dyn Trait<T> + 'a`|covariant|**invariant**|

<br>

**Variance over** `'a`:
- The type `&'a T` is **covariant over** `'a`. In other words, if `'long` **outlives** `'short`, then `&'long T` is a **subtype** of `&'short T`. That is, `&'long T` can be used wherever `&'short T` is expected (because it lives at least as long).
- The type `&'a mut T` is also **covariant over** `'a`. In other words, if `'long` **outlives** `'short`, then `&'long mut T` is a **subtype** of `&'short mut T`. That is, `&'long mut T` can be used wherever `&'short mut T` is expected (because it lives at least as long).

<br>

**Variance over** `T`:
- The type `&'a T` is **covariant over** `T`. In other words, if `T` is a **subtype** of `U` then, then `&'a T` is a **subtype** of `&'a U`. That is, `&'a T` can be used wherever `&'a U` is expected.
- The type `&'a mut T` is **invariant over** `T`. In other words, if `T` is a **subtype** of `U` then, then **neither** `&'a mut T` is a subtype of `&'a mut U` **nor** `&'a mut U` is a subtype of `&'a mut T`. That is, `&'a mut T` **cannot** be used wherever `&'a mut U` is expected and vice versa.

<br>

Consider `T` is a **subtype** of `U`, for example, `T = &'long str` and `U = &'short str`. This means that:
- `&'a &'long str` is a **subtype** of `&'a &'short str`;
- `&'a mut &'long str` **cannot** be a **subtype** of `&'a mut &'short str`, even if `&'long str` is a **subtype** of `&'short str`;

<br>

Consider the following example: **string literals** always have `'static` lifetime. Nevertheless, we can assign `s` to `t`:
```rust
fn bar<'a>() {
    let s: &'static str = "hi";
    let t: &'a str = s;
}
```

Since `'static` **outlives** the lifetime parameter `'a`, `&'static str` is a **subtype** of `&'a str`.<br>

<br>

## Example 1
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

Here `'b` specifies that lifetimes of the `Movie` struct **outlives** the `Reviewer` struct.

<br>

## Example 2: `&mut T` is invariant over `T`
```rust
fn assign<T>(input: &mut T, val: T) {
    *input = val;
}

fn main() {
    let mut hello: &'static str = "hello";
    {
        let world = String::from("world");
        assign(&mut hello, &world);
    }
    println!("{hello}");
}
```

All `assign` does is take a **mutable reference** and a **value** and **overwrite** the **referent** with it.<br>
In the **caller** we pass in `&mut &'static str` and `&'world str`.<br>
Because `&mut T` is **invariant** over `T`, the compiler concludes it **can't** apply any subtyping to the first argument, and so `T` must be exactly `&'static str`.<br>

<br>

This works:
```rust
fn main() {
    let mut hello: &'static str = "hello";
    {
        let world: &'static str = &"world";
        assign(&mut hello, &world);
    }
    println!("{hello}");
}
```

<br>

## Example 3
This **doesn't** compile:
```rust
fn deref0<'a, 'b>(v: &'b &'a mut u32) -> &'a u32 { 
    *v
}
```
**Error**:
```
lifetime may not live long enough
consider adding the following bound: 'b: 'a
```

<br>

But, this works:
```rust
fn deref1<'a, 'b>(v: &'b &'a mut u32) -> &'b u32 { 
    *v
}
```

And this works:
```rust
fn deref2<'a, 'b>(v: &'b &'a u32) -> &'a u32 { 
    *v
}
```

<br>

# Lifetime arithmetic
In fact, **scopes** and **lifetimes** are **different concepts**:
- in rust code, **all** objects, including constants, owned variables and references, **have scopes**;
- **lifetime parameters** are associated with **references** to express **relationships between scopes**;

<br>

For the expression `x: &'a T`, instead of saying `'a` is the lifetime of `x`, we should say: `'a` is a **lifetime parameter associated** with the **reference** `x`.<br>
In terms of algebra, **scopes** are **values** like `1`, `2`, `3`, and **lifetimes** are **variables** like `x`, `y`, `z`.<br>

<br>

### Type declaration
```rust
x: &'a T
```

<br>

The **lifetime** `'a` associated with the reference **outlives scope** of identifier `x` (denote scope of `x` as `'x`) that stores this reference, i.e. `'a: 'x`.<br>

<br>

### Borrowing
```rust
x: &'a T = &y
```
here `y` is **lender**.<br>

<br>

The **scope** of **lender** `y` (denote scope of `y` as `'y`) **outlives** the **lifetime** `'a` associated with the reference to lender: `'y: 'a`.<br>
In other words, **lifetime** of **reference** is *less than or equal to* **sope** of **lender**.<br>

<br>

### Assigning reference to reference
```rust
x: &'a T = y: &'b T
```
here `x` and `y` are both references.<br>

<br>

The **lifetime** `'b` of the **assignor** `y` **outlives** the **lifetime** `'a` of the **assignee** `x`: `'b: 'a`.<br>

<br>

### Struct's lifetimes
Consider **struct** that **contains reference**:
```rust
S<'b> { x: &'b T }
```

And consider **reference to struct**:
```rust
&'a S<'b>
```

<br>

The **lifetime** `'b` associated with the *member of struct* **otlives** the **lifetime** `'a` associated with a **reference to struct**: `'b: 'a`.<br>

<br>

### Double references
```rust
x: &'b &'a T
```

<br>

The lifetime `'a` **otlives** the lifetime `'b`: `'a: 'b`.<br>

<br>

### Static scope
Static objects are **not** located in **stack** or **heap**. They are **located** in **data segments** or **code segments** that are mapped to the process memory.<br>
`'static` lifetime **outlives** any lifetime `'a`: `'static: 'a`.<br>

<br>

## Examples
### Example 1
```rust
{
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
```
The compiler tries to associate a lifetime `'a` with reference `r` that satisfies the follow inequalities:
- rule 1: `​scope(r)` ⊆ `'a`;
- rule 2: `​'a` ⊆ `scope(x)`;
​​ 
​<br>

### Example 2
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string is long");
    let s2 = String::from("xyz");
    let result;
    {
        let rs1 = &s1;
        let rs2 = &s2;
        result = longest(rs1, rs2);
    }
    println!("The longest string is {}", result);
}
```

In this example, lifetime `'a` is associated with both `rs1` and `rs2`. The compiler needs to find a lifetime `'a` that satisfies:
- `​scope(rs1)` ⊆ `'a` ⊆ `scope(s1)`;
- `​scope(rs2)` ⊆ `'a` ⊆ `scope(s2)`;
- `​scope(result)` ⊆ `'a`;

And `scope(result)` satisfies these inequalities, so `'a` could be `scope(result)`, and the compiler passes the check;

<br>

### Example 3
```rust
struct S<'a> {
    x: &'a u32,
}

fn foo<'a, 'b, 'c, 'd>(s: &'b S<'a>) -> &'d S<'c> where 'a: 'c, 'b: 'd {
    s
}
```
The constraints in the where clause are necessary in order to satisfy the **assignment rule**.<br>
There are also two implied constraints from the struct reference rule: `'a: 'b` and `'c: 'd`.<br>

<br>

### Example 4
```rust
#[derive(Debug)]
struct S {}

fn main() {
    let x = S {};
    let y = &x;
    let z = x;
    println!("{:?}", y);
}
```
An instance of struct `S` is first bound to `x`, then moved to `z`. But the scope of `x` ends when `x` is moved to `z`.<br>
Any lifetime associated with `y` could **not** be satisfied because `scope(y)` ⊈ `scope(x)`. Thus the code does **not** compile.<br>

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
To move values between threads with `std::thread::spawn` thier types need to implement `Send`, but they also need to **not** contain any **dynamic references** (the `'static` lifetime bound).

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

Explanation:
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
