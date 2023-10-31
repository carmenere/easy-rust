# Overflow-checks
`rustc` flag `-C overflow-checks=yes|no` controls the behavior of **runtime integer overflow** ([RFC 560](https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md)):
- when this flag is **enabled** `overflow-checks=yes` a **panic** will occur on **overflow** (e.g., `255 + 1` causes to **panic**).<br>
- when this flag is **disabled** `overflow-checks=no` a **two’s complement** is used (e.g., `255 + 1` becomes `0` for an `u8` integer).<br>

<br>

Rust behaves differently in **debug mode** and **release mode** on **integer overflow**:
- in **debug mode** `overflow-checks=yes` by default;
- in **release mode** `overflow-checks=no` by default;

<br>

#### Examples
```Rust
RUSTFLAGS="-C overflow-checks=yes|no" cargo run --release

RUSTFLAGS="-C overflow-checks=yes|no" cargo run
```
