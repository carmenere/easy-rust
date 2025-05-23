# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Owned types. Borrowed types](#owned-types-borrowed-types)
* [References](#references)
  * [Example 1](#example-1)
  * [Example 2.](#example-2)
* [Operators](#operators)
* [Borrowing rules](#borrowing-rules)
  * [Examples](#examples)
    * [Lend the value inside scope of shared reference](#lend-the-value-inside-scope-of-shared-reference)
    * [Move by mutable reference](#move-by-mutable-reference)
    * [Move by shared reference](#move-by-shared-reference)
    * [Copy value](#copy-value)
    * [More complex example](#more-complex-example)
* [Reborrowing](#reborrowing)
  * [Example 1](#example-1-1)
  * [Example 2](#example-2-1)
* [Semantics for references](#semantics-for-references)
* [Dangling references](#dangling-references)
<!-- TOC -->

<br>

# Owned types. Borrowed types
There are **2 kinds of types** in Rust:
- **owned type** means **non-reference type**, e.g. `i32`, `String`, `Vec`, etc;
- **borrowed type** means **any reference type** *regardless of mutability*, e.g. `&i32`, `&mut i32`, etc;

<br>

# References
A **reference** (aka **borrow**) is an **address** of **some value** (aka **referent**, **lender**).<br>
**Reference doesn’t own value it points to**, i.e., when *reference* **goes out of scope**, the **borrow ends**, and the **value** *reference* points to **isn't destroyed**.<br>
**References** allow to pass values to functions **without transferring ownership**.<br>

There are 2 kind of references:
- **shared references** (aka **immutable references**): `&T` or `&'a T`;
- **exclusive references** (aka **mutable references**): `&mut T` or `&'a mut T`;

<br>

**Borrowing** is the action of **creating a reference** to some value.<br>
**Borrower** is an **identifier** (**variable**) which **owns** some **reference**.<br>
Both, **immutable** and **mutable** **borrowers** can contain **mutable** or **immutable** **references**.<br>
If a **borrower** declared as **immutable**, it **isn’t possible** to assign a **mutable reference** to **it**.<br>

<br>

So, there are **2 kinds of borrowers** in Rust:
1. **Immutable borrower** `b`:
```Rust
let b: &T;
let b: &mut T;
```
2. **Mutable borrower** `b`:
```Rust
let mut b: &T;
let mut b: &mut T;
```

<br>

## Example 1
```Rust
fn main() {
    let mut val = 100;
    let b1 = &val;
    let b2 = &mut val;
    let mut b3 = &val;
    let mut b4 = &mut val;
}
```

Here:
- `val` mutable value
- `&val` shared reference
- `&mut val` mutable reference
- `b1` immutable borrower
- `b2` immutable borrower
- `b3` mutable borrower
- `b4` mutable borrower

<br>

## Example 2.
```Rust
fn main() {
    let t = String::from("hello");
    let s = &t;  // borrowing
    println!("The length of '{}' is {}.", s.len());
}
```

Here: 
`s` – borrower and it owns reference `&t` to `t`.

<br>

# Operators
|**Operator**|**Name**|**Description**|
|:-------|:---|:----------|
|`&`|**Reference operator**|To **borrow value**, i.e., take a reference.|
|`*`|**Dereference operator**|To **use a borrowed value**.|

<br>

# Borrowing rules
The **borrowing rules**:
1. **Any** reference **cannot outlive** the **value** it points to;
   - for example, function **cannot** return reference to value it owns;
2. An **exclusive** references **cannot be aliased**; 
3. **Any** reference **doesn't own** the **value** it points to;
   - in other words, the **value** reference points to **cannot be moved through dereferencing**;
   - when *reference* **goes out of scope**, the **borrow ends**, and the **value** *reference* points to **isn't destroyed**;

**What aliased mean**? _Variables_ and _pointers_ **alias** if they **point to overlapping regions of memory**.<br>

<br>

Rules 1 and 2 **prevent data races** at compile time.<br>
Rule 3 **prevents from dangling references**.<br>

<br>

In other words, rules **1** and **2** mean: **at any given time** there can be:<br>
a. **only 1** *mutable reference* `&mut T`;<br>
**OR**<br>
b. **any number** of *shared references* `&T`.<br>

<br>

**Owner restrictions** during borrowing:
1. During a **shared borrow**, the **owner can’t**:
   - **mutate** the *value*;
   - **mutably lend** the *value* (but still can **immutably lend** the *value*);
   - **move** the *value*;

2. During a **mutable borrow**, the **owner can’t**:
   - have **any access** (**read** or **mutate**) to the *value*;
   - **lend** (**mutably** or **immutably**) the *value*;

<br>

## Examples
### Lend the value inside scope of shared reference
```Rust
fn main() {
    let mut owner = 5;
    let ro_ref = &owner;
    let rw = &mut owner;   // attempt to mutably lend the value inside scope of shared reference, error!
    println!("ro_ref: {}", ro_ref);
}
```

<br>

### Move by mutable reference
```Rust
fn main() {
    struct Foo(i32);
    let mut i1 = Foo(1);
    let r = &mut i1;
    let mut i2 = *r;
}
```

**Output**:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0507]: cannot move out of `*r` which is behind a mutable reference
  --> src/main.rs:14:18
   |
14 |     let mut i2 = *r;
   |                  ^^
   |                  |
   |                  move occurs because `*r` has type `Foo`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&*r`

For more information about this error, try `rustc --explain E0507`.
error: could not compile `playrs` due to previous error
```

<br>

### Move by shared reference
```Rust
fn main() {
    struct Foo(i32);
    let mut i1 = Foo(1);
    let r = &i1;
    let mut i2 = *r;
}
```

**Output**:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0507]: cannot move out of `*r` which is behind a shared reference
  --> src/main.rs:14:18
   |
14 |     let mut i2 = *r;
   |                  ^^
   |                  |
   |                  move occurs because `*r` has type `Foo`, which does not implement the `Copy` trait
   |                  help: consider borrowing here: `&*r`

For more information about this error, try `rustc --explain E0507`.
error: could not compile `playrs` due to previous error
```

<br>

### Copy value
```Rust
fn main() {
    let mut i1 = 100;
    let r = &i1;
    let mut i2 = *r;
}
```

**Output**:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/playrs`
```

<br>

### More complex example
```Rust
fn main() {
    enum Foo {
        A,
    }

    struct Bar {
        a: i32,
    }

    fn main() {
        let foo = &Foo::A;
        let bar = &Bar{a: 0};

        let foo_2 = *foo; // A move occurs here and Rust generates following error:
                          // cannot move out of `*foo` which is behind a shared reference

        let bar_2 = *bar; // A move occurs here and Rust generates following error:
                          // cannot move out of `*bar` which is behind a shared reference

        let a = (*bar).a; // *bar works here because Rust doesn't move it, Rust understands that it only needs acccess field "a".
    }
}
```

<br>

# Reborrowing
**Shared references** `&T` implement `Copy`, which makes them very flexible: just copy it.<br>
**Exclusive references** `&mut T` **do not** implement `Copy`. Instead, you can use them through a **reborrow**. **Reborrow** is what makes `&mut` **usable**.<br>
**Reborrow** creates **new** reference with **new shorter lifetime** than lifetime of **original** reference.<br>
The _original reference_ **cannot** be used **until** _reborrowed reference_ **in use**.<br>

<br>

Consider `b` is **borrower**:
- if `b` contains *shared reference*, it is possible to **reborrow** *shared reference*: `let b1 = &*b`.
- if `b` contains *mutable reference*, it is possible to **reborrow** *mutable reference*: `let b1 = &mut *b`.

<br>

There's a lot of **implicit reborrowing** in Rust. For example, in function calls **mutable references aren’t moved**, they are **implicitly reborrowed without invaliding the original reference**.<br>

<br>

## Example 1
Following code **does not work**:
```Rust
struct Foo {
    val: i32,
}

fn main() {
    let mut owner = Foo { val: 100 };
    let p0 = &mut owner;
    let p1 = p0;
    println!("p1 => {}", p1.val);
    println!("p0 => {}", p0.val);
}
```

To fix this code it is needed to use **reborrow**: `&mut *`.<br>

Following code **works**:
```Rust
struct Foo {
    val: i32,
}

fn main() {
    let mut owner = Foo { val: 100 };
    let p0 = &mut owner;
    let p1 = &mut *p0;
    println!("p1 => {}", p1.val);
    println!("p0 => {}", p0.val);
}
```

<br>

## Example 2
```rust
fn foo<'v>(v: &'v mut Vec<i32>) {
    v.push(0);         // line 1
    println!("{v:?}"); // line 2
}
```

You're **not** moving `v: &mut Vec<i32>` when you pass it to `push` on _line 1_, or you couldn't print it on line 2.<br>
But you're **not** copying it either, because `&mut _` does not implement `Copy`.<br>
Instead `*v` is **reborrowed** for some **shorter lifetime** than `'v`, which ends on _line 1_.<br>

<br>

An **explicit reborrow** looks like this:
```rust
Vec::push(&mut *v, 0);
```

The `v` **can't** be used **while** the **reborrow** `&mut *v` **exists**, but after it **expires**, you can use `v` **again**.<br>
In this way, both `&mut` are still **exclusive borrows**.<br>

<br>

# Semantics for references
Semantics for references:
- **Shared reference** has **Copy semantics**.
- **Mutable reference** has **Move semantics**.

But **in function calls** **mutable references** **aren’t moved**, they are **implicitly reborrowed**.

Following code **does not work**:
```Rust
fn some_foo(f: Foo) {
    println!("f => {:?}", f);
}

#[derive(Debug)]
struct Foo {
    val: i32,
}

fn main() {
    let mut owner = Foo { val: 100 };
    some_foo(owner);
    println!("owner = {:?}", owner);
}
```

<br>

Following code **works**:
```rust
fn some_foo(p: &mut Foo) {
    println!("p = {:?}", p);
}

#[derive(Debug)]
struct Foo {
    val: i32,
}

fn main() {
    let mut owner = Foo { val: 100 };
    let p0 = &mut owner;
    some_foo(p0);
    println!("p0 = {:?}", p0);
}
```

<br>

# Dangling references
**Dangling reference** is **reference** to **deallocated resource**.<br>
Rust gives compiler guarantees that references will never be dangling references.<br>

Example:
```Rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a shared reference to the String.
} // Here, s goes out of scope, and is dropped.
```

Because `s` is created inside `dangle`, when the code of `dangle` is finished, `s` will be **deallocated**. <br>
But we tried to return a reference to it. That means this reference would be pointing to an dealocated `String`. **Danger**!<br>

Solutions:
1. Use **lifetime** for reference.
2. Return **ownership**.

<br>

Return **ownership** example:
```Rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

