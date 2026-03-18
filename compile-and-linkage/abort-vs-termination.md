# `std::sys` and `libc`
The **PAL** (**platform abstraction layer**) contains **platform-specific abstractions**.<br>

- [**`libc`**](https://docs.rs/libc/latest/src/libc/lib.rs.html)
- [**`std::sys`**](https://doc.rust-lang.org/stable/src/std/sys/mod.rs.html)
  - [**`std::sys::process`**](https://doc.rust-lang.org/stable/src/std/sys/process/mod.rs.html)
    - [**`std::sys::process::unix`**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/mod.rs.html)
      - [**`std::sys::process::unix::unix`**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/unix.rs.html)
      - [**`std::sys::process::unix::common`**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/common.rs.html)
  - [**`std::sys::pal`**](https://doc.rust-lang.org/stable/src/std/sys/pal/mod.rs.html)
    - [**`std::sys::pal::unix`**](https://doc.rust-lang.org/stable/src/std/sys/pal/unix/mod.rs.html)
      - [**`std::sys::pal::unix::os`**](https://doc.rust-lang.org/stable/src/std/sys/pal/unix/os.rs.html)

<br>

## imp
An `imp` common module **naming convention**.<br>
The name `imp` is frequently used as a *naming convention* for a private module within a larger crate (especially the standard library).<br>
This module handles the actual, often **platform-specific** *implementation details*, which are then exposed through a public interface.<br>

**Example** `std::sys::process` as `imp`:
```rust
use crate::sys::{AsInner, AsInnerMut, FromInner, IntoInner, process as imp};
```

<br>

# Program termination
## trait `Termination`
A trait [**`Termination`**](https://doc.rust-lang.org/beta/std/process/trait.Termination.html) is for implementing arbitrary return types in the `main` function.:
```rust
pub trait Termination {
    fn report(self) -> ExitCode;
}
```

<br>

The **C-main function** only supports returning **integers**. So, every type implementing the `Termination` trait has to be converted to an integer.<br>
The **standard library** provides the **canonical** `SUCCESS` and `FAILURE` **exit codes**. They both depend on platform. For example, in [**std::sys::process::unix::common**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/common.rs.html) you can see that **exit codes for unix** defined in `use libc::{EXIT_FAILURE, EXIT_SUCCESS}` constants:
- the `libc::EXIT_SUCCESS` to indicate a **successful execution**;
- in case of a **failure**, `libc::EXIT_FAILURE` is returned;

<br>

The `Infallible`, **never type** `!`, `()`, `ExitCode` and `Result<T: Termination, E: Debug` all implement `Termination`:
- `impl Termination for Infallible`
- `impl Termination for !`
- `impl Termination for ()`
- `impl Termination for ExitCode`
- `impl<T: Termination, E: Debug> Termination for Result<T, E>`

<br>

The `ExitCode` can be returned from the `main()` function, as it implements `Termination`:
```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    if !check_foo() {
        return ExitCode::from(42);
    }

    ExitCode::SUCCESS
}
```

<br>

## `std::process::exit()` vs. `std::process::abort()`
In idiomatic Rust, **simply returning** from the `main()` function is the **preferred** method for **clean termination**, as it **automatically runs destructors** and allows the **proper exit code** to be returned.<br>

Both `std::process::exit()` and `std::process::abort()` are for exceptional, non-recoverable scenarios:
- use `std::process::exit()` when you need to **terminate** the program at an **unexpected point** (**not** via returning from `main()`) but the **program state is still stable enough** that you don't risk corrupting data during minimal cleanup;
- use `std::process::abort()` when the program is in a severely corrupted or **inconsistent state** where **attempting any form of cleanup** (even minimal I/O flushing) *might cause* **unpredictable errors** or **security issues**;


<br>

## `std::process::abort()`
Ultimately the `libc::abort()` is called:
- `std::process::abort()`
  - `std::sys::abort_internal()`
    - `unsafe { libc::abort() }`
      - *C standard library's* [**`abort()`**](https://man7.org/linux/man-pages/man3/abort.3.html) (on Unix);

<br>

The **`abort()`** function causes **abnormal** *process termination* by raising the `SIGABRT` signal. The shell prints **Aborted** for aborted process. The **default disposition** for `SIGABRT` is `Core`, which means **terminate the process** and **dump core**. So, the primary reason to use `std::process::abort()` over other exit methods (like `std::process::exit()`) is to **stop immediately** and generate a **core dump** for post-mortem analysis.<br>
**Note** that the functions registered with `atexit(3)` and `on_exit(3)` are **not called**.<br>

<br>

The **`abort()`** function first **unblocks** the `SIGABRT` signal, and then **raises** that **signal** `SIGABRT` for the calling process:
- if the `SIGABRT` signal is **ignored**, or **caught by a handler that returns**, the **`abort()`** function **terminates** the process;
  - it does this by **restoring** the *default disposition* for `SIGABRT` and then **raising** the signal for a **second time**;
- if the `SIGABRT` signal is **caught by a handler that does not return**, the **`abort()`** function **doesn't terminate** the process;
  - because if the *signal handler* **does not return**, then the **final step is not performed**.<br>

<br>

The function has the signature `pub fn std::process::abort() -> !`, which indicates that it **never returns to the caller**.<br>

<br>

## `std::process::exit()`
Ultimately the `libc::exit()` is called:
- `std::process::exit(code)`
  - `std::sys::os::exit(code)`
    - `unsafe {  libc::exit(code) }`
      - *C standard library's* [**`exit()`**](https://man7.org/linux/man-pages/man3/exit.3.html) (on Unix);

<br>

The **`exit()`** function causes **normal** *process termination* and the least significant byte of status is returned to the parent.<br>
**All functions** registered with `atexit(3)` and `on_exit(3)` are **called**, in the **reverse order** of their registration.<br>
**All open** `stdio(3)` **buffers** are **flushed** and **closed**.  **Files** created by `tmpfile(3)` are **removed**.<br>

The **C standard** specifies two constants, `EXIT_SUCCESS` and `EXIT_FAILURE`, that may be passed to `exit()` to indicate **successful** or **unsuccessful** termination, respectively.<br>

<br>

The [`std::process::exit()`](https://doc.rust-lang.org/std/process/fn.exit.html) **terminates** the *current process* with the specified **exit code**.<br>
This function will **never return** and will **immediately terminate the current process**. The **exit code** will be available for consumption by another process.<br>

Note that because this function never returns, and that it terminates the process, no destructors on the current stack or any other thread’s stack will be run.<br>
If a **clean shutdown is needed** it is **recommended** simply return a type implementing `Termination` (such as `ExitCode` or `Result`) from the `main` function and avoid this function altogether.<br>



In Rust, `std::process::exit()` is part of the **stable public API** for **terminating a process**, while `std::sys::os::exit` is an internal, **unstable** function that is part of the standard library's private implementation details.<br>

The definition of `std::process::exit()`:
```rust
pub fn exit(code: i32) -> ! {
    crate::rt::cleanup();
    crate::sys::os::exit(code)
}
```

<br>

The `std::sys::os::exit()` is implemented inside [**std::sys::pal::unix::os**](https://doc.rust-lang.org/stable/src/std/sys/pal/unix/os.rs.html):
```rust
pub fn exit(code: i32) -> ! {
    crate::sys::exit_guard::unique_thread_exit();
    unsafe { libc::exit(code as c_int) }
}
```

<br>

# ExitCode vs. ExitStatus
There are 2 types:
- struct [**ExitCode**](https://doc.rust-lang.org/beta/std/process/struct.ExitCode.html)
- struct [**ExitStatus**](https://doc.rust-lang.org/beta/std/process/struct.ExitStatus.html)

<br>

The type `std::process::ExitCode` :
- represents the **status code** the current process can return to its parent under **normal termination**;
- `ExitCode` implements `From<u8> for ExitCode` for constructing other arbitrary exit codes;

<br>

The **disposition of a process** in an operating system refers to the **final stage** of its lifecycle (terminated/exit) or its **signal handling behavior** (how it responds to signals like kill, stop, or core dump).<br>

<br>

The type `std::process::ExitStatus` :
- an `ExitStatus` represents **every possible disposition of a process**;
  - on Unix this is the **wait status**. It is **not** simply an *exit status* (a value passed to `exit()`);
- this type is used to represent the **exit status** of a **child process**;
- child processes are created via the `Command` struct and their **exit status** is exposed through the `status` method, or the `wait` method of a `Child` process;

<br>

Both `ExitStatus` and `ExitCode` are defined in [**std::process**](https://doc.rust-lang.org/src/std/process.rs.html):
```rust
use crate::sys::process as imp;

pub struct ExitStatus(imp::ExitStatus);
pub struct ExitCode(imp::ExitCode);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(imp::ExitCode::SUCCESS);
    pub const FAILURE: ExitCode = ExitCode(imp::ExitCode::FAILURE);
}

impl From<u8> for ExitCode {
    /// Constructs an `ExitCode` from an arbitrary u8 value.
    fn from(code: u8) -> Self {
        ExitCode(imp::ExitCode::from(code))
    }
}
```

<br>

## Low level details
- [**libc**](https://docs.rs/libc/latest/src/libc/lib.rs.html)
- [**std::sys**](https://doc.rust-lang.org/stable/src/std/sys/mod.rs.html)
- [**std::sys::process**](https://doc.rust-lang.org/stable/src/std/sys/process/mod.rs.html)
```rust
cfg_select! {
    target_family = "unix" => {
        mod unix;
        use unix as imp;
    }
    target_os = "windows" => {
        mod windows;
        use windows as imp;
    }
    ...
    _ => {
        mod unsupported;
        use unsupported as imp;
    }
}
```
- [**std::sys::process::unix**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/mod.rs.html)
```rust
mod common;
cfg_select! {
    ...
    _ => {
        mod unix;
        use unix as imp;
    }
}
pub use imp::{ExitStatus, Process};
pub use self::common::ExitCode;
```
- [**std::sys::process::unix::unix**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/unix.rs.html)
```rust
/// Unix exit statuses
//
// This is not actually an "exit status" in Unix terminology.  Rather, it is a "wait status".
#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct ExitStatus(c_int);
```
- [**std::sys::process::unix::common**](https://doc.rust-lang.org/stable/src/std/sys/process/unix/common.rs.html)
```rust
use libc::{EXIT_FAILURE, EXIT_SUCCESS, c_int, gid_t, pid_t, uid_t};

pub struct ExitCode(u8);

impl ExitCode {
    pub const SUCCESS: ExitCode = ExitCode(EXIT_SUCCESS as _);
    pub const FAILURE: ExitCode = ExitCode(EXIT_FAILURE as _);

    #[inline]
    pub fn as_i32(&self) -> i32 {
        self.0 as i32
    }
}

impl From<u8> for ExitCode {
    fn from(code: u8) -> Self {
        Self(code)
    }
}
```

<br>

