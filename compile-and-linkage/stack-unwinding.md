# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Stack unwinding](#stack-unwinding)
- [Stack unwinding in Rust](#stack-unwinding-in-rust)
  - [`std::panic::catch_unwind`](#stdpaniccatch_unwind)
  - [Setting panic hooks](#setting-panic-hooks)
  - [Viewing backtraces](#viewing-backtraces)
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

# Stack unwinding in Rust
In Rust, `panic!()` macro is the standard way to handle **unrecoverable errors** during normal execution. **By default**, `panic!` starts **stack unwinding**, calling destructors as it goes up the stack.<br>

<br>

There are **2 panic strategies** in Rust:
- **`unwind`** (*by default*):
  - when a **panic occurs**, a **panic hook function** is called;
    - in `#![no_std]` application you must set **custom panic handler** with the `#[panic_handler]` attribute;
      - the `#[panic_handler]` attribute must be applied to a function with signature `fn(&PanicInfo) -> !` and such function must appear **once** *in the dependency graph* of a crate;
  - **cleans up** resources by **unwinding** the **current thread's stack** and then **terminates** the *current thread*;
    - if the *stack unwinding reaches* the **main thread**, the **whole process** is **terminated**;
  - **allows panic recovery** with `catch_unwind`;
- **`abort`**:
  - **immediately terminates** the *process* **without** cleaning up memory or resources;

So, **`abort`** is **faster** and produces **smaller binaries**, but **skips cleanup**.<br>

<br>

**Pros&Cons**:
- **`abort`**:
  - **pros**:
    - smaller binary;
    - faster panic exit;
  - **cons**:
    - **cannot** catch panic at runtime;
- **`unwind`**:
  - **pros**:
    - allows **panic recovery** with `catch_unwind`;
    - but **panic during a panic** leads to `std::process::abort()` anyway;
  - **cons**:
    - slight performance overhead;
    - larger binary;

<br>

Both [**`-C panic`**](https://doc.rust-lang.org/cargo/reference/profiles.html#panic) **flag** and [**`panic`**](https://doc.rust-lang.org/cargo/reference/profiles.html#panic) setting in `Cargo.toml` control which **panic strategy** to use.<br>

<br>

The valid options for **`-C panic`** **flag** are:
- `abort`: **terminate the process** upon panic;
- `immediate-abort`: **terminate the process** upon panic, and **do not call** *any panic hooks*;
- `unwind`: **unwind the stack** upon panic;

<br>

The valid options for **`panic`** setting in `Cargo.toml`
- `abort`: **terminate the process** upon panic;
- `unwind`: **unwind the stack** upon panic;

<br>

**Example** (`Cargo.toml`):
```toml
[profile.release]
panic = 'abort'
```

<br>

## `std::panic::catch_unwind`
A Rust panic is **not** always implemented **via unwinding**, but can be implemented **by aborting** the process as well. This function **only catches unwinding panics**, **not** those that abort the process.<br>
**Note**, that **panic during a panic** leads to `std::process::abort()` anyway.<br>

**Example**:
```rust
fn main() {
    let r: Result<!, Box<dyn Any + Send + 'static>> = std::panic::catch_unwind(|| {
        panic!("oh no!");
    });
    println!("Life after panic.");
}
```
**Output**:
```bash
thread 'main' (883349) panicked at example/src/main.rs:3:9:
oh no!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Life after panic.
```

<br>

## Setting panic hooks
When a panic happens, it first tells us **which thread** panicked, the **location in the code** where the panic happened, **why** it panicked (detailed **error message**) and a **note** about how to display a backtrace.<br>

**Default panic message**:
```rust
fn main() {
    panic!();
}
```

**Output**:
```bash
thread 'main' (885163) panicked at example/src/main.rs:2:5:
explicit panic
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<br>

**Custom panic message**:
```rust
fn main() {
    panic!("custom message");
}
```
**Output**:
```bash
thread 'main' (886499) panicked at example/src/main.rs:2:5:
custom message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<br>

```rust
fn main() {
    None::<u64>.unwrap();
}
```
**Output**:
```bash
thread 'main' (892904) panicked at example/src/main.rs:2:17:
called `Option::unwrap()` on a `None` value
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<br>

But we can change all of this if we want by using a method called `set_hook()`. This sets up a **global panic hook**, which will be called instead of the default panic hook. When setting a panic hook, **avoiding panics** is especially important because **panic during a panic** will lead to `std::process::abort()`.<br>
**Note**, if a custom panic hook has been set, it will be invoked **before** the panic is caught, **before** unwinding.<br>

<br>

**Example**:
```rust
fn main() {
    std::panic::set_hook(Box::new(|_| {
        println!("Foo");
        println!("Bar");
    }));
    panic!();
}
```
**Output**:
```bash
Foo
Bar
```

<br>

A closure inside `set_hook()` receives `&std::panic::PanicHookInfo`, which implements both `Debug` and `Display`:
```rust
fn main() {
    std::panic::set_hook(Box::new(|info: &std::panic::PanicHookInfo<'_>| {
        println!("info = {info}");
    }));
    panic!();
}
```
**Output**:
```bash
info = panicked at example/src/main.rs:5:5:
explicit panic
```

<br>

The `PanicHookInfo` struct provides **information about a panic**:
```rust
pub struct PanicHookInfo<'a> {
    payload: &'a (dyn Any + Send),
    location: &'a Location<'a>,
    can_unwind: bool,
    force_no_backtrace: bool,
}
```

<br>

**Methods**:
- `location()`
  - **returns** information about the **location** from which the panic originated, if available;
- `payload()`
  - **returns** the **payload** associated with the panic;
  - this will commonly, but not always, be a `&'static str` or `String`;
- `payload_as_str()`
  - **returns** the **payload** **if** it is of type `&'static` str or `String`;
  - it downcasts payload to `&str` or to `String`:
the `payload_as_str`:
```rust
  pub fn payload_as_str(&self) -> Option<&str> {
      if let Some(s) = self.payload.downcast_ref::<&str>() {
          Some(s)
      } else if let Some(s) = self.payload.downcast_ref::<String>() {
          Some(s)
      } else {
          None
      }
  }
```

<br>

The `std::panic::take_hook` **unregisters** the **current** panic hook and **returns it** and registers the **default** hook. If the *default* hook is **registered** it will be returned, but remain registered.<br>

<br>

## Viewing backtraces
To view a **backtrace** just use a function called `Backtrace::capture()`, which is located in the `std::backtrace` module. The `Backtrace::capture()` looks for either `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` environment variable set to `1` or `full`.<br>

The `Backtrace` struct also has a method called `status()` that returns an **enum** called a `BacktraceStatus`. It has the `#[non_exhaustive]` attribute which means that you have to add `_ => ...` variant **after all variants** listed inside the `match`. The `Disabled` variant means neither `RUST_BACKTRACE` nor `RUST_LIB_BACKTRACE` set to `1` or `full`.

**Example**:
```rust
use std::backtrace::Backtrace;

fn foo() {
    use std::backtrace::BacktraceStatus::*;
    let bt: Backtrace = Backtrace::capture();
    match bt.status() {
        Unsupported => println!("Current architecture doesn't support backtraces."),
        Disabled => println!("Backtrace isn't enabled."),
        Captured => println!("Captured bactrace:\n{}", bt),
        _ => todo!(),
    }
}

fn main() {
    unsafe {std::env::set_var("RUST_BACKTRACE", "full")};
    foo();
}
```
**Output**:
```bash
Captured bactrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2: std::backtrace::Backtrace::create
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/backtrace.rs:331:13
   3: example::foo
             at ./example/src/main.rs:5:25
   4: example::main
             at ./example/src/main.rs:16:5
   5: core::ops::function::FnOnce::call_once
             at /Users/an.romanov/.rustup/toolchains/1.94.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   6: std::sys::backtrace::__rust_begin_short_backtrace
             at /Users/an.romanov/.rustup/toolchains/1.94.0-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:166:18
   7: std::rt::lang_start::{{closure}}
   8: core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/core/src/ops/function.rs:287:21
   9: std::panicking::catch_unwind::do_call
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panicking.rs:581:40
  10: std::panicking::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panicking.rs:544:19
  11: std::panic::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panic.rs:359:14
  12: std::rt::lang_start_internal::{{closure}}
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/rt.rs:175:24
  13: std::panicking::catch_unwind::do_call
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panicking.rs:581:40
  14: std::panicking::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panicking.rs:544:19
  15: std::panic::catch_unwind
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/panic.rs:359:14
  16: std::rt::lang_start_internal
             at /rustc/4a4ef493e3a1488c6e321570238084b38948f6db/library/std/src/rt.rs:171:5
  17: _main
```

<br>

So, in **custom panic handler**, we can **capture** and **print** **backtrace**.<br>

<br>
