# Table of contents
- [Table of contents](#table-of-contents)
- [Formatted print](#formatted-print)
  - [Grammar](#grammar)
  - [Formatting parameters](#formatting-parameters)
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
- [Raw string literals](#raw-string-literals)
- [Formatting values](#formatting-values)
  - [Formatting text values](#formatting-text-values)
  - [Formatting numbers](#formatting-numbers)
  - [Dynamic widths and precisions](#dynamic-widths-and-precisions)
- [Using `std::fmt::Arguments` type and `format_args!` macro](#using-stdfmtarguments-type-and-format_args-macro)

<br>

# Formatted print
## Grammar
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

## Formatting parameters
`int|identifier:[[fill]align][sign]['#']['0'][width]['.' precision]type`.<br>

<br>

### `fill`
`fill` parameter sets the **padding character**, by default `space`.<br>

> **Note**:<br>
> if the **value** being formatted is smaller than `width` then **padding character** will be printed around it.<br>
> `fill` can be set only if `align` is set, `fill` **can't** be **alone**.<br>
> `fill` is **not sign-aware**.

<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:e>5}", -1);`|`eee-1`|`fill` is **not** **sign-aware**.|

<br>

### `align`
`align` parameter sets **alignment** for value inside field of width `width`.<br>
**By default** `align` is **left**-aligned for **non-numerics** and **right**-alignment for **numeric**.<br>

Variants:
- `>` right;
- `<` left;
- `^` middle;

<br>

### `sign`
`sign` parameter is used **only** for **numeric** types and can take following values: `+` or `-` and indicates that the correct **sign** `+` or `-` should **always** be printed.<br>
**By default** only the **negative sign** (`-`) of signed values is printed.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:+5}", 1);`|`   +1`|Forces to print `+`.|

<br>

### `#`
`#` parameter indicates that the **alternate** form of printing should be used.<br>
The alternate forms are:
- `#?` **pretty-print** for `Debug` formatting trait;
- `#x` **precedes** the value with a `0x`;
- `#X` **precedes** the value with a `0x`;
- `#b` **precedes** the value with a `0b`;
- `#o` **precedes** the value with a `0o`;

<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:+#010x}", 1);`|`+0x0000001`||
|`println!("{:+#010x}", -1);`|`+0xffffffff`|Non decimal are treated as positive.|
|`println!("{:+#010}", -1);`|`-000000001`|`-` is used for decimal.|

<br>

### `0`
`0` parameter is used for integer and sets the padding character to `0` and makes padding **sign-aware**.<br>
**sign-aware** means that **padding zeros** are always placed **after** the *sign* and **before** the *digits*.<br>
`0` parameter **ignores** both `fill` and `align`.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:5}", -1);`|`   -1`|By default uses `space` as **padding character**.|
|`println!("{:05}", -1);`|`-0001`|Ues `0` as **padding character**, sign at the most left position.|
|`println!("{:d>05}", -1);`|`-0001`|Ignores `d>`.|
|`println!("{:#010x}", 1);`|`0x00000001`||

<br>

### `width`
`width` parameter sets **minimum width** for printed value.<br>
If width of value **isn't enough** then the padding specified by `fill`/`alignment` *formatting parameters* will be used.<br>
The **value** for the `width` can also be provided as a `usize` in the **list of parameters** by adding a postfix `$`.<br>

|Example|Output|Explanation|
|:------|:-----|:----------|
|`println!("{:5}!", "A");`|`A    !`|Here `width` is hardcoded into **format string** and it's value is `5`.|
|`println!("{:1$}!", "B", 3);`|`B  !`|Here `width` is passed through **positional argument** number `1` and it's value is `3`.|
|`println!("{:width$}!", "C", width=8);`|`C       !`|Here `width` is passed through **named argument** `width` and it's value is `8`.|

<br>

### `type`
`type` parameter defines appropriate **formatting trait** to use.

<br>

## Formatting traits
Mapping between allowed values of `type` and **formatting traits**:
- **no value** is specified ⇒ `Display` trait
- `?` ⇒ `Debug` trait, it can be combined with others;
- `#?` ⇒ the `#` character requests **pretty-print** for `Debug` trait;
- *Debugging formatting* prints numbers **in decimal**, but you can put an `x` or `X` **before** `?` to **request** hexadecimal: `x?` or `X?`:
  - p`rintln!("hex: {:x?}", [9, 15, 240]);` prints ``
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
|`format!`|Writes *formatted text* to `String`.|
|`print!`|Writes *formatted text* to `io::stdout`.|
|`println!`|Same as `print!` but appends a **newline**.|
|`eprint!`|Writes *formatted text* to `io::stderr`.|
|`eprintln!`|Same as `eprint!` but appends a **newline**.|
|`write!`|Writes *formatted text* to **destination** which implements `&mut std::fmt::Write` or `&mut std::io::Write`.|
|`writeln!`|Same as `write!` but appends a **newline**.|
|`format_args!`|Compiler **built-in** macro. It **avoids heap allocations**.|

<br>

All these macros take one **string literal** and **zero** or **more arguemnts** and produce **formatted text**:
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

# Raw string literals
Sometimes you end up using too many escape characters and just want Rust to print a string as you see it. To do this, you can add `r#` to the beginning and `#` to the end. The
`r` here stands for **raw**:
```rust
println!(r#"He said, "A =  "B";"#);
```

But what if `#` marks the end of the string and you need to print text with a `#"` inside? In that case, you can start with `r##` and end with `##`.<br>
You can **keep adding** `#` to the beginning and end if you have longer instances of the `#` symbol in your text.<br>

So when you add a `b` to print as follows,
```rust
fn main() {
    println!("{:?}", b"This will look like numbers");
}
```
you will get an output that shows all the bytes:
```bash
[84, 104, 105, 115, 32, 119, 105, 108, 108, 32, 108, 111, 111, 107, 32, 108,
105, 107, 101, 32, 110, 117, 109, 98, 101, 114, 115]
```

<br>

You can also put `b` and `r` **together** if you need to:
```rust
fn main() {
    println!("{:?}", br##"I like to write "#"."##);
}
```

<br>

# Formatting values
The **formatting parameters** have the form `{which:how}`:
- both `which` and `how` are **optional** and `{}` is valid;
- `which` value **selects argument**, **by index** if it is `number` or **by name** if it is `identifier`;
- `how` value says how the argument should be formatted;
  - if `how` is present, the **colon** `:` before it is **required**;

<br>

## Formatting text values
When **formatting argument** has a **textual type** like `&str` or `String` the `how` value has the following parts, **all optional**: `[[padding]align][width]['.'max_length]`
- `padding`: a **padding character** to use, **by default** it is `space`;
- `align`, variants:
  - `>` right;
  - `<` left, **by default** for **textual type**;
  - `^` middle;
- `width`: **after** *any truncation*, if the argument is **shorter** than `width`, Rust **aligns** and **pads** it;
- `.max_length`: rust **truncates** argument if it is **longer** than `max_length`;

<br>

**Note**, Rust’s formatter has a naive understanding of width: it assumes **one character occupies one column**.<br>

<br>

**Example 1**:
```rust
fn main() {
    let s = "foobar";
    println!("{{}}: '{}'", s);
    println!("{{:3}}: '{:3}'", s);
    println!("{{:10}}: '{:10}'", s);
    println!("{{:2.4}}: '{:2.4}'", s);
    println!("{{:4.2}}: '{:4.2}'", s);
    println!("{{:3.10}}: '{:3.10}'", s);
    println!("{{:10.10}}: '{:10.10}'", s);
    println!("{{:*^4.2}}: '{:*^4.2}'", s);
}
```

**Output**:
```rust
{}: 'foobar'
{:3}: 'foobar'
{:10}: 'foobar    '
{:2.4}: 'foob'
{:4.2}: 'fo  '
{:3.10}: 'foobar'
{:10.10}: 'foobar    '
{:*^4.2}: '*fo*'
```

<br>

**Example 2**:
```rust
fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
```
**Output**:
```bash
---------TODAY'S NEWS---------
|                            |
SEOUL--------------------TOKYO
```

<br>

## Formatting numbers
When the **formatting argument** has a **numeric type** like `usize` or `f64`, the `how` value has the following parts, **all optional**: `[[padding]align][sign]['#']['0'][width]['.' precision]type`:
- `padding`: a **padding character** to use, **by default** it is `space`;
- `align`, variants:
  - `>` right, **by default** for **numeric type**;
  - `<` left
  - `^` middle;
- `+` **requests** that the number’s **sign always be shown**;
  - **by default** only the **negative sign** (`-`) of signed values is printed;
- `#` **requests** an **explicit radix prefix** like `0x` or `0b`;
- `0` it `0` parameter **ignores** both `fill` and `align` and sets the **padding character** to `0` for **numeric type** and makes padding **sign-aware**;
- `width`: **after** *any truncation*, if the argument is **shorter** than `width`, Rust **aligns** and **pads** it;
- `.precision`: for **floating-point** arguments, indicating **how many digits after the decimal point**;
- rust **truncates** argument if it is **longer** than `max_length`;
- `type` defines appropriate **formatting trait** to use;
  - for **integer types**, if you included the `#` character, *formatting trait* **also includes** an explicit **radix prefix**, `0b`, `0o`, `0x`, or `0X`;

<br>

If you have a **reference**, you can use `{:p}` to print the **pointer address**:<br>
**Code**:
```rust
fn main() {
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
}
```
**Output**:
```bash
0x16f3aa214
```

<br>

You can print **binary**, **hexadecimal**, and **octal**:<br>
**Code**:
```rust
fn main() {
    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);
}
```
**Output**:
```bash
Binary: 1000101011, hexadecimal: 22b, octal: 1053
```

<br>

**Examples**:
```rust
fn main() {
    let s = 555.999;
    println!("{{}}: '{}'", s);
    println!("{{:3}}: '{:3}'", s);
    println!("{{:10}}: '{:10}'", s);
    println!("{{:2.4}}: '{:2.4}'", s);
    println!("{{:4.2}}: '{:4.2}'", s);
    println!("{{:3.10}}: '{:3.10}'", s);
    println!("{{:10.10}}: '{:10.10}'", s);
    println!("{{:*^8.2}}: '{:*^8.2}'", s);

    println!("{{:*^+#010.2}}: '{:*^+#010.2}'", 555.999); // ignores *^

    println!("{{:-#010.2}}: '{:-#010.2}'", 777777);
    println!("{{:+#010.2}}: '{:+#010.2}'", 777777);

    println!("{{:+#12.2X}}: '{:+#12.2X}'", 777777);
    println!("{{:+#012.2X}}: '{:+#012.2X}'", 777777);

    println!("{{:+10.2X}}: '{:+10.2X}'", 777777);
    println!("{{:+012.2X}}: '{:+012.2X}'", 777777);

    println!("{{:+10.2x}}: '{:+10.2x}'", 777777);
    println!("{{:+012.2x}}: '{:+012.2x}'", 777777);

    println!("{{:+#10.2b}}: '{:+#10.2b}'", 77);
    println!("{{:+#024.2b}}: '{:+#024.2b}'", 77);

    println!("hex: {:?}", [9, 15, 240]);
    println!("hex: {:x?}", [9, 15, 240]);
    println!("hex: {:02x?}", [9, 15, 240]);
    println!("hex: {:#x?}", [9, 15, 240]);
    println!("hex: {:#02x?}", [9, 15, 240]);
    println!("hex: {:#04x?}", [9, 15, 240]);
}
```

**Output**:
```rust
{}: '555.999'
{:3}: '555.999'
{:10}: '   555.999'
{:2.4}: '555.9990'
{:4.2}: '556.00'
{:3.10}: '555.9990000000'
{:10.10}: '555.9990000000'
{:*^8.2}: '*556.00*'
{:*^+#010.2}: '+000556.00'
{:-#010.2}: '0000777777'
{:+#010.2}: '+000777777'
{:+#12.2X}: '    +0xBDE31'
{:+#012.2X}: '+0x0000BDE31'
{:+10.2X}: '    +BDE31'
{:+012.2X}: '+000000BDE31'
{:+10.2x}: '    +bde31'
{:+012.2x}: '+000000bde31'
{:+#10.2b}: '+0b1001101'
{:+#024.2b}: '+0b000000000000001001101'
hex: [9, 15, 240]
hex: [9, f, f0]
hex: [09, 0f, f0]
hex: [
    0x9,
    0xf,
    0xf0,
]
hex: [
    0x9,
    0xf,
    0xf0,
]
hex: [
    0x09,
    0x0f,
    0xf0,
]
```

<br>

## Dynamic widths and precisions
The `width`, `.max_length` and `precision` can be specified **at run time**: `<number>$` or `<name>$`. Also, in place of the text `max_length` or floating-point `precision`, you can write `*`:
```rust
fn main() {
    println!("{{:>width$.limit$}}: '{:>width$.limit$}'", "foobar", width=10, limit=4);
    println!("{{:>2$.1$}}: '{:>2$.1$}'", "foobar", 4, 10);
    println!("{{:>0$.2$}}: '{2:>1$.*}'", 4, 10, "foobar");
}
```

<br>

# Using `std::fmt::Arguments` type and `format_args!` macro
The `std::format_args!` is a compiler built-in macro. It **avoids heap allocations**.<br>
The `std::format_args!` produces a value of type `fmt::Arguments`.<br>

If some function receives `std::fmt::Arguments` you can pass `format_args!()` to it. How it works:
- **at compile time**, the `format_args!()` macro **parses** the *template string* and **checks** it against the **arguments’ types**, reporting an error if there are any problems;
- **at run time**, it evaluates the arguments and builds an `Arguments` value carrying all the information necessary to format the text: a **pre-parsed** *form of the template* and **references** *to the argument values*;

<br>

**Note**, constructing `Arguments` value is **cheap**: it’s just gathering up some pointers.<br>

<br>

A value of `fmt::Arguments` can be passed to the following functions:
- [fmt::format()](https://doc.rust-lang.org/std/fmt/fn.format.html): takes `fmt::Arguments` and **returns** the **resulting formated string** as `String`;
- [fmt::write()](https://doc.rust-lang.org/std/fmt/fn.write.html): takes **destination** (some type that implements `&mut std::fmt::Write` or `&mut std::io::Write`) and `fmt::Arguments` and **writes** the **resulting formated string** to **destination**;

<br>

**Notes**:
1. Use `write!` instead  `fmt::write()`.
2. Use `format!` instead `fmt::format()`.

<br>

The `File` type implements the `std::io::Write` trait, whose `write_fmt` method takes an `std::fmt::Arguments` and does the formatting:
```rust
use std::io::prelude::*;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let mut buffer = File::create("foo.txt")?;

    // this call: write!(buffer, "{} {}", 10, 20)?; turns into this:
    buffer.write_fmt(format_args!("{} {}", 10, 20))?;

    Ok(())
}
```
