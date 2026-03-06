# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Const context](#const-context)
- [Const functions](#const-functions)
- [Constants](#constants)
  - [Examples](#examples)
- [Static](#static)
  - [Examples](#examples-1)
- [`const` vs. `static`](#const-vs-static)
- [`~const`](#const)
- [Const generics](#const-generics)
<!-- TOC -->

<br>

# Const context
A **const context** refers to a specific place in the code where expressions **must be evaluated entirely at compile time**.<br>
A **const context** is one of the following:
- array **type** length expressions;
- array **repeat** length expressions;
- the **initializer of**;
    - **constants**;
      - `const X: u32 = ...`
    - **statics**;
      - `static Y: u32 = ...`
    - **enum discriminants**;
- a **const generic argument**;
  - `struct Buffers<const N: usize>`
- a **const block**;

<br>

Any code within a **const context** is **guaranteed** to be executed **during compilation**, or it will produce a compiler error.<br>

<br>

An expression that **can** be called from a *const context* is called **const expression**.<br>
A function that **can** be called from a *const context* is called **const function**.<br>

**Not all** functions are `const`: **not all** things are **allowed** in a *const context*.<br>

Operations that require *runtime-specific features* are **disallowed** in a **const context**:
- **heap allocations**, e.g. `Vec::new()`;
  - but **empty** vectors are **allowed** in **const context**;
- **I/O**;
- **mutable** `static` variables;

<br>

# Const functions
A **const function** is defined with the `const` keyword: `const fn`. The **body** of a *const function* may only use *const expressions*.<br>
The types of a *const function’s* parameters and return type are **restricted** to those that are **compatible** with a *const context*.<br>

So, *const function* can be called either from a *const context* or from **outside** a *const context*:
- when `const fn` is called from a *const context*, it is **evaluated** by the compiler **at compile time**;
- when `const fn` is called from **outside** a *const context*, it **behaves like** a function **without** `const` keyword;

<br>

**Example**:
```rust
const fn get_size() -> usize {
    // This is a const fn, so it can be used in a const context
    42
}

const BUFFER_SIZE: usize = get_size(); // This is a const context

fn main() {
    let buffer: [u8; BUFFER_SIZE]; // This is a const context too: array size must be known at compile time
    // ...
}
```

<br>

# Constants
**Constant** is **not** variable or place in memory, a **constant** is an **unchangeable value** that is **evaluated at compile time**.<br>

**Properties**:
- **uppercase** by convention;
- data type is **mandatory**;
- values can not be changed;
- **global** or **local** scope;

<br>

## Examples
```Rust
const URL: &str = "google.com";
```

<br>

# Static
**Static variables** are **global variables** with following properties:
- *static variables* **must** have **static lifetime**.
- *static variables* can be **mutable** or **immutable**.
- *static variables* have **fixed address** in the memory.
- **mutable** *static variables* can only be **read** and **modified** inside `unsafe { }` **block**;
- *static variables* **must be thread-safe** (implement the `Sync` trait) to prevent data races, as the compiler assumes they might be accessed from multiple threads
  - **note**, it is **not possible** to use `std::cell::OnceCell` for a *static variable* in a multi-threaded context because `OnceCell` is **not thread-safe**;

<br>

## Examples
```Rust
static mut COUNTER: u64 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

fn main () {
    // access to modify static variable
    unsafe {
        increment();
    }

    // access to read static variable
    unsafe {
        println!("Counter is {}.", COUNTER);
    }
}
```

<br>

# `const` vs. `static`
`const`:
- **has no fixed address in memory**;
- value is **inlined** to each place where it is used;
- **faster** runtime;
- **bigger** executable file;

<br>

`static`:
- **has fixed address in memory**;
- value is **loaded from memory**;
- **slower** runtime because we need load data from memory;
- **smaller** executable file;

<br>

# `~const`
The primary purpose of `~const` is to allow for **generic functions** to be run in a *const context*.<br>

Consider a trait `MyTrait` with a method foo:
```rust
trait MyTrait {
    fn foo(&self);
}
```

You would then have two potential implementations: one for general runtime use, and one that is explicitly marked as usable in a const context:
- **implementation that can be used at compile time**:
```rust
struct A;

impl const MyTrait for A {
    fn foo(&self) { /* is is a const function! */}
}
```
- **regular runtime implementation**:
```rust
struct B;

impl MyTrait for B {
    fn foo(&self) { /* is is NOT a const function */}
}
```

Then you can to create a generic function `test` that **can be called** in a *const context* (*compile time*) if `MyTrait` is implemented with `impl const ...` for a given type `X`:
```rust
const fn test<X>(x: &X)
where
    X: ~const MyTrait
{
    x.foo();
}
```

Here, `X: ~const MyTrait` means the type `X` must implement `MyTrait` in a way that is compatible with *const context*.<br>

Now a call to `test(&A)` is **allowed** in a *const context* while a call to `test(&B)` is **not** allowed:
```rust
static THE_A: () = test(&A); // OK
static THE_B: () = test(&B); // ❌ ERROR: B does not implement MyTrait as `impl const`
```

<br>

So,
- when `test` is called with type `A` within a *const context*, the compiler uses the `impl const MyTrait for A` version and evaluates it at compile time;
- when `test` is called with type `B` or type `A` in a **runtime context**, it uses the **regular implementation**;

<br>

# Const generics
From the compiler point of view type `[i32; 3]` is **not** the same type as `[i32; 4]`.<br>

Consider example:
```rust
struct Buffers {
    array_one: [u8; 640],
    array_two: [u8; 640]
}

struct BigBuffers {
    array_one: [u8; 1280],
    array_two: [u8; 1280]
}
```

If we want to implement a trait like `Display` we would have to implement the trait **for each struct**.<br>
We need a **different struct for each specific array size**, and **each struct need to implement the traits**.<br>

**Const generics** solves this problem:
```rust
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N]
}
```

<br>

Now, we only need a **single struct**: the **type** `T` is **fixed**, but the **number** is **not**: the value `N` **can be any number**.<br>

Example:
```rust
#[derive(Debug)]
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N],
}

fn main() {
    let buffer_1 = Buffers {
        array_one: [0u8; 1],
        array_two: [0; 1],
    };
    let buffer_2 = Buffers {
        array_one: [0i32; 2],
        array_two: [10; 2],
    };
    println!("{buffer_1:#?}\n{buffer_2:#?}");
}
```

**Output**:
```rust
Buffers {
    array_one: [
        0,
    ],
    array_two: [
        0,
    ],
}
Buffers {
    array_one: [
        0,
        0,
    ],
    array_two: [
        10,
        10,
    ],
}
```
