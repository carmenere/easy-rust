# Table of contents
- [Table of contents](#table-of-contents)
- [Owned types. Borrowed types](#owned-types-borrowed-types)
- [References](#references)
    - [Example 1](#example-1)
    - [Example 2.](#example-2)
- [Operators](#operators)
- [NLL vs. LL](#nll-vs-ll)
  - [NLL and iterator invalidation](#nll-and-iterator-invalidation)
- [Lifetimes](#lifetimes)
- [Borrowing rules](#borrowing-rules)
    - [Examples](#examples)
      - [Lend the value inside scope of shared reference](#lend-the-value-inside-scope-of-shared-reference)
      - [Move by mutable reference](#move-by-mutable-reference)
      - [Move by shared reference](#move-by-shared-reference)
      - [Copy value](#copy-value)
      - [More complex example](#more-complex-example)
- [Reborrowing](#reborrowing)
- [Semantics for references](#semantics-for-references)
- [Dangling references](#dangling-references)

<br>

# Owned types. Borrowed types
There are **2 kinds of types** in Rust:
- **Owned type** means **non-reference type**, e.g. `i32`, `String`, `Vec`, etc.
- **Borrowed type** means **any reference type** *regardless of mutability*, e.g. `&i32`, `&mut i32`, etc.

<br>

# References
A **reference** is the **address** of **some value**.<br>
**Reference doesn’t own value it points to**, i.e., when *reference* **goes out of scope**, the **borrow ends**, and the **value** *reference* points to **isn't destroyed**.<br>
**References** allow to pass values to functions **without transferring ownership**.<br>

There are **2 kinds of references** in Rust:
1. **Shared reference** (aka **immutable reference**):
```Rust
&T
&'a T   // with lifetime
```
2. **Mutable reference**:
```Rust
&mut T
&'a mut T   // with lifetime
```

<br>

**Borrowing** is the action of **creating a reference** to some value, called **lender**.<br>
**Borrower** is an **identifier** (**variable**) which **owns** some **reference**.<br>
Both, **immutable** and **mutable** **borrowers** can contain **mutable** or **immutable** **references**.<br>
If an **identifier** (**variable**) declared as **immutable**, it **isn’t possible** to assign a **mutable reference** to **it**.<br>

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

### Example 1
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
- `&val`  shared reference
- `&mut val`	mutable reference
- `b1` immutable borrower
- `b2` immutable borrower
- `b3` mutable borrower
- `b4` mutable borrower

<br>

### Example 2.
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

# NLL vs. LL
**NLL** (**non-lexical lifetime**) vs. **LL** (**lexical lifetime**):
- **LL** means that **scope** of **identifier** starts **from** the point at which it was declared by `let` keyword **until** the **end of the block** (until `{`).
- **NLL** means that **scope** of **identifier** starts **from** the point at which it was declared by `let` keyword **until** the **last time it is used**.

Here **lifetime** is a synonym for **scope**.

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

# Lifetimes
Every **variable** in Rust has **LL** which **begins** when it is **created** by `let` keyword and **ends** when it is **destroyed** (closing curly bracket `}`).
Every **reference** in Rust has **NLL** which **begins** when it is **created** by `let` keyword and **ends** when it is **used last time**.<br>

But, the **reference** must be **valid** until the **lender** is **destroyed**. So, **lifetimes** and **scopes** (**NLL** and **LL**) are **not** the same.<br>

A **lifetime** is the **scope** within which a **reference** must be **valid**.<br>

*Lifetimes* are **denoted** with an **apostrophe**: `'a`, `'b`.<br>

<br>

# Borrowing rules
Borrowing rules:<br>
1. Scope of **mutable reference** `&mut T` **can’t** *intersect* with scope of any other reference to type `T`.
2. Scope of **shared reference** `&T` **can** *intersect* with scope of any other **shared reference** to type `T`.
3. Reference **can’t outlive value it points to**, i.e. the **borrow** must be **valid** **until** the **lender** is **destroyed**.
   - For example, function **cannot** return reference to value it owns.
4. Since reference **doesn't own** the value it points to, **reference cannot move the value**. But **reference can copy value**.

<br>

In other words, rules 1 and 2 are means: **at any given time** there can be:<br>
a. **only 1** *mutable reference* `&mut T`;<br>
**OR**<br>
b. **any number** of *shared references* `&T`.<br>

Rules 1 and 2 **prevent data races** at compile time.<br>
Rule 3 **prevents from dangling references**.<br>

**Owner restrictions** during borrowing:
1. During a **shared borrow**, the **owner can’t**:
   - **mutate** the *value*;
   - **mutably lend** the *value* (but still can **immutably lend** the *value*);
   - **move** the *value*.

2. During a **mutable borrow**, the **owner can’t**:
   - have **any access** (**read** or **mutate**) to the *value*;
   - **lend** (**mutably** or **immutably**) the *value*.

<br>

### Examples
#### Lend the value inside scope of shared reference
```Rust
fn main() {
    let mut owner = 5;
    let ro_ref = &owner;
    let rw = &mut owner;   // attempt to mutably lend the value inside scope of shared reference, error!
    println!("ro_ref: {}", ro_ref);
}
```

<br>

#### Move by mutable reference
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

#### Move by shared reference
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

#### Copy value
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

#### More complex example
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
Consider `b` is **borrower**:
- if `b` contains *shared reference*, it is possible to **reborrow** *shared reference*: `let b1 = &*b`.
- if `b` contains *mutable reference*, it is possible to **reborrow** *mutable reference*: `let b1 = &mut *b`.

<br>

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

