# Table of contents
- [Table of contents](#table-of-contents)
- [Self-referential structs](#self-referential-structs)
- [Pinning](#pinning)
  - [Pin projections](#pin-projections)
  - [Structural pinning](#structural-pinning)
- [Ways to pin a value](#ways-to-pin-a-value)
  - [Pinning to the heap](#pinning-to-the-heap)
  - [Pinning to the stack](#pinning-to-the-stack)
- [The connection with futures](#the-connection-with-futures)

<br>

# Self-referential structs
There is no way to take a **reference** to `self` in Rust and store that **reference** in `self`. To do this in **safe** Rust you have to **cast** the **reference** to a **pointer**.<br>
A **self-referential struct** (aka **self-referential type**) is a struct that stores **pointer** to `self` in at least one of its field.<br>

After **self-referential type** is **moved** to another location in memory, the `self` is located at a **new** address, but the value of pointer inside struct is **not** updated and **points** the **old** location.<br>

<br>

**Moving** a **self-referential type**:
![Moving a self-referential type](/img/moving_self_referential_struct.png)

<br>

Since the *pointer* was **not** updated **after** the instance of **self-referential type** was **moved**, it **still** *points* to the **old** location, which can cause problems.

<br>

# Pinning
**Pinning** **prevents from moving** *self-referential types*.<br>

Pinning is a part of Rust's std and consists of 2 parts:
- **type** [**std::pin::Pin**](https://doc.rust-lang.org/std/pin/struct.Pin.html):
  - it **gives a guarantee** that the **pinned** value **doesn't** change its **address** in memory while using **safe** Rust;
  - it allows us to use `unsafe` for operations that can lead to problems;
  - it allows us to write **self-referential types** that are **safe**;
- **marker trait** [**std::marker::Unpin**](https://doc.rust-lang.org/std/marker/trait.Unpin.html):
  - if type implements `Unpin`, **pinning** values of such type will **have no effect**;

<br>

To force **pinning** values of some type **to have effect** you must implement `!Unpin` for type:
  - to mark type as `!Unpin` you must add **marker trait** [**std::marker::PhantomPinned**](https://doc.rust-lang.org/std/marker/struct.PhantomPinned.html) to this type;

<br>

The type `PhantomPinned` explicitly implements `!Unpin`:
```rust
impl !Unpin for PhantomPinned {}
```

<br>

So, implementing `!Unpin` **guarantees** that the **pinned value** of such type will **no longer move**, in other words the **pinned value** **remains** at the **same** location in memory **until** it gets **dropped**, **so long as** you **stay** in **safe** Rust.<br>

<br>

## Pin projections
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

# Ways to pin a value
## Pinning to the heap
**Pinning to the heap** is usually done by pinning a `Box`:
- there is `Box::pin(value: T)` constructor to **pin value**: it **takes ownership** of value of type `T` and returns `Pin<Box<T>>`;
- the `Pin<Box<T>>` implements `From<Box<T>>`, so `Pin::from(value: T)` **takes ownership** of value of type `T` and returns `Pin<Box<T>>`;

<br>

> **Note**:<br>
> **Pinning to the heap** is the **preffered way** to **pin values** in Rust.<br>

<br>

Consider example:
```rust
use std::{marker::PhantomPinned, pin::Pin};

#[derive(Default)]
struct SelfRef {
    a: u8,
    b: Option<*const u8>,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn init(self: Pin<&mut Self>) {
        unsafe {
            let Self{a, b, .. } = self.get_unchecked_mut();
            *a = 10;
            *b = Some(a);
        }
    }

    fn b(self: Pin<&mut Self>) -> Option<&mut u8> {
        unsafe {
            self.get_unchecked_mut().b.map(|mut b| &mut *(b as *mut u8))
        }
    }
}

fn main() {
    let mut pinned: Pin<Box<SelfRef>> = Box::pin(SelfRef::default());

    println!("{}", pinned.as_ref().a);

    pinned.as_mut().init();

    println!("{}", pinned.as_ref().a);
    
    *pinned.as_mut().b().unwrap() = 20;
    println!("{}", pinned.as_ref().a);
}
```

<br>

## Pinning to the stack
**Pinning to the stack** is **harder** since we **pin** *by taking* `&mut T`, and we **must** to **guarantee** that we will **not** *move* `T` **until** it has **dropped**.<br>
Rust **can't** help us here, so **it's up to us** to uphold that guarantee.<br>

This is why **stack pinning** is `unsafe`.<br>

For this reason the Rust's std provides `pin!` macro that helps us with **safe** *stack pinning*.<br>

```rust
use std::{marker::PhantomPinned, pin::Pin};

#[derive(Default)]
struct SelfRef {
    a: u8,
    b: Option<*const u8>,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn init(self: Pin<&mut Self>) {
        unsafe {
            let Self{a, b, .. } = self.get_unchecked_mut();
            *a = 10;
            *b = Some(a);
        }
    }

    fn b(self: Pin<&mut Self>) -> Option<&mut u8> {
        unsafe {
            self.get_unchecked_mut().b.map(|mut b| &mut *(b as *mut u8))
        }
    }
}

fn stack_pinning() {
    let mut x = SelfRef::default();

    let mut pinned = unsafe {
        Pin::new_unchecked(&mut x)
    };

    println!("{}", pinned.as_ref().a);

    pinned.as_mut().init();
    println!("{}", pinned.as_ref().a);

    *pinned.as_mut().b().unwrap() = 20;
    println!("{}", pinned.as_ref().a);
}

fn main() {
    stack_pinning()
}
```

<br>

# The connection with futures
The `poll()` method requires the *future* be passed as `Pin<&mut Self>` value.<br>
So, you **cannot** poll *future* **until** you have constructed a `Pin` wrapper for it, and once you have done that, the *future* **can’t be moved**.<br>

Rust marks **futures** as `!Unpin` types so that they **cannot** be moved once they have been **pinned**.<br>