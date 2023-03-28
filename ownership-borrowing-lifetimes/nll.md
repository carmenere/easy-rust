# Rules of references
A **lifetime** is a **scope** within a **reference** is **valid**, i.e., until **borrow** lasts.<br>

**NLL** (**non-lexical lifetime**) vs. **LL** (**lexical lifetime**):
- **LL** means that **scope** of reference starts **from** the point at which it was declared by ``let`` keyword **until** the **end of the block** (until ``{``).
- **NLL** means that **scope** of reference starts **from** the point at which it was declared by ``let`` keyword **until** the **last time reference is used**.

All **references** in Rust have **NLL**.<br>

<br>

> **NLL rules**:<br>
> 1. Scope of **mutable reference** ``&mut T`` **can’t** *intersect* with scope of any other reference to type ``T``.<br>
> 2. Scope of **shared reference** ``&T`` **can** *intersect* with scope of any other reference to type ``T``.<br>
> 3. Reference **can’t outlive value it points to**, i.e. function cannot return reference to value it owns.<br>

<br>

In other words, rules 1 and 2 are means: **at any given time** there can be:
- **only 1** *mutable reference* ``&mut T``;
**OR** 
- **any number** of *shared references* ``&T``.

<br>

Rules 1 and 2 **prevent data races** at compile time.<br>
Rule 3 **prevents from dangling references**.<br>

<br>

**Owner restrictions** during borrowing:
1. During a **shared borrow**, the **owner can’t**:
   - **mutate** the *value*;
   - **mutably lend** the *value* (but still can **immutably lend** the *value*);
   - **move** the *value*.

2. During a **mutable borrow**, the **owner can’t**:
   - have **any access** (**read** or **mutate**) to the *value*;
   - **lend** (**mutably** or **immutably**) the *value*.

<br>

#### Example
```Rust
fn main() {
    let mut owner = 5;
    let ro_ref = &owner;
    let rw = &mut owner;   // attempt to mutably lend the value inside scope of shared reference, error!
    println!("ro_ref: {}", ro_ref);
}
```

<br>

# NLL and iterator invalidation
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

## Examples
### 1. NLL of p0 and NLL of p1 start and end at the point at which they were borrowed and do not intersect with NLL of owner
```Rust
fn main() {
    let mut owner = Foo { f: 0 };    // NLL of owner starts here

    let p0 = &mut owner;               // NLL of p0 starts and ends here
    
    println!("owner: {}", owner.f);  // Read of owner here

    let p1 = &mut owner;               // NLL of p1 starts and ends here
    
    println!("owner: {}", owner.f);  // NLL of owner ends here
}
```

Output:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/playrs`
owner: 0
owner: 0
```

<br>

### 2. Scopes do not intersect because NLL of p0 and NLL of p1 start and end at the point at which they were borrowed
```Rust
fn main() {
    let mut owner = Foo { f: 0 };    // NLL of owner starts here

    let p0 = &mut owner;                // NLL of p0 starts here

    println!("owner: {}", owner.f);  // Read of owner here

    println!("owner: {}", owner.f);  // Latest read of owner here; NLL of owner ends here

    println!("p0: {}", p0.f);           // NLL of p0 ends here
}
```

Output:
```bash
cargo run 
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org) 
error[E0502]: cannot borrow `owner.f` as immutable because it is also borrowed as mutable 
  --> src/main.rs:23:27 
   | 
21 |     let p0 = &mut owner; 
   |              ---------- mutable borrow occurs here 
22 | 
23 |     println!("owner: {}", owner.f); 
   |                           ^^^^^^^ immutable borrow occurs here 
24 | 
25 |     println!("p0: {}", p0.f);
   |                        ---- mutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `playrs` due to previous error
```

<br>

### 3. Owner is used after NLL of p0 ends
```Rust
fn main() {
    let mut owner = Foo { f: 0 };     // NLL of owner starts here

    let p0 = &mut owner;                 // NLL of p0 starts here

    println!("p0: {}", p0.f);            // Read of p0 here

    println!("p0: {}", p0.f);            // Latest read of p0 here; NLL of p0 ends here

    println!("owner: {}", owner.f);   // Latest read of owner here; NLL of owner ends here
}
```

Output:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/playrs`
p0: 0
owner: 0
```

<br>

### 4. Mutable reference has move semantics in assignment
```Rust
fn main() {
    let mut owner = Foo { f: 0 };

    let p0 = &mut owner;

    let p1 = p0;
    
    println!("p0: {}", p0.f);
}
```

Output:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0382]: borrow of moved value: `p0`
  --> src/main.rs:33:24
   |
29 |     let p0 = &mut owner;
   |         -- move occurs because `p0` has type `&mut Foo`, which does not implement the `Copy` trait
30 |
31 |     let p1 = p0;
   |              -- value moved here
32 |
33 |     println!("p0: {}", p0.f);
   |                        ^^^^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playrs` due to previous error
```

<br>

### 5. Mutable reference implicitly reborrowed (type coercion) when passing in function
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]

fn reborrow_rw(v: &mut Foo) {
    let p = &mut *v;
    println!("reborrow_rw(): p: {}", p.f);
}

struct Foo {
    f: i32,
}

fn main() {
    let mut owner = Foo { f: 0 };

    let p0 = &mut owner;

    reborrow_rw(p0);

    println!("p0: {}", p0.f);

    let p1 = p0;
    
    // println!("p0: {}", p0.f); // error
}
```

Output:
```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
    Finished dev [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/playrs`
reborrow_rw(): p: 0
p0: 0
```
