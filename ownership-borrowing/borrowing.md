# What is reference
What if we want to let a function use a value **without transferring ownership**? Rust has feature for this concept, called **borrowing**.<br>

<br>

A **reference** is the **address** of **some value**.<br>
**Reference doesn’t own value it points to**, i.e., when *reference* **goes out of scope**, the **borrow ends**, and the **value** *reference* points to **isn't destroyed**.<br>

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

- **Owned type** means **non-reference type**, e.g. `i32`, `String`, `Vec`, etc.
- **Borrowed type** means **any reference type** *regardless of mutability*, e.g. `&i32`, `&mut i32`, etc.

<br>

**Borrowing** is the action of **creating a reference** to some value, called **lender**.<br>
**Borrower** is an **identifier** (**variable**) which **owns** some **reference**.<br>
Both, **immutable** and **mutable** **borrowers** can contain **mutable** or **immutable** **references**.<br>

<br>

So, there are **2 kinds of borrowers** in Rust:
1. **Immutable borrower** ``b``:
```Rust
let b: &T;
let b: &mut T;
```
1. **Mutable borrower** ``b``:
```Rust
let mut b: &T;
let mut b: &mut T;
```

If an **identifier** (**variable**) declared as **immutable**, it **isn’t possible** get a **mutable reference** to **it**.<br>

<br>

#### Example 1
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
- ``val`` mutable value
- ``&val``  shared reference
- ``&mut val``	mutable reference
- ``b1`` immutable borrower
- ``b2`` immutable borrower
- ``b3`` mutable borrower
- ``b4`` mutable borrower

<br>

#### Example 2.
```Rust
fn main() {
    let t = String::from("hello");
    let s = &t;  // borrowing
    println!("The length of '{}' is {}.", s.len());
}
```

Here: 
``s`` – borrower and it owns reference ``&t`` to ``t``.

<br>

#### Operators
|**Operator**|**Name**|**Description**|
|:-------|:---|:----------|
|``&``|**Reference operator**|To **borrow value**, i.e., take a reference.|
|``*``|**Dereference operator**|To **use a borrowed value**.|

<br>

# Restrictions of references
Since references do not own the value, **references cannot move the value**.<br>
But **references can copy value**.<br>

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
Consider ``b`` is **borrower**:
- if ``b`` contains *shared reference*, it is possible to **reborrow** *shared reference*: ``let b1 = &*b``.
- if ``b`` contains *mutable reference*, it is possible to **reborrow** *mutable reference*: ``let b1 = &mut *b``.

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

To fix this code it is needed to use **reborrow**: ``&mut *``.<br>

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

Because ``s`` is created inside ``dangle``, when the code of ``dangle`` is finished, ``s`` will be **deallocated**. <br>
But we tried to return a reference to it. That means this reference would be pointing to an dealocated ``String``. **Danger**!<br>

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

