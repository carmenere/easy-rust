# Table of contents
- [Table of contents](#table-of-contents)
- [Attributes](#attributes)
  - [Inner attributes](#inner-attributes)
      - [Example](#example)
  - [Outer attributes](#outer-attributes)
      - [Example](#example-1)
  - [Attributes arguments](#attributes-arguments)
      - [Attributes without arguments](#attributes-without-arguments)
      - [Attributes with arguments](#attributes-with-arguments)
  - [Kinds of attributes](#kinds-of-attributes)
- [Built-in attributes](#built-in-attributes)
  - [Attr: `derive`](#attr-derive)
  - [Attr: `path`](#attr-path)
- [Conditional compilation](#conditional-compilation)
  - [Configuration options](#configuration-options)
- [Key-value options](#key-value-options)
  - [Examples](#examples)
  - [Attr: `cfg`](#attr-cfg)
  - [Attr: `cfg_attr`](#attr-cfg_attr)
- [The `cfg!` macro](#the-cfg-macro)

<br>

# Attributes
Declaration of **any item** in a Rust can be decorated (annotated) with one or more **attribute**. <br>
Each **attribute** contains **instructions for compiler**.<br>


In Rust items are
- Functions
- Types (structs, enums, unions, type aliases)
- Traits
- Impl blocks
- Macros
- Constants and statics
- Extern blocks
- Extern crates
- Imports
- Modules

<br>

There are 2 types of attributes:
- **inner** attributes;
- **outer** attributes.

<br>

To attach attribute **to whole crate** include **inner atribute** to the **root module**: `main.rs` or `lib.rs`.

<br>

## Inner attributes
**Inner** attributes apply to **all items** within the scope where attribute is declared.

#### Example
```Rust
mod Bar {
    #![bar]
}
```

Here, the `#![bar]` attribute applies **to all items** inside module `Bar`.

<br>

## Outer attributes
**Outer** attributes apply **only to 1 item** following the attribute.

#### Example
```Rust
#[foo]
struct Foo;
```
Here, the `#[foo]` attribute applies **only to the next item** `Foo`.

<br>

## Attributes arguments
Some *attributes* **require arguments**, some *attributes* can be used **without arguments**.

#### Attributes without arguments
```Rust
// A unit test
#[test]
fn check() {
    assert_eq!(2, 1 + 1);
}
```

<br>

#### Attributes with arguments
```Rust
// A conditional compilation
#[cfg(target_os = "linux")]
mod bar {
    /* ... */
}

// A lint attribute
#[allow(non_camel_case_types)]
type int8_t = i8;

```

<br>

## Kinds of attributes
- **built-in** attributes;
- **tool** attributes;
- **macro** attributes;
- **derive macro helper** attributes.

<br>

# Built-in attributes
**Built-in attributes** per categories:
- Conditional compilation
    - `cfg`
    - `cfg_attr`
- Testing
    - `test`
- Derive
    - `derive`
- Macros
    - `proc_macro`
- Lint
    - `allow`
    - `warn`
    - `deny`
    - `forbid`
    - `deprecated`
    - `must_use`
- Code generation
    - `inline`
- Documentation
    - `doc`
- Preludes
    - `no_std`
- Modules
    - `path`
- Limits
    - `recursion_limit`
    - `type_length_limit`
- Runtime
    - `panic_handler`
    - `global_allocator`
- ABI, linking, symbols, and FFI

<br>


## Attr: `derive`
The `derive` attribute allows *certain* **traits** to be **automatically implemented** for data structures.
```Rust
#[derive(PartialEq, Clone)]
struct Foo<T> {
    a: i32,
    b: T,
}
```

<br>

## Attr: `path`
The `path` attribute specifies the filename for a module.
```Rust
#[path = "foo.rs"]
mod c;
```

It means file `foo.rs` will be included into module tree as `c` module.

<br>

# Conditional compilation
## Configuration options
**Configuration options** are either **names** or **key-value pairs**, and are either **set** or **unset**.<br>

<br>

# Key-value options
- `target_abi`
- `target_arch` set once with the **target’s CPU architecture**, it is similar to the first element of the target triple, but **not** identical;
  - **example** values:
    - `"x86"`
    - `"x86_64"`
    - `"mips"`
    - `"powerpc"`
    - `"powerpc64"`
    - `"arm"`
    - `"aarch64"`
- `target_endian` set once with either a value of `"little"` or `"big"` depending on the **endianness** of the target’s CPU;
- `target_env`
- `target_family` defines the **family** of the operating systems or architectures, **any** number of target_family key-value pairs can be set;
  - **example** values:
    - `"unix"`;
    - `"windows"`;
    - `"wasm"`;
    - Both `"unix"` and `"wasm"`;
- `target_feature` set feature available for the current **target triple**;
  - each **target triple** has a **set of features** that may be enabled;
  - **example** values:
    - `"avx"`
    - `"avx2"`
    - `"crt-static"`
    - `"rdrand"`
    - `"sse"`
    - `"sse2"`
    - `"sse4.1"`
- `target_os` set once with the **target’s operating system**;
- `target_vendor`
- `panic` set once with the panic strategy;
  - **example** values:
    - `"abort"`
    - `"unwind"`

<br>

**Example**:
```rust
#[cfg(target_feature = "crt-static")]
compile_error!("Detected crt-static mode");
```
<br>

**Names options**:
- `test` enabled when compiling the test;
- `unix`
  - `unix` is set if `target_family = "unix"` is set.
- `windows`
  - `windows` is set if `target_family = "windows"` is set.

<br>

## Examples
Print out **all** set configuration options:
```bash
rustup default
1.76.0-aarch64-apple-darwin (default)

rustc --print cfg
debug_assertions
panic="unwind"
target_arch="aarch64"
target_endian="little"
target_env=""
target_family="unix"
target_feature="aes"
target_feature="crc"
target_feature="dit"
target_feature="dotprod"
target_feature="dpb"
target_feature="dpb2"
target_feature="fcma"
target_feature="fhm"
target_feature="flagm"
target_feature="fp16"
target_feature="frintts"
target_feature="jsconv"
target_feature="lor"
target_feature="lse"
target_feature="neon"
target_feature="paca"
target_feature="pacg"
target_feature="pan"
target_feature="pmuv3"
target_feature="ras"
target_feature="rcpc"
target_feature="rcpc2"
target_feature="rdm"
target_feature="sb"
target_feature="sha2"
target_feature="sha3"
target_feature="ssbs"
target_feature="vh"
target_has_atomic="128"
target_has_atomic="16"
target_has_atomic="32"
target_has_atomic="64"
target_has_atomic="8"
target_has_atomic="ptr"
target_os="macos"
target_pointer_width="64"
target_vendor="apple"
unix
```

<br>

**On MacOS**:
```bash
rustc -C target-feature=+crt-static --print cfg | grep crt
```

<br>

**On Linux**:
```bash
rustc -C target-feature=+crt-static --print cfg | grep crt
target_feature="crt-static"
```

<br>

Cargo sets **features** in the package using the rustc `--cfg` flag. If feature is set it is added to **configuration options** list:
```bash
rustc --cfg 'feature="foo"' --cfg 'feature="bar"' --print cfg | grep 'foo\|bar'
feature="bar"
feature="foo"
```

<br>

## Attr: `cfg`
The `cfg` attribute conditionally includes the thing it is attached to based on a **configuration predicate**.<br>
If the **predicate** is `false`, the thing is **removed** from the source code.<br>

```rust
#[cfg(target_os = "macos")]
fn macos_only() {
  // ...
}
```

<br>

The **predicate** is one of the following:
- a **configuration option**: the predicate is `true` if the **option** is **set**, and `false` if it is **unset**;
- `all()` with a comma-separated list of **configuration predicates**;
- `any()` with a comma-separated list of **configuration predicates**;
- `not()` with a **configuration predicate**;

<br>

## Attr: `cfg_attr`
The `cfg_attr` conditionally includes **other attributes** based on a configuration predicate.<br>
When the configuration predicate is **true**, this attribute expands out to the attributes listed after the predicate.<br>

For example, depending on `target_os` the module `os` corresponds to different `.rs` files:
```rust
#[cfg_attr(target_os = "linux", path = "linux.rs")]
#[cfg_attr(windows, path = "windows.rs")]
mod os;
```

<br>

Enother example:
```rust
#[cfg_attr(feature = "magic", sparkles, crackles)]
fn bewitched() {}
```

When the `magic` feature flag is **enabled**, the above will expand to:
```rust
#[sparkles]
#[crackles]
fn bewitched() {}
```


<br>

# The `cfg!` macro
The built-in `cfg!` macro takes in a **single configuration predicate** and evaluates to the `true` **literal** when the *predicate* is `true` and the `false` **literal** when it is `false`:<br>

```rust
let machine_kind = if cfg!(unix) {
  "unix"
} else if cfg!(windows) {
  "windows"
} else {
  "unknown"
};

println!("I'm running on a {} machine!", machine_kind);
```