# Example: syscall wrapper
## main.rs
```rust
use std::ffi::CString;
use libc;

#[allow(unused_macros)]
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

fn main() {
    let c_str = CString::new("/tmp/test_file").unwrap();
    let fd = syscall!(open(c_str.as_ptr() as *const i8, libc::O_CREAT));
    let _ = syscall!(close(fd.unwrap()));
}
```

<br>

## Cargo.toml
```toml
[package]
name = "syscall"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2.150"

[profile.abc]
inherits = "release"
opt-level = 3
```