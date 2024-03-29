# Table of contents
- [Table of contents](#table-of-contents)
- [Formatted print](#formatted-print)
  - [`fill`](#fill)
  - [`align`](#align)
  - [`sign`](#sign)
  - [`#`](#)
  - [`0`](#0)
  - [`width`](#width)
  - [`type`](#type)
- [Formatting traits](#formatting-traits)
- [Related macros](#related-macros)
  - [Example](#example)
  - [`format!`](#format)
    - [Example](#example-1)
  - [`write!` and `writeln!`](#write-and-writeln)
    - [`std::fmt::Write`](#stdfmtwrite)
    - [`std::io::Write`](#stdiowrite)
  - [`print!` and `println!`](#print-and-println)
  - [`format_args!`](#format_args)
    - [std::fmt::write](#stdfmtwrite-1)
      - [Examples](#examples)
    - [std::fmt::format](#stdfmtformat)
      - [Examples](#examples-1)
- [Implementations in `std`](#implementations-in-std)
  - [`impl Write for &mut [u8]`](#impl-write-for-mut-u8)
    - [Methods](#methods)
  - [`impl<A: Allocator> Write for Vec<u8, A>`](#impla-allocator-write-for-vecu8-a)
    - [Methods](#methods-1)
  - [`impl Write for String`](#impl-write-for-string)
    - [Methods](#methods-2)

<br>

# Formatted print
The actual grammar for the **formatting syntax** is:
```bash
format_string := text [ maybe_format text ] *
maybe_format := '{' '{' | '}' '}' | format
format := '{' [ argument ] [ ':' format_spec ] [ ws ] * '}'
argument := integer | identifier

format_spec := [[fill]align][sign]['#']['0'][width]['.' precision]type
fill := character
align := '<' | '^' | '>'
sign := '+' | '-'
width := count
precision := count | '*'
type := '' | '?' | 'x?' | 'X?' | identifier
count := parameter | integer
parameter := argument '$'
```

<br>

## `fill`
`fill` parameter sets the **padding character**.<br>
If the **value** being formatted is smaller than `width` then **padding character** will be printed around it.<br>
Defaults is a `space` (for **numeric** and **non-numerics**).<br>

> **Note**:<br>
> `fill` can be set only if `align` is set, `fill` **can't** be **alone**.
> `fill` is **not** **sign-aware**.

<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:e>5}", -1);`|`eee-1`|`fill` is **not** **sign-aware**.|

<br>

## `align`
`align` sets **alignment** for value inside field of with `width`.<br>
Defaults:
- **left**-aligned for **non-numerics**;
- **right**-alignment for **numeric**;

<br>

## `sign`
`sign` is used for numeric types and can take following values: `+` or `-`.<br>
The `+` value indicates that the correct **sign** `+` or `-` should **always** be printed. By default only the **negative sign** (`-`) of signed values is printed.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:+5}", 1);`|`   +1`|Forces to print `+`.|

<br>

## `#`
`#` parameter indicates that the **alternate** form of printing should be used.<br>
The alternate forms are:
- `#?` **pretty-print** for `Debug` formatting trait;
- `#x` **precedes** the value with a `0x`;
- `#X` **precedes** the value with a `0x`;
- `#b` **precedes** the value with a `0b`;
- `#o` **precedes** the value with a `0o`;

<br>

## `0`
`0` parameter is used for integer formats and sets the padding character to `0` and makes padding **sign-aware**.<br>
**sign-aware** means that **padding zeros** are always placed **after** the `sign` (if any) and **before** the digits.<br>
`0` parameter **ignores** both `fill` and `align`.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:5}", -1);`|`   -1`|By default uses `space` as **padding character**.|
|`println!("{:05}", -1);`|`-0001`|Ues `0` as **padding character**, sign at the most left position.|
|`println!("{:d>05}", -1);`|`-0001`|Ignores `d>`.|
|`println!("{:#010x}", 1);`|`0x00000001`||

<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:+#010x}", 1);`|`+0x0000001`||
|`println!("{:+#010x}", -1);`|`+0xffffffff`|Non decimal are treated as positive.|
|`println!("{:+#010}", -1);`|`-000000001`|`-` is used for decimal.|

<br>

## `width`
`width` parameter sets **minimum width** for appropriate value.<br>
If width of value **isn't enough** then the padding specified by `fill`/`alignment` **formatting parameters** will be used.<br>
The **value** for the `width` can also be provided as a `usize` in the **list of parameters** by adding a postfix `$`.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:5}!", "A");`|`A    !`|Here `width` is hardcoded into **format string** and it's value is `5`.|
|`println!("{:1$}!", "B", 3);`|`B  !`|Here `width` is passed through **positional argument** number `1` and it's value is `3`.|
|`println!("{:width$}!", "C", width=8);`|`C       !`|Here `width` is passed through **named argument** `width` and it's value is `8`.|

<br>

## `type`
`type` parameter defines appropriate **formatting trait** to use for appropriate value of `type` **formatting parameters**.

<br>

# Formatting traits
Mapping between allowed values of `type` **formatting parameters** and **formatting traits**:
- **no value** is specified ()`*empty*`) ⇒ `Display` trait
- `?` ⇒ `Debug` trait
- `x?` ⇒ `Debug` trait with **lower-case** hexadecimal integers
- `X?` ⇒ `Debug` trait with **upper-case** hexadecimal integers
- `o` ⇒ `Octal` trait
- `x` ⇒ `LowerHex` trait
- `X` ⇒ `UpperHex` trait
- `p` ⇒ `Pointer` trait
- `b` ⇒ `Binary` trait
- `e` ⇒ `LowerExp` trait
- `E` ⇒ `UpperExp` trait

<br>

This means that any type of argument which implements the `fmt::Binary` trait can then be formatted with `{:b}`.

<br>

# Related macros
There are a number of **related macros** in the `std` **module**.<br>
|Macros|Description|
|:-----|:----------|
|`format!`|Writes **formatted text** to `String`.|
|`print!`|Same as `format!` but the text is printed to `io::stdout`.|
|`println!`|Same as `print!` but appends a **newline**.|
|`eprint!`|Same as `print!` but the text is printed to `io::stderr`.|
|`eprintln!`|Same as `eprint!` but appends a **newline**.|
|`write!`|First argument is the **destination** and it can be either a `&mut io::Write` or a `&mut fmt::Write`.|
|`writeln!`|Same as `write!` but appends a **newline**.|
|`format_args!`|**Compiler built-in macros**. `format_args!`, unlike its derived macros, **avoids heap allocations**.|

<br>

## Example
```rust
use std::{io::{Write as _}, fmt::Write as _};

fn main() {
    let mut v: Vec<u8> = Vec::new();
    write!(&mut v, "Hello {}!", "world");      // expand to (&mut buf[..]).write_fmt(format_args!("Hello {0}!", "world"));
    println!("v = {:?}", v);                   // Vec<u8> doesn't implement std::fmt::Display

    let mut buf:[u8; 16] = [0; 16];
    write!(&mut buf[..], "Hello {}!", "world"); // expand to (&mut buf[..]).write_fmt(format_args!("Hello {0}!", "world"));
    println!("buf= {:?}", buf);                 // [u8; 16] doesn't implement std::fmt::Display

    let mut s: String = String::with_capacity(16);
    write!(&mut s, "Hello {}!", "string");
    println!("String = {}", s);

    std::fmt::write(&mut s, format_args!("{}", std::str::from_utf8(&buf[..12]).unwrap()));
    println!("String = {}", s);
}
```

<br>

**Output**:
```bash
v = [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33]
buf= [72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33, 0, 0, 0, 0]
String = Hello string!
String = Hello string!Hello world!
```

<br>

## `format!`
`format!` takes **string literal** and zero or more arguemnts and writes **formatted text** to `String`:
```rust
let s: String = format!("Hello, {}!", "world");
```

<br>

`format!` supports different **specifiers** for arguments:
|Specifier|Example|Output|
|:------|:------|:-----|
|**Positional argument**|`println!("{0} and {1}.", "a", "b");`|`a and b.`|
|**Named argument**|`println!("{a} and {b}.", a="1", b="2");`|`1 and 2.`|
|**Next argument**|`println!("{} and {}.", "x", 22);`|`x and 22.`|
|**Intermingled**|`println!("{x} and {y}; {}; {2}", 44, x="x", y=22);`|`x and 22; 44; 22`|

<br>

The **next argument** specifier can be thought of as an **iterator over the argument**: each time a **next argument** specifier (`{}`) is seen, the iterator advances.<br>

<br>

### Example
```rust
println!("{name} {} {} {}", 1, 2, name = "name");
```
prints
```rust
name 1 2 name
```

<br>

## `write!` and `writeln!`
`write!` and `writeln!` are two macros which are used to emit the format string to a specified **stream**.<br>
This is used to **prevent intermediate allocations of format strings** and instead directly write the output.<br>
Under the hood, this function is actually invoking the `write_fmt` function defined on the `std::io::Write` and the `std::fmt::Write` trait.<br>

The 
```rust
write!(buffer, "{:.*}", 2, 1.234567)?;
```
**turns into**
```rust
buffer.write_fmt(format_args!("{:.*}", 2, 1.234567))?;
```

<br>

### `std::fmt::Write`
This trait only accepts **UTF-8–encoded** data and is **not flushable**.<br>
If you only want to accept Unicode and you don’t need flushing, you should implement this trait; otherwise you should implement `std::io::Write`.<br>

[std::fmt::Write](https://doc.rust-lang.org/std/fmt/trait.Write.html)

<br>

### `std::io::Write`
A trait **for** objects which are **byte-oriented sinks**.<br>
Implementors of the `Write` trait are sometimes called **writers**.<br>

[std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html)

Example:
```rust
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create("foo.txt")?);

    buffer.write_all(b"some bytes")?;
    buffer.flush()?;
    Ok(())
}
```

<br>

The `write` method will attempt to write some data into the object, returning how many bytes were successfully written.<br>
If `write` method consumed `n > 0` bytes of `buf` it must return `Ok(n)`. If the return value is `Ok(n)` then `n` must satisfy `n <= buf.len()`.<br>
The trait also provides convenience methods like `write_all`, which calls write in a loop until its entire input has been written.<br>

<br>

## `print!` and `println!`
`print!` and `println!` emit their output to `stdout`.<br>
Similarly to the `write!` macro, the goal of these macros is to **avoid intermediate allocations when printing output**.

<br>

## `format_args!`
**Compiler built-in macros**. `format_args!`, unlike its derived macros, **avoids heap allocations**.<br>
The result of the `format_args!` macro is a value of type `fmt::Arguments`.<br>
A value of type `fmt::Arguments` can be passed to the `std::fmt::format` and `std::fmt::write` functions in order to process the **format string**. 

<br>

### std::fmt::write
The `std::fmt::write` function takes an **output stream** and an `Arguments` struct.<br>
The `Arguments` struct that can be **precompiled** with the `format_args!` macro.<br>

> **Note**:<br>
> Using `write!` might be **preferable**.<br>

<br>

#### Examples
```rust
use std::fmt;

let mut output = String::new();
fmt::write(&mut output, format_args!("Hello {}!", "world"))
    .expect("Error occurred while trying to write in String");
assert_eq!(output, "Hello world!");
```

```rust
use std::fmt::Write;

let mut output = String::new();
write!(&mut output, "Hello {}!", "world")
    .expect("Error occurred while trying to write in String");
assert_eq!(output, "Hello world!");
```

<br>

### std::fmt::format
The `std::fmt::format` function takes an `Arguments` struct and **returns the resulting formatted string**.<br>
The `Arguments` struct that can be **precompiled** with the `format_args!` macro.<br>

> **Note**:<br>
> Using `format!` might be **preferable**.<br>

<br>

#### Examples
```rust
use std::fmt;

let s = fmt::format(format_args!("Hello, {}!", "world"));
assert_eq!(s, "Hello, world!");
```

```rust
let s = format!("Hello, {}!", "world");
assert_eq!(s, "Hello, world!");
```

<br>

# Implementations in `std`
## `impl Write for &mut [u8]`
`std::io::Write` is implemented for `&mut [u8]` by copying **into** the slice, overwriting its data.<br>
Note that **writing updates the slice**: the slice will be **empty** when it has been completely overwritten.<br>

If the number of bytes to be written **exceeds** the size of the slice:
- `write` returns `Ok(0)`;
- `write_all` returns an error of kind `ErrorKind::WriteZero`;

<br>

### Methods
- `fn write(&mut self, data: &[u8]) -> Result<usize>`<br>
Write a **buffer** `data` into **this writer**, returning how many bytes were written.

<br>

- `fn write_all(&mut self, data: &[u8]) -> Result<()>`<br>
Attempts to write an entire buffer `data` into **this writer**.

<br>

- `fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>`<br>
Writes a **formatted string** `fmt` into **this writer**, returning any error encountered.

<br>

- `fn flush(&mut self) -> Result<()>`<br>
Flush this output stream, ensuring that all intermediately buffered contents reach their destination.

<br>

## `impl<A: Allocator> Write for Vec<u8, A>`
`std::io::Write` is implemented for `Vec<u8>` by appending to the vector. The vector will grow as needed.

<br>

### Methods
- `fn write(&mut self, buf: &[u8]) -> Result<usize>`<br>
Write a buffer `buf` into this writer, returning how many bytes were written.

<br>

- `fn write_all(&mut self, buf: &[u8]) -> Result<()>`<br>
Attempts to write an entire buffer `buf` into this writer.

<br>

- `fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<()>`<br>
Writes a **formatted string** `fmt` into this writer, returning any error encountered.

<br>

- `fn flush(&mut self) -> Result<()>`<br>
Flush this output stream, ensuring that all intermediately buffered contents reach their destination.

<br>

## `impl Write for String`
`std::fmt::Write` is implemented for `String`.

<br>

### Methods

- `fn write_str(&mut self, s: &str) -> Result<(), Error>`<br>
Writes a **string slice** `s` **into** this writer, returning whether the write succeeded. 

<br>

- `fn write_char(&mut self, c: char) -> Result<(), Error>`<br>
Writes a **char** `c` **into** this writer, returning whether the write succeeded.

<br>

- `fn write_fmt(&mut self, args: Arguments<'_>) -> Result<(), Error>`<br>
Glue for usage of the `write!` macro with implementors of this trait.
