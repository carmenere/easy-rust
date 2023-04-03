# Features
**Features** provide a **mechanism** for **conditional compilation**.<br>
**Features** are defined in the ``[features]`` section of ``Cargo.toml`` file.<br>
**Each feature** can either be **enabled** or **disabled**.


Properties of features:
- Each *feature* specifies an **array** of **other features** or **optional dependencies** that it enables;
- If *feature* is enabled it in turn enables the listed features in array;
- Empty array means that feature does not enable any other features;
- *By default*, **all features are disabled** unless **explicitly enabled** or **listed in default feature**.

#### Example
```toml
[features]
bmp = []
png = []
ico = ["bmp", "png"]
foo = []
```

Then feature ``foo`` can be used to conditionally include **any item** in code, for instance, module ``bar``:
```Rust
#[cfg(feature = "foo")]
pub mod bar;
```

<br>

## Ways to manage features
- *Features* of **package being built** are managed by **command-line flags**.
- *Features* of **dependencies** are managed in the **dependency declaration**;
- *Features* of **dependencies** are also managed in the ``[features]`` table,  the syntax is ``feature_name = ["package-name/feature-name"]``.

### Command-line flags to mange features
|CLI flag|Description|
|:---|:----------|
|``--features FEATURES``|Here ``FEATURES`` is a **space** or **comma** separated **list of features** to activate.<br>Example:`` --features "foo bar"``|
|``--no-default-features``|**Disables default feature**|
|``--all-features``|Activates all available features of all packages|

### Dependency declaration attributes to mange features
|Attribute|Description|
|:--------|:----------|
|``features=["foo", "bar"]``|Comma separated **list of features** to activate.|
|``default-features=true/false``|**Enables** or **disables** **default feature** of dependency.|
|``optional=true``|``optional=true`` makes dependency **optional**. It means that such dependency **will not be compiled by default**.|

#### Example
```toml
[dependencies]
foo1 = { version = "0.1", features=["bar1", "baz1"] }
foo2 = { version = "0.2", features=["baz2"], default-features = false }
foo3 = { version = "0.3", optional = true }
```

### ``package-name/feature-name`` in the ``[features]`` table

*Features* of **dependencies** can also be enabled in the ``[features]`` table.<br>
The syntax is ``feature_name = ["package-name/feature-name"]``.

#### Example
```toml
[dependencies]
foo = { version = "0.1", default-features = false }

[features]
baz = ["foo/bar"]
```

<br>

## Default feature
There is special feature: **default feature**.<br>
*By default*, **default feature** is **enbaled**.<br>

#### Example
```toml
[features]
default = ["ico", "webp"]
bmp = []
png = []
ico = ["bmp", "png"]
webp = []
```

#### Ways to disable **default feature** 
- The ``--no-default-features`` *command-line flag* **disables** the **default feature** of **the package**.
- The ``default-features = false`` *attribute* of a dependency declaration **disables** the **default feature** of **the dependency**.

<br>

## Optional dependencies
#### Example
```toml
[dependencies]
foo = { version = "0.11.1", optional = true }
```

By default, above **optional dependency** ``foo`` **implicitly defines a feature** ``foo`` that looks like this:
```toml
[features]
foo = ["dep:foo"]
```

This means that **dependency** ``foo`` will only be included if the ``foo`` **feature** is **enabled**.<br>
The same ``cfg(feature = "foo")`` syntax can be used in the code, and the dependency can be enabled by ``--features foo``.

In some cases, you may not want to expose a feature that has the same name as the optional dependency.<br>
For example, perhaps the optional dependency is an internal detail, or you want to group multiple optional dependencies together, or you just want to use a better name.<br>
If you specify the **optional dependency** with the ``dep:`` prefix **anywhere in the** ``[features]`` **table**, this **disables** the **implicit feature**.<br>

**Note**: The ``dep:`` syntax is only available starting with `Rust 1.60`.<br>

For example, let's say in order to support the **AVIF** image format, our library needs two other dependencies to be enabled:
```toml
[dependencies]
ravif = { version = "0.6.3", optional = true }
rgb = { version = "0.8.25", optional = true }

[features]
avif = ["dep:ravif", "dep:rgb"]
```

In this example, the ``avif`` feature will enable the two listed dependencies.<br>
This also avoids creating the implicit ``ravif`` and ``rgb`` features, since we don't want users to enable those individually as they are internal details to our crate.

# More examples
## 1. Reference example
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

