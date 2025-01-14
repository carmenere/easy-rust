# Table of contents
- [Table of contents](#table-of-contents)
- [Rust build script (build.rs)](#rust-build-script-buildrs)
  - [Change detection](#change-detection)
- [Linkage](#linkage)
  - [Types of libraries](#types-of-libraries)
  - [`*-sys` packages](#-sys-packages)
  - [The manifest `links = %name%` key](#the-manifest-links--name-key)
- [C standard library](#c-standard-library)
- [C runtime (CRT)](#c-runtime-crt)
  - [Static and dynamic C runtimes](#static-and-dynamic-c-runtimes)
- [Examples](#examples)
  - [Building C code as part of a package](#building-c-code-as-part-of-a-package)

<br>

# Rust build script (build.rs)
**Dependencies** of the **build script** are declared through the `build-dependencies` section of the manifest.<br>
The **build script** does **not** have access to the dependencies listed in the `dependencies` or `dev-dependencies` section. Also, **build dependencies** are **not** available to the **package** itself.

<br>

Sometimes it’s necessary:
- to **generate** some **source rust code** just **before** build a package for various reasons;
- to **build** some **native** `C` or `C++` **code** as **part of a package**;
- to **link** to a **native library** to bind its functionality;
  - **native library** can either be **located on the system** or need to be **built from source**;

**Build sript** solves all problems above.

<br>

For the `build.rs` script the **general approach** is:
1. **Find** the library. There's [**pkg_config**](https://crates.io/crates/pkg-config) crate for this.
2. Select **static** or **dynamic** linking by printing `cargo:rustc-link-lib=<name>` or `cargo:rustc-link-lib=static=<name>`.
3. **Optionally** build the library from source. If the library is **unlikely** to be installed on the system *by default*, it's nice to automatically **build it from source** and link **statically**. There's [**cc**](https://crates.io/crates/cc) crate for this.
4. **Expose C headers**. For Rust you will need to translate **C headers** (`*.h` files) into a Rust module containing `extern "C" {}` **declarations** (aka **Rust FFI bindings to C**). There's [**bindgen**](https://crates.io/crates/bindgen) crate for this.

All these helper crates `bindgen`, `cc`, `pkg-config` and others **must** be added to `[build-dependencies]` section to be used in **build script**.<br>

<br>

If *package* has **build script**, `cargo build` performs 3 steps:
1. **Compiles** a **build script** into an *executable* **script**.
2. **Run** the **script**, which may perform any number of tasks.
3. **Build** *package*.

<br>

**Notes**:
- The **build script** uses the `OUT_DIR` environment variable to discover where the output files should be located. In general, build scripts **should not** modify any files **outside** of `OUT_DIR`:
```rust
let out_dir = env::var("OUT_DIR").unwrap();
```
- The **build script** can use the process’ **current working directory** to find where the **input** files should be located.

<br>

*Build scripts* **communicate** with `cargo` by printing to **stdout**. `cargo` **interprets** each line that starts with `cargo::` as an **instruction**. All **other** lines are **ignored**. The **order** of `cargo::` **instructions** affects the **order of arguments** that cargo passes to `rustc`. In turn, the **order of arguments** passed to `rustc` affects the **order of arguments** passed to the **linker**.<br>

The **output** of the **script** is **hidden** *during normal compilation*. But **all** the lines printed to stdout by a **build script** are written to a file like `target/debug/build/<pkg>/output` (the precise location may depend on your configuration). If **error** occurs then `cargo` prints out the **output** of the **build script**.
If you would like to see the output in your terminal *during normal compilation*, invoke `cargo` with the `-vv` flag.

<br>

[All instructions that Cargo recognizes](https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script).<br>

Some instructions:
- `cargo::rustc-link-lib=LIB` instruction tells `cargo` to **link** the **given library** `LIB` using the compiler’s `-l` **flag**;
  - **full syntax** for `LIB` is `[KIND[:MODIFIERS]=]NAME[:RENAME]`;
- `cargo::rustc-link-search=[KIND=]PATH` instruction tells `cargo` to pass the `-L` **flag** to the compiler to add a directory to the library search path;

<br>

The `-l` **flag** allows to specify linking to a specific **native library** when building a crate. This is typically used to link a **native library** using **FFI**.<br>
The **kind** of library can optionally be specified with the form `-l KIND=lib` where `KIND` may be one of:
- `dylib` a **native dynamic library**;
- `static` a **native static library** (such as a `.a` archive);
- `framework` a macOS framework;

<br>

The `-L` **flag** adds a path to search for external crates and libraries to the **library search path**.

The **kind** of search path can optionally be specified with the form `-L KIND=PATH` where `KIND` may be one of:
- `dependency` **only** search for transitive dependencies in this directory;
- `crate` **only** search for this crate's direct dependencies in this directory;
- `native` **only** search for native libraries in this directory;
- `framework` **only** search for macOS frameworks in this directory;
- `all` — Search for all library kinds in this directory. This is the default if KIND is not specified;

<br>

## Change detection
By default, `cargo` **always** re-running the build script if **any** file within the package is **changed** (or the list of files controlled by the `exclude` and `include` fields). If `cargo` determines nothing has changed, it will **not** re-run the **script**.<br>
It is recommended that every build script emit **at least one** of the `rerun-if` instructions. If `rerun-if` instructions are emitted, then `cargo` **re-runs** the script if **only** the given values (by `rerun-if` instructions) have changed.<br>

`rerun-if` instructions:
- `cargo::rerun-if-changed=PATH` tells `cargo` to **re-run** the **build script** if the **file** at the given **path** has **changed**, **if** the path points to a **directory**, it will scan the entire directory for any modifications;
- `cargo::rerun-if-env-changed=VAR` tells `cargo` to **re-run** the **build script** if the **value** of an environment **variable** of the given **name** has **changed**.

<br>

# Linkage
The compiler supports various methods to link crates together both **statically** and **dynamically**.<br>

<br>

## Types of libraries
The option `crate-type` in *target* in `Cargo.toml` defines the **crate types** that will be generated by the *target*. The available options are `bin`, `lib`, `rlib`, `dylib`, `cdylib`, `staticlib` and `proc-macro`. This can only be specified for **libraries** and **examples**. **Binaries**, **tests**, and **benchmarks** are **always** the `bin` **crate type**.<br>

<br>

There are **several types of libraries**:
|  |Linked from Rust|Linked from non-Rust|
|:-|:---------------|:-------------------|
|**Static**|`rlib`|`staticlib`|
|**Dynamic**|`dylib`|`cdylib`|

<br>

- `rlib` produces a **static Rust library**;
  - this is used as an **intermediate artifact**;
  - this is used to produce **statically** linked **executables** as well as **staticlib** outputs;
- `dylib` produces a **dynamic Rust library**:
  - this **forces** **dynamic library** generation;
  - the resulting **dynamic library** can be used as a **dependency** for **other libraries** and/or **executables**;
  - this output type will create:
    - `*.so` files on **Linux**;
    - `*.dylib` files on **macOS**;
    - `*.dll` files on **Windows**;
- `cdylib` produces a **dynamic system library** (aka **shared library**), it can be used by **non**-Rust apps;
  - this is used when compiling a **dynamic library** to be loaded from **another language**;
  - this output type will create:
    - `*.so` files on **Linux**;
    - `*.dylib` files on **macOS**;
    - `*.dll` files on **Windows**;
- `staticlib` produces a **static system library**, it can be used by **non**-Rust apps:
  - this output type will create:
    - `*.a` files on **Linux**, **macOS** and **Windows**;
    - `*.lib` files on **Windows** (**MSVC**);
- `lib` is an **aliase** for one of `rlib`/`dylib`/`cdylib`/`staticlib` but the **actual type** of library is **chosen by compiler**;

<br>

## `*-sys` packages
**C library** (aka **system library**, **native library**).
A **native dependency** is any dependency that requires compilation of `C++`/`C` code.<br>
**Packages** that link to **system libraries** (**C libraries**) are also called **native dependencies**.<br>
**Native dependencies** have a **naming convention** of having a `-sys` **suffix**.<br>

So, it is fairly simple to **find native dependencies**:
- `cargo tree | grep '\-sys'`;

It is common to have a **companion package** without the `-sys` suffix that provides a safe, high-level abstractions on top of the **sys package**.<br>
For example, the `git2` crate provides a high-level interface to the `libgit2-sys` crate.

<br>

## The manifest `links = %name%` key
The `links` key is only **informational** and it does **not** actually **link** to anything. It **informs** Cargo that this crate links with the given **C library** (aka **system library**, **native library**) and Cargo must **ensure** that only **one** copy of the library is **linked**. It is **forbidden** to have **two packages** that link to the **same native library**.<br>

When the `links` key is used, the package **must** have a **build script**, and the *build script* **should** use the `rustc-link-lib` instruction to **link** the **library**.<br>

The name `%name%` of **C library** must be **without** any **prefix**/**suffix** (e.g. `links = "z"`, **not** `"libz.so"`), example:<br>
```toml
[package]
# ...
links = "foo"
```

This states that the package links to the `libfoo` **native library**.<br>

<br>

# C standard library
The **C standard library** (aka **ISO C library** or **libc**) is the **standard library** for the **C programming language**, as specified in the **ISO C standard**.<br>
Some of the popular **implementations** of *C standard library*:
- **BSD libc**;
- **glibc** (GNU C Library);
- **musl** (**lightweight** implementation of **libc** for Linux systems);

<br>

# C runtime (CRT)
**CRT** stands for **C runtime**.<br>

There is a very important difference between **C standard library** and **CRT**:
- the **C standard library** defines functions that are (always) available to the programmer;
- **CRT** is a thin layer of code compiled in binary that contains **startup routine** and **error handling** code;

<br>

When an executable is loaded in the memory, then OS calls `_start` **entrypoint** of the binary, **not** `main`. This `_start` **entrypoint** runs some **setup code** required **before** calling the `main` function and some **cleanup** code required **after** main returns. This **program startup code** is implemented in the **CRT**.<br>

<br>

The **CRT** is an **object file**, **not** library. For example, you could write a program which **does not** use the **C standard library** but you always need the **CRT** because otherwise, your program could **not** be executed. So, **CRT** is automatically linked into your program by the compiler.<br>

The **CRT** comes in two flavors:
- **CRT1** is used on systems that support **constructors** (functions called before `main`) and **destructors** (functions called after `exit`). In this case `main` is treated like a normal function call;
- **CRT0** is used on systems that do **not** support constructors/destructors (the **zero** (**0**) stands for **the very beginning**);

<br>

The **CRT** is shipped as part of the **compiler** or **OS** and usually reside in the `crt0.o` or `crt1.o` **object file**:
```bash
anton@zinfandel-x86:~/Projects/foo$ ls -hal /lib/x86_64-linux-gnu/ | grep crt
-rw-r--r--   1 root root  1.8K Aug  8 17:47 crt1.o
-rw-r--r--   1 root root  1.2K Aug  8 17:47 crti.o
-rw-r--r--   1 root root   760 Aug  8 17:47 crtn.o
-rw-r--r--   1 root root  2.5K Aug  8 17:47 gcrt1.o
-rw-r--r--   1 root root  2.3K Aug  8 17:47 grcrt1.o
-rw-r--r--   1 root root   608 Aug  8 17:47 Mcrt1.o
-rw-r--r--   1 root root  1.7K Aug  8 17:47 rcrt1.o
-rw-r--r--   1 root root  1.7K Aug  8 17:47 Scrt1.o
```

<br>

The work performed by **CRT** depends on the **language**, **compiler**, OS and **C standard library** implementation. Ususally **CRT** performs following work:
- **before** calling the `main`:
  - initializes the **stack**;
  - pushes `argc` and `argv` onto the **stack**;
  - initializes the `.bss` section to zero;
  - initializes the `heap`;
  - **call** the `main()`;
- **after** `main` returns:
  - pops `argc` and `argv` from the **stack**;
  - stores the **return code** from `main()` in `eax`;
  - calls `exit()`;

<br>

In a **typical** Rust **binary** that links the **standard library**, **execution starts** in a **CRT**. This creates a stack and places the arguments in the right registers. Then **crt0** invokes the [**entry point**](https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L32-L73) of the **Rust runtime**, which is marked by the [**start**](https://github.com/rust-lang/rust/blob/bb4d1491466d8239a7a5fd68bd605e3276e97afb/src/libstd/rt.rs#L31) *language item*.<br>

<br>

## Static and dynamic C runtimes
All targets in the compiler have a **default mode** of **linking** to the **C runtime library**. Here, **C runtime library** means **not** only **CRT**, but also **implementation** of *C standard library*, for eample, **glibc** or **musl**. Typically targets are linked **dynamically** by default, e.g. `aarch64-apple-darwin` or `x86_64-unknown-linux-gnu`. But there are **exceptions** which are linked **statically** by default, e.g. `x86_64-unknown-linux-musl`.<br>

<br>

The `crt-static` **feature** of `target-feature` **codegen option** configure **linkage** of the **C runtime library**:
- `-C target-feature=+crt-static` instructs the Rust compiler to **statically** link to **C runtime library**;
- `-C target-feature=-crt-static` instructs the Rust compiler to **dynamically** link to **C runtime library**;

It's recommended to **inspect the resulting binary** to ensure that it's linked as you would expect after the compiler succeeds.<br>

> **Note**:<br>
> **Alpine** libraries link **dynamically** to **musl libc**, while `rustc` **statically** links to **musl libc** *by default*.<br>

<br>

# Examples
## Building C code as part of a package
The package layout:
```rust
.
├── Cargo.toml
├── build.rs
└── src
    ├── hello.c
    └── main.rs
```

<br>

The `Cargo.toml`:
```toml
[package]
name = "hello-world-c"
version = "0.1.0"
edition = "2021"

[build-dependencies]
cc = "1.0"

[dependencies]
```

<br>

The `build.rs`:
```rust
fn main() {
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
    println!("cargo::rerun-if-changed=src/hello.c");
}
```

<br>

The `src/hello.c`:
```c
#include <stdio.h>

void hello() {
    printf("Hello, World!\n");
}
```

<br>

The `src/main.rs`:
```rust
extern { fn hello(); }

fn main() {
    unsafe { hello(); }
}
```