# Description
The `sccache` tool is a **general-purpose compiler cache** that can be used with Rust projects.<br>
**By default**, sccache uses up to **10 GiB** of **local storage**. The `sccache` tool can be used **directly** with a number of **networked storage backends**, e.g. `S3`, `Redis`, `Memcached`.<br>

<br>

# `sccache` commands
The `sccache` tool is distributed as **crate**.<br>

|Example|Explanation|
|:------|:----------|
|**cargo install** `sccache`|Installs `sccache` crate.|

<br>

# Example
Once installed, `sccache` is **enabled** by using it as a `rustc` **wrapper** with `cargo`.<br>
The `cargo` accepts the `RUSTC_WRAPPER` argument as an environment variable.<br>

<br>

To build any Rust project using `sccache` **set** and **export** `RUSTC_WRAPPER` to path to `sccache`, for example:
```bash
export RUSTC_WRAPPER=`which sccache`
cargo build
```

<br>

To configure `sccache`, you can specify environment variables.<br>
To configure `sccache` to use the `Redis` backend, set `SCCACHE_REDIS`:
```bash
export SCCACHE_REDIS=redis://10.10.10.10/sccache
```
