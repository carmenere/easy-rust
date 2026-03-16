# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Stack unwinding](#stack-unwinding)
- [`panic!`](#panic)
- [Stack unwinding in Rust](#stack-unwinding-in-rust)
<!-- TOC -->

<br>

# Stack unwinding
**Stack unwinding** refers to the process of *deallocating resources* and *cleaning up the call stack* when an **exception is thrown** but **not** caught within a current function.<br>
**Stack unwinding** invokes **cleanup code** at each *stack frame* as it goes up the stack. All **objects** that were created on the *stack frame* are **destroyed**. **Destructors** are called in **reverse order** of construction. This guarantees cleanup of stack-based resources.<br>

**Stack unwinding** happens:
- **normally**, when a function returns;
- during **exception handling**;
- at **panics**;

<br>

**Stack unwinding** steps:
- when an *exception is thrown*, the **runtime** begins a search for a **matching catch block** *to handle the exception*;
  - the control moves *from* the **throw statement** *to* the **first catch statement** that can handle the *thrown exception*;
- the **runtime** moves up the call stack *frame by frame* searching for a *matching catch block*;
- if **no** *matching catch block* is **found** within the *current function*, the function’s **stack frame** is **unwound**;
  - if during stack unwinding a **destructor** *throws an exception* and that exception is **not** handled the program is **terminated**;
- this process continues recursively **until** an appropriate *catch block* is found or **until** the top-level function is reached;
  - if an **exception is not caught** (**no** handler is **found** and the top-level function is **reached**) then **terminates** the program is **terminated**;
  - if the **catch statement** is **reached**, all of the variables that are in scope between the **throw** and **catch statements** are destroyed;

<br>

# `panic!`
In Rust, `panic!` macro is the standard way to handle **unrecoverable errors** during normal execution, but it is **not** a direct terminate the program function.<br>
**By default**, `panic!` starts **stack unwinding**, running destructors as it goes up the stack. <br>

In programs compiled with `panic=abort`, `panic!` behaves more like `std::process::abort()`.

<br>

# Stack unwinding in Rust
There are **2 panic strategies** in Rust:
- **unwind**, *by default*;
  - when a **panic occurs**, a **panic hook function** is called;
    - in a **no-std** crate, you'll need to set your own panic handler, use the `#[panic_handler]` attribute;
  - **terminates** the *current thread* **by unwinding the stack** allowing the rest of the program **continue execution**;
    - **but**, if the *stack unwinding reaches* the **main thread**, the **whole program** is **terminated**.<br>
- **abort**
  - **immediately terminates** the process **without** cleaning up memory or resources;
  - when `panic = 'abort'`, then `panic!` behaves like `std::process::abort()`;

<br>

The *panic strategy* can be set in the `Cargo.toml`:
```toml
[profile.release]
panic = 'abort'
```
