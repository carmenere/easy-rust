# Manage dependencies
There are 2 ways to manage external crates:
- **add**/**del** it to/from `Cargo.toml` file in `[dependencies]` section;
- using `cargo` command: `cargo add [options] dependency`

The `dependency` can be specified with:
- `crate`: fetches crate `crate` of **latest** version from a **registry**;
- `crate@version`: fetches crate `crate` of version `version` from a **registry**;
- `--path path`: fetches from the **specified path**;
- `--git url`: pulls from a **git repo** at url;

<br>

**Some options**:
- `--optional`: **marks** the `dependency` as **optional**;
- `--no-default-features`: **disables** the **default features**;
- `--features features`: space or comma separated list of **features to activate**;

<br>

Example of `cargo add rand` **output**:
```rust
cargo add rand
    Updating crates.io index
      Adding rand v0.10.0 to dependencies
             Features:
             + alloc
             + std
             + std_rng
             + sys_rng
             + thread_rng
             - chacha
             - log
             - serde
             - simd_support
             - unbiased
...
```

<br>

Example of `cargo add  --features 'serde' --no-default-features rand` **output**:
```rust
cargo add  --features 'serde' --no-default-features rand 
    Updating crates.io index
      Adding rand v0.10.0 to dependencies
             Features:
             + serde
             - alloc
             - chacha
             - log
             - simd_support
             - std
             - std_rng
             - sys_rng
             - thread_rng
             - unbiased
```

<br>

The `Cargo.toml` after `cargo add rand`:
```toml
[dependencies]
rand = "0.10.0"
```

<br>

The `Cargo.toml` after `cargo add rand --features 'serde'`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"] }
```

<br>

The `Cargo.toml` after `cargo add  --features 'serde' --no-default-features rand`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"], default-features = false }
```

<br>

The `Cargo.toml` after `cargo add  --features 'serde' --no-default-features --optional rand`:
```toml
[dependencies]
rand = { version = "0.10.0", features = ["serde"], default-features = false, optional = true }

[features]
rand = ["dep:rand"]
```

<br>

# `rand` crate
**RNG** means **random number generator**.<br>

**Rng** traits:
- `rand::TryRng` is a **base trait** for for **RNGs**;
- `rand::Rng: TryRng` is the **dyn-safe** implementation for **RNG**;
- `rand::RngExt` is an extension trait over `Rng`, i.e. it is a **user-level interface** for **RNGs**;
  - this trait is **not** *dyn compatible*

<br>

Both traits `Rng` and `RngExt` can be brought into scope by `use rand::prelude::*;` or `use rand::{Rng, RngExt};`.<br>

`RngExt` methods:
- `random` returns a **random value** via the **standard uniform** distribution;
- `random_iter` returns an **iterator** over random variates
  - `rng.random_iter().take(5).collect();`
- `random_range` returns a **random value** in the **given range**;
- `random_bool(p: f64)` returns a **bool** with a **probability** `p`;
- `random_ratio(numerator: u32, denominator: u32)` returns a **bool** with a **probability** `numerator/denominator`;

<br>

**Any type** that implements `TryRng` also implements `Rng`:
```rust
impl<R> Rng for R
where
    R: TryRng<Error = Infallible> + ?Sized,
```

<br>

**Any type** that implements `Rng` also implements `RngExt`:
```rust
impl<R: Rng + ?Sized> RngExt for R
```

<br>

## Function `rand::rng()`
A `rand::rng()` returns a **thread-local generator**.<br>
The *thread-local generator* can be obtained via `rand::rng()` or via `ThreadRng::default()`.<br>
The *thread-local generator* **cannot** be passed between threads (is **not** `Send` and not `Sync`).<br>

For example, `ThreadRng` implements `TryRng`:
```rust
impl TryRng for ThreadRng {
    type Error = Infallible;
    ...
}
```

<br>

Thus `ThreadRng` also implements `Rng` and `RngExt` due to blanket implementation.<br>

<br>

## Module `rand::rngs`
[rand::rngs](https://docs.rs/rand/0.10.0/rand/rngs/index.html)

<br>

## Example
`rand::random()` is a convenient alternative to `rand::rng().random()`.<br>


```rust
use rand::{rng};
use rand::prelude::*;

fn main() {
    let mut my_rng = rng();
    
    let b = my_rng.random_ratio(1,2);
    let x: u64 = my_rng.random_range(0..100);
    let y: u16 = my_rng.random();

    let random_u64 = my_rng.next_u64();
    let random_u32 = my_rng.next_u32();

    println!("b={}", b);
    println!("x={}", x);
    println!("y={}", y);
    println!("random_u32={}", random_u32);
    println!("random_u64={}", random_u64);
    
    // rand::random()
    let z: f64 = rand::random();
    println!("z={}", z);    
}
```

**Output**:
```rust
b=false
x=70
y=5405
random_u32=153476317
random_u64=11800991613183827219
z=0.8727143056527192
```

<br>

# `std::time`
More about time [**here**](https://github.com/carmenere/easy-manuals/blob/main/time/time.md).<br>

```rust
let now = std::time::Instant::now();
...
let elapsed = now.elapsed();
```

<br>

The `elapsed` corresponds the **real** time of cli command `time`.<br>
The `Instant` uses `clock_gettime` syscall with **clock source** `CLOCK_MONOTONIC`.<br>


<br>