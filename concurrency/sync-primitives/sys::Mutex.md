# Table of contents
- [Table of contents](#table-of-contents)
- [Rust language project](#rust-language-project)
- [`std`](#std)
- [Paths](#paths)

<br>

# Rust language project
The Rust's `Cargo.toml` has `"library/std"` in its `members` section.<br>
[**Cargo.toml**](https://github.com/rust-lang/rust/blob/master/Cargo.toml):
```toml
[workspace]
resolver = "1"

members = [
  "library/std",
]
```

<br>

# `std`
The std's `Cargo.toml` has `libc` in its `dependencies` section.<br>
[**Cargo.toml**](https://github.com/rust-lang/rust/blob/master/library/std/Cargo.toml):
```toml
[target.'cfg(not(all(windows, target_env = "msvc")))'.dependencies]
libc = { version = "0.2.153", default-features = false, features = ['rustc-dep-of-std'], public = true }

[target.'cfg(all(windows, target_env = "msvc"))'.dependencies]
libc = { version = "0.2.153", default-features = false }
```

<br>

The `lib.rs` of `std` imports `sys` and `sync` modules:
```rust
pub mod sync;

// Platform-abstraction modules
mod sys;
mod sys_common;
```

<br>

# Paths
- `rust/library/std/src/sys/sync/mutex/pthread.rs` ([**pthread.rs**](https://github.com/rust-lang/rust/blob/master/library/std/src/sys/sync/mutex/pthread.rs)):
  - this uses `pthread` **syscalls** directly (through `libc`):
    - `libc::pthread_mutexattr_init`;
    - `libc::pthread_mutexattr_settype`;
    - `libc::pthread_mutexattr_destroy`;
    - `libc::pthread_mutex_init`;
    - `libc::pthread_mutex_lock`;
    - `libc::pthread_mutex_trylock`;
    - `libc::pthread_mutex_unlock`;
    - `libc::pthread_mutex_destroy`;
- `rust/library/std/src/sync/mutex.rs` ([**std::sync::Mutex**](https://github.com/rust-lang/rust/blob/master/library/std/src/sync/mutex.rs))
  - **std::sync::Mutex** uses primitives from `rust/library/std/src/sys/sync/mutex/pthread.rs`;
