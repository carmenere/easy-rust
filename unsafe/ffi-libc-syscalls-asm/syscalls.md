# Raw syscalls
Any OS provides *stable* **syscall ABI** through its library. A **syscall ABI** guarantees that if you put right data in certain registers and call specific CPU instruction that passes control to the OS, it will always do the same thing.<br>
A **raw syscall** is one that bypasses the OS-provided library for making syscalls. A **raw syscall** depends on platform.<br>

**Examples**:
- **raw syscall** for MacOS aarch64:
```rust
use std::arch::asm;

#[inline(never)]
fn syscall(msg: String) {
    unsafe {
        asm!(
            "mov x16, 4",
            "mov x0, 1",
            "svc 0",
            in("x1") msg.as_ptr(),
            in("x2") msg.len(),
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        )
    }
}

fn main() {
    syscall("Hello!\n".to_string());
}
```
- **raw syscall** for Linux aarch64:
```rust
use std::arch::asm;

#[inline(never)]
fn syscall(msg: String) {
    unsafe {
        asm!(
            "mov x0, 1",
            "mov w8, 64",
            "svc 0",
            in("x1") msg.as_ptr(),
            in("x2") msg.len(),
            out("w8") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        );
    }
}

fn main() {
    syscall("Hello!\n".to_string());
}
```

<br>

**Explanations**:
|Code|Meaning|
|:---|:------|
|`#[inline(never)]`|Attribute ``#[inline(never)]`` asks the compiler don't inline function during optimization.|
|`mov x0, 1`|Puts **fd** number of **stdin** (**1**) to register `x0`.|
|`mov w8, 64`|Puts **fd** number of **syscall** (**64** for linux aarch64) to `w8` register. It changes from *OS to OS* and from *arch to arch*.|
|`svc 0`|Calls **syscall instruction** which issues a **software interrupt** and passes control to CPU.|
|`in("x1") msg.as_ptr()`|Puts **address of buffer** where string is stored to `x1` register.|
|`in("x2") msg.len()`|Puts length of string in bytes to `x2` register.|

<br>

The last 4 lines are not instructions to CPU, they tell compiler that it **cannot** store anything in these registers and assume that **data** is **untouched** when we exit the **inline assembly** block.

<br>

# OS syscalls
The next level of abstraction is the OS kernel API and ABI. Kernel ABI is about **calling convention**.<br>

Before call a **foreign function** you must specify what **calling convention** to use since there is no way for compiler to know it.<br>

The **C calling convention** (aka **C declaration**, **cdecl**) is the most common one.<br>

The **calling convention** specifies:
- how **arguments** are passing to function;
- how function **returns** its result;
- how **registers** are used;
- how **stack** is set up;

<br>

**Example**:
```rust
#[cfg(target_family = "unix")]

use std::io;

#[link(name = "C")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

fn syscall(msg: String) -> io::Result<()> {
    let res = unsafe {
        write(1, msg.as_ptr(), msg.len())
    };
    if res == -1 {
        return Err(io::Error::last_os_error())
    }
    Ok(())
}

fn main() {
    syscall("Hello!\n".to_string());
}
```

<br>

Explanations:
|Code|Meaning|
|:---|:------|
|`#[cfg(target_family = "unix")]`|This differentiates between the platforms.|
|`#[link(name = "C")]`|This attribute should be applied to an `extern` block with non-Rust ABI. This tells the compiler to link to the **C library** on the system.|
|`extern "C" {}`|This tells the compiler that we want to use **C calling convention** when calling the function `write` in the **C library** we're linking to.<br>This function must have the **exact same name** as in library we're linking to, but its parameters **don't** have to have the same name, but **must** be in the **same order**.<br>It can be written without `"C"`, because `"C"` is assumed if **nothing** is specified.|
|`unsafe`|We must wrap calls to **foreign functions** in an `unsafe` blocks, because Rust **can't guarantee safety** when calling **external functions**.|
|`io::Error::last_os_error()`|Returns an **error** representing the **last OS error** which occurred. This function reads the value of **errno** for the target platform and returns a corresponding instance of `Error` for the error code.|
|`res == -1`|Syscalls often return the value `-1` on **error**.|

<br>

## `errno.h`
- [errno-base.h](https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno-base.h)
- [errno.h](https://github.com/torvalds/linux/blob/master/include/uapi/asm-generic/errno.h)

<br>

Rust *standard library* **wraps** the calls to the underlying OS for us, so **we don't have to care about kernel API and ABI**.<br>

<br>

# Example of macro that wraps any syscall
Add `libc` to `[dependencies]` in `Cargo.toml`:
```toml
[dependencies]
libc = "0.2.150"
```

<br>

The code:
```rust
use std::ffi::CString;

#[macro_export]
macro_rules! syscall {
    (
        $fn: ident ( $($arg: expr),* $(,)* ) 
    ) => 
    {
        {
            let res = unsafe { libc::$fn($($arg, )*) };
            if res == -1 {
                Err(std::io::Error::last_os_error())
            } else {
                Ok(res)
            }
        }
    };
}

pub fn create_file(path: CString) {
    let fd = syscall!(open(path.as_ptr() as *const i8, libc::O_CREAT));
    let _ = syscall!(close(fd.unwrap()));
}

fn main() {
    create_file(CString::new("/tmp/testfile").unwrap());
}
```