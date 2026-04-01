# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Manage dependencies](#manage-dependencies)
<!-- TOC -->

<br>

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

The `Cargo.toml` after `cargo add rand`:
```toml
[dependencies]
rand = "0.10.0"
```

<br>

The *features* that are **enabled** have a `+` to the **left**, and those that **aren’t** enabled have a `-` instead.<br>

<br>

A lot of crates use **features**, which let you compile just a part of the crate. *Some features* are **enabled by default**, but if you want to **add** *more functionality*, you have to **activate** *additional features*.<br>

The `cargo add <crate_name>` command **does not** have a dedicated flag to display **only** the default features of a crate, but it can list **all enabled features** including the default ones using the `--dry-run` (or shorthand `-n`) flag. If crate is already added to `Cargo.toml` and has some additional features, e.g. `reqwest = { version = "0.13.2", features = ["blocking"] }`, all these features will be displayed as `+`. To show **only default features** - **remove** crate by `cargo remove <crate_name>` and then call `cargo add --dry-run <crate_name>`.
```bash
cargo add --dry-run reqwest
    Updating crates.io index
      Adding reqwest v0.13.2 to dependencies
             Features:
             + __rustls
             + __rustls-aws-lc-rs
             + __tls
             + charset
             + default-tls
             + http2
             + rustls
             + system-proxy
             - __native-tls
             - __native-tls-alpn
             - blocking
             - brotli
             - cookies
             - deflate
             - form
             - gzip
             - hickory-dns
             - http3
             - json
             - multipart
             - native-tls
             - native-tls-no-alpn
             - native-tls-vendored
             - native-tls-vendored-no-alpn
             - query
             - rustls-no-provider
             - socks
             - stream
             - zstd
warning: aborting add due to dry run
```

<br>

You can also find out whether a feature is behind a feature flag by looking through the source code for the attribute `#[cfg(feature = "<feature_name>")]`.
Example: [`reqwest` crate](https://docs.rs/reqwest/latest/src/reqwest/lib.rs.html)
```rust
    #[cfg(feature = "blocking")]
    pub mod blocking;
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
