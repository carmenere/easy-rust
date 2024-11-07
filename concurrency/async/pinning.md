# Table of contents
- [Table of contents](#table-of-contents)
- [Self-referential types](#self-referential-types)
- [Pinning API contract](#pinning-api-contract)
  - [`Pin<Ptr>`](#pinptr)
  - [`Unpin` and `PhantomPinned`](#unpin-and-phantompinned)
  - [Why pinning is important for futures](#why-pinning-is-important-for-futures)
- [Ways to pin a value](#ways-to-pin-a-value)
  - [Pinning to the heap](#pinning-to-the-heap)
  - [Pinning to the stack](#pinning-to-the-stack)
- [Pin projections](#pin-projections)
  - [Structural pinning](#structural-pinning)
- [Examples](#examples)
  - [Self-referential swap problem](#self-referential-swap-problem)

<br>

# Self-referential types
A **self-referential types** (aka **self-referential structs**) are types that **hold** *pointers* or *references* to **their own data**.<br>

After **self-referential type** is **moved** to another location in memory, it is located at a **new** address, but the value of pointer inside struct is **not** updated and points the **old** location.<br>

There is no way to take a **reference** to `self` in **safe** Rust and store that **reference** in `self`. To do this in **safe** Rust you have to **cast** the **reference** to a **pointer**.
<br>

**Moving** a **self-referential type**:
![Moving a self-referential type](/img/moving_self_referential_struct.png)

Since the *pointer* was **not** updated **after** the instance of **self-referential type** was **moved**, it **still** *points* to the **old** location, which can cause problems.

<br>

# Pinning API contract
The **pointee** is the **value** of some type `T` the *pointer* `Ptr` points to.<br>

**Pinning API contract** provides following **guarantees** for **pointee**:
- *pointee* **will not be moved** out of its memory location **until** its `drop()` handler is called;
- *pointee* **will remain valid** at that same memory location **until** its `drop()` handler is called;

<br>

A **pinned value** is an any **value** that **satisfies pinning API contract**.<br>

<br>

**Pinning API contract** is a part of Rust's **std** and consists of:
- type [**std::pin::Pin**](https://doc.rust-lang.org/std/pin/struct.Pin.html);
- marker trait [**std::marker::Unpin**](https://doc.rust-lang.org/std/marker/trait.Unpin.html);
- marker type [**std::marker::PhantomPinned**](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html);

<br>

## `Pin<Ptr>`
In order to **pin a value** of some type `T`, we **wrap** a *pointer to that value* (of some type `Ptr<T>`) in a `Pin`.<br>
A `Pin<Ptr>` **pins** the *pointer’s pointee value* of type `T`, **not** the `Ptr`.<br>
A `Pin<Ptr>` can wrap any pointer type, forming a promise that the **pointee** will **satisfy** the **pinning API contract**.<br>

<br>

## `Unpin` and `PhantomPinned`
`Unpin` is a **marker trait**. `Unpin` is an **auto trait** (similar to `Send` and `Sync`) which means `Unpin` will be implemented for struct, if all its fields are `Unpin`.<br>

So,
- if a type is `Unpin`, it means it **can** be safely **moved** after it is **pinned**, in other words, **pinning** for `Unpin` types has **no effect**;
- if a type is `!Unpin` (does **not** implement `Unpin`), it **cannot** be **moved** after being **pinned**;

<br>

Since most types in Rust implement `Unpin` **by default**, there is a way to **indicate** that type **does not** implement `Unpin`: a `PhantomPinned` **marker type**.<br>

The **marker type** `PhantomPinned` explicitly implements `!Unpin`:
```rust
impl !Unpin for PhantomPinned {}
```

<br>

By **including** `PhantomPinned` as a **field**, you **ensure** that structs **will not** implement `Unpin`:
```rust
use std::{marker::PhantomPinned, pin::Pin};

#[derive(Default)]
struct MyStruct {
    a: u8,
    b: Option<*const u8>,
    _pin: PhantomPinned,
}
```

<br>

So `!Unpin` type **cannot** be **moved** after it has been **pinned**, in other words the *pinned value* **remains** at the **same** location in memory **until** it gets **dropped**, so long as you stay in **safe** Rust.<br>
Inside the `unsafe` block the pinning API contract guarantees are upheld **by the developer** and **not** the compiler.<br>

<br>

## Why pinning is important for futures
Every `async` *function*/*block* may contain local variables and **references** to them.<br>
Rust transfroms every `async` *function*/*block* into **struct** that implements `Future` trait (aka **future**) which in turn holds all variables of original `async` *function*/*block*. So, compiler-generated *futures* may contain **self-referential pointers** and **moving** them can **invalidate** those **pointers**.<br>

**Pinning** ensures that the data the future relies on is safely anchored in memory, **preventing** UB when the future is moved.<br>

Rust injects `PhantomPinned` into **futures** making them `!Unpin` so that they **cannot** be moved once they have been **pinned**.<br>
That's why the `poll()` method requires the *future* be passed as `Pin<&mut Self>` value.<br>

So, you **cannot** poll *future* **until** you **pin** it, and once you have done that, the *future* **can’t be moved**.<br>

<br>

# Ways to pin a value
The `Pin` type provides 2 methods to **pin values**:
- `Pin::new` for `Unpin` types;
- `Pin::new_unchecked` for `!Unpin` types;

<br>

The `new_unchecked` is **unsafe**:
```rust
impl<Ptr: Deref<Target: Unpin>> Pin<Ptr> {
    pub const fn new(pointer: Ptr) -> Pin<Ptr> {
        // SAFETY: the value pointed to is `Unpin`, and so has no requirements
        // around pinning.
        unsafe { Pin::new_unchecked(pointer) }
    }
}

impl<Ptr: Deref> Pin<Ptr> {
    pub const unsafe fn new_unchecked(pointer: Ptr) -> Pin<Ptr> {
        Pin { __pointer: pointer }
    }
}
```

<br>

The values can be pinned to **heap** or to **stack**:
- **heap-pinned** values are useful when you need the value to **outlive** the current scope or be **shared** across threads.<br>
- **stack-pinned** values are **limited** by the *stack frame's lifetime* and **cannot** be moved out of the current scope.<br>

<br>

## Pinning to the heap
**Pinning to the heap** is usually done by **pinning** a `Box`.<br>
There is `Box::pin(x: T)` method to **pin value**: it **takes ownership** of value of type `T` and returns `Pin<Box<T>>`.<br>
If `T` **does not** implement `Unpin`, then `x` will be **pinned** in memory and **unable** to be **moved**:
```rust
impl<T> Box<T> {
    pub fn pin(x: T) -> Pin<Box<T>> {
        Box::new(x).into()
    }
}
```

<br>

The `Pin<Box<T>>` also **implements** `From<Box<T>>`: it **converts** a `Box<T>` **into** a `Pin<Box<T>>`:
```rust
impl<T: ?Sized> From<Box<T>> for Pin<Box<T>>
{
    fn from(boxed: Box<T>) -> Self {
        Box::into_pin(boxed)
    }

    pub const fn into_pin(boxed: Self) -> Pin<Self>
    {
        unsafe { Pin::new_unchecked(boxed) }
    }
}
```

<br>

**Constructing** and **pinning** of the `Box` can also be done in **two steps**:
```rust
let boxed_value = Box::new(MyStruct { /* fields */ });
let pinned_value = Pin::new(boxed_value);
```

<br>

Both approaches achieve the **same result**: a pinned value on the heap. However, `Box::pin` is more **idiomatic** and concise.<br>

<br>

## Pinning to the stack
Rust's std provides `pin!` macro to **pin** values to **stack**.<br>

<br>

# Pin projections
The **pin projections** are **helper methods** on a **pinned** type. They often look like:
```rust
fn foo(self: Pin<&mut Self>)
```

<br>

## Structural pinning
**Structural pinning** means that **if** a **struct** is **pinned**, the **field** is **pinned** too.<br>
**Structural pinning** is connected to *pin projections*: if you have `Pin<&mut T>` where `T` has field `a` of type `A`, that **can be moved freely** and field `b` of type `B` that **can't be moved**, you must write **2 projections**:
- write a **pin projection** for `a` that returns a **regular reference** to `a`:
```rust
fn a(self: Pin<&mut Self>) -> &A
```
In this case we say that **pinning** is **not** *structural*.<br>
- write a **pin projection** for `b` that returns a **pinned version** of `b`:
```rust
fn b(self: Pin<&mut Self>) -> Pin<&mut B>
```
In this case we say that **pinning** is **structural** for `b` and **field** `b` is **pinned** when the **struct** of `T` is **pinned**.<br>

<br>

# Examples
## Self-referential swap problem
Consider example:
```rust
use std::{marker::PhantomPinned, pin::Pin};

struct Foo {
    a: u8,
    b: *const u8,
    _marker: PhantomPinned,
}

impl Foo {
    fn new(v: u8) -> Self {
        Foo {
            a: v,
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(&mut self) {
        let ptr: *const u8 = &(self.a);
        self.b = ptr;
    }

    fn a(&self) -> &u8 {
        &self.a
    }

    fn b(&self) -> &u8 {
        assert!(!self.b.is_null(), "Foo::b called without Foo::init being called first");
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut foo1: Foo = Foo::new(111);
    foo1.init();

    let mut foo2: Foo = Foo::new(222);
    foo2.init();

    println!("BEFORE moving");
    println!("foo1.a: {}, foo1.b: {}", foo1.a(), foo1.b());
    println!("foo2.a: {}, foo2.b: {}", foo2.a(), foo2.b());

    std::mem::swap(&mut foo1, &mut foo2);

    println!("AFTER moving");
    println!("foo1.a: {}, foo1.b: {}", foo1.a(), foo1.b());
    println!("foo2.a: {}, foo2.b: {}", foo2.a(), foo2.b());
}
```

<br>

If we we swap `t1` with `t2` and thereby **move** the data we get:
```rust
BEFORE moving
foo1.a: 111, foo1.b: 111
foo2.a: 222, foo2.b: 222
AFTER moving
foo1.a: 222, foo1.b: 111
foo2.a: 111, foo2.b: 222
```

<br>

The pointer to `foo1.b` still points to the **old** location which is inside `foo2` **now**.<br>
The pointer to `foo2.b` still points to the **old** location which is inside `foo1` **now**.<br>

<br>

**Self-referential swap problem**:
![Self-referential swap problem](/img/self_referential_swap_problem.png)

<br>

Let's **pin values**. Add following lines before `std::mem::swap`:
```rust
let mut foo1_pin = unsafe { Pin::new_unchecked(&mut foo1) };
let mut foo2_pin = unsafe { Pin::new_unchecked(&mut foo2) };
```

<br>

The code **still compiles** and more over we get the **same output**:
```rust
BEFORE moving
foo1.a: 111, foo1.b: 111
foo2.a: 222, foo2.b: 222
AFTER moving
foo1.a: 222, foo1.b: 111
foo2.a: 111, foo2.b: 222
```

<br>

**Why**? The compiler **does not** allow you to **move** the values, if you **move** them **through** the `Pin` wrapper **only**.<br>
If you can **access** your values **directly**, **without** using `Pin`, there is nothing compiler can do for you. That’s why, **pinning** `!Unpin` is **unsafe**.<br>
You (as the developer) must hold a **promise**, which is: I will **not move** the **pinned values** through **other ways**.<br>
In our example `foo1` and `foo2` are **accessed directly** in `std::mem::swap` function.<br>

<br>

To access the values **only** through `Pin` we must **shadow** the variable names with variables of `Pin` type, in order to **limit ourselves**.<br>

Add following fix to example above:
```rust
let mut foo1 = unsafe { Pin::new_unchecked(&mut foo1) };
let mut foo2 = unsafe { Pin::new_unchecked(&mut foo2) };
```

In the fix above we assign returned value from `Pin::new_unchecked` to variables `foo1` and `foo2`.<br>
But the code **still compiles**, but the **output** is **changed**:
```rust
BEFORE moving
foo1.a: 111, foo1.b: 111
foo2.a: 222, foo2.b: 222
AFTER moving
foo1.a: 222, foo1.b: 222
foo2.a: 111, foo2.b: 111
```

<br>

What we have swapped **were not** our **structs**, **but** the **pointers**.<br>
So now, **pin pointer** `foo1` is pointing towards **value** `foo2` and **pin pointer** `foo2` is pointing towards **value** `foo1`.<br>
This is **ok**, since the **values** of type `Foo` have **not** moved.<br>

<br>

How can we try to move the actual `foo1` and `foo2` values through the **pin pointer**? We can pass a **mutable references** to **values** to `swap`:
```rust
std::mem::swap(foo1.get_mut(), foo2.get_mut());
```

<br>

The **final example**:
```rust
use std::{marker::PhantomPinned, pin::Pin};

struct Foo {
    a: u8,
    b: *const u8,
    _marker: PhantomPinned,
}

impl Foo {
    fn new(v: u8) -> Self {
        Foo {
            a: v,
            b: std::ptr::null(),
            _marker: PhantomPinned, // This makes our type `!Unpin`
        }
    }

    fn init(&mut self) {
        let ptr: *const u8 = &(self.a);
        self.b = ptr;
    }

    fn a(&self) -> &u8 {
        &self.a
    }

    fn b(&self) -> &u8 {
        assert!(!self.b.is_null(), "Foo::b called without Foo::init being called first");
        unsafe { &*(self.b) }
    }
}

fn main() {
    let mut foo1: Foo = Foo::new(111);
    foo1.init();

    let mut foo2: Foo = Foo::new(222);
    foo2.init();

    println!("BEFORE moving");
    println!("foo1.a: {}, foo1.b: {}", foo1.a(), foo1.b());
    println!("foo2.a: {}, foo2.b: {}", foo2.a(), foo2.b());

    let mut foo1 = unsafe { Pin::new_unchecked(&mut foo1) };
    let mut foo2 = unsafe { Pin::new_unchecked(&mut foo2) };

    std::mem::swap(foo1.get_mut(), foo2.get_mut());

    println!("AFTER moving");
    println!("foo1.a: {}, foo1.b: {}", foo1.a(), foo1.b());
    println!("foo2.a: {}, foo2.b: {}", foo2.a(), foo2.b());
}
```

**This code doesn't compile**. And compiler prints: `error[E0277]: PhantomPinned cannot be unpinned`.<br>