[dependencies]
futures = {version = "0.3"}
```

<br>

```bash
an.romanov@NB0737 ~/P/play-rust (master)> rm -rf target
an.romanov@NB0737 ~/P/play-rust (master)> cargo run
   Compiling proc-macro2 v1.0.44
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling syn v1.0.100
   Compiling autocfg v1.1.0
   Compiling futures-core v0.3.24
   Compiling futures-task v0.3.24
   Compiling futures-channel v0.3.24
   Compiling memchr v2.5.0
   Compiling futures-sink v0.3.24
   Compiling futures-util v0.3.24
   Compiling pin-utils v0.1.0
   Compiling pin-project-lite v0.2.9
   Compiling futures-io v0.3.24
   Compiling slab v0.4.7
   Compiling futures-macro v0.3.24
   Compiling futures-executor v0.3.24
   Compiling futures v0.3.24
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 5.70s
     Running `target/debug/ololo`
ABC!
```

## 2. Enabling certain features of particular package inside ``[features]`` section and assigning aliases for them
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

[dependencies]
futures = {version = "0.3", default-features = false }

[features]
abc = ["futures/alloc"]
```

<br>

```bash
an.romanov@NB0737 ~/P/play-rust (master)> rm -rf target
an.romanov@NB0737 ~/P/play-rust (master)> cargo run --features "abc"
    Blocking waiting for file lock on build directory
   Compiling futures-core v0.3.24
   Compiling futures-task v0.3.24
   Compiling futures-util v0.3.24
   Compiling futures-channel v0.3.24
   Compiling futures-sink v0.3.24
   Compiling pin-utils v0.1.0
   Compiling pin-project-lite v0.2.9
   Compiling futures-io v0.3.24
   Compiling futures v0.3.24
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 3.33s
     Running `target/debug/ololo`
ABC!
```

<br>

## 3. Disabling ``default-features`` of certain package inside dependency declaration
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

[dependencies]
futures = {version = "0.3", default-features = false }
```

```bash
an.romanov@NB0737 ~/P/play-rust (master)> rm -rf target
an.romanov@NB0737 ~/P/play-rust (master)> cargo run
   Compiling futures-core v0.3.24
   Compiling futures-task v0.3.24
   Compiling futures-channel v0.3.24
   Compiling futures-util v0.3.24
   Compiling futures-sink v0.3.24
   Compiling pin-project-lite v0.2.9
   Compiling pin-utils v0.1.0
   Compiling futures-io v0.3.24
   Compiling futures v0.3.24
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 2.21s
     Running `target/debug/ololo`
ABC!
```

<br>

## 4. Optional dependency (``optional = true``)
#### 4.1 Disabling certain package
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
futures = {version = "0.3", optional = true }
```

<br>

```bash
an.romanov@NB0737 ~/P/play-rust (master)> cargo run
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/ololo`
ABC!
```

<br>

#### 4.2 Enabling certain package by assigning alias to this package, e.g., ``<abc>``, and passing it to cli argument ``--features`` of ``cargo``
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
futures = {version = "0.3", optional = true }

[features]
abc = ["dep:futures"]
```

<br>

```bash
an.romanov@NB0737 ~/P/play-rust (master)> rm -rf target
an.romanov@NB0737 ~/P/play-rust (master)> cargo run --features "abc"
   Compiling proc-macro2 v1.0.44
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling autocfg v1.1.0
   Compiling futures-core v0.3.24
   Compiling syn v1.0.100
   Compiling memchr v2.5.0
   Compiling futures-task v0.3.24
   Compiling futures-channel v0.3.24
   Compiling futures-sink v0.3.24
   Compiling futures-util v0.3.24
   Compiling pin-project-lite v0.2.9
   Compiling futures-io v0.3.24
   Compiling pin-utils v0.1.0
   Compiling slab v0.4.7
   Compiling futures-macro v0.3.24
   Compiling futures-executor v0.3.24
   Compiling futures v0.3.24
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 5.63s
     Running `target/debug/ololo`
ABC!
```

<br>

#### 4.3 Enabling certain package, e.g., ``futures``, by passing its name to cli argument ``--features`` of ``cargo``
```toml
[package]
name = "ololo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
futures = {version = "0.3", optional = true }
```

<br>

```bash
an.romanov@NB0737 ~/P/play-rust (master)> rm -rf target
an.romanov@NB0737 ~/P/play-rust (master)> cargo run --features "futures"
   Compiling proc-macro2 v1.0.44
   Compiling quote v1.0.21
   Compiling unicode-ident v1.0.4
   Compiling autocfg v1.1.0
   Compiling futures-core v0.3.24
   Compiling syn v1.0.100
   Compiling memchr v2.5.0
   Compiling futures-task v0.3.24
   Compiling futures-channel v0.3.24
   Compiling futures-sink v0.3.24
   Compiling futures-util v0.3.24
   Compiling pin-utils v0.1.0
   Compiling pin-project-lite v0.2.9
   Compiling futures-io v0.3.24
   Compiling slab v0.4.7
   Compiling futures-macro v0.3.24
   Compiling futures-executor v0.3.24
   Compiling futures v0.3.24
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 5.58s
     Running `target/debug/ololo`
ABC!
```
