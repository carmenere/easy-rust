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
- [`std::format_args`](#stdformat_args)

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
`int|identifier:[[fill]align][sign]['#']['0'][width]['.' precision]type`.

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
By default `align` is **left**-aligned for **non-numerics** and **right**-alignment for **numeric**.<br>

Variants:
- `>` right;
- `<` left;
- `^` middle;

<br>

### `sign`
`sign` parameter is used **only** for **numeric** types and can take following values: `+` or `-` and indicates that the correct **sign** `+` or `-` should **always** be printed. By default only the **negative sign** (`-`) of signed values is printed.<br>

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

# `std::format_args`
The `std::format_args` is a compiler built-in macro. It **avoids heap allocations**.<br>
The `std::format_args` produces a value of type `fmt::Arguments`.<br>

A value of `fmt::Arguments` can be passed to the following functions:
- [fmt::format()](https://doc.rust-lang.org/std/fmt/fn.format.html): takes `fmt::Arguments` and **returns** the **resulting formated string** as `String`;
- [fmt::write()](https://doc.rust-lang.org/std/fmt/fn.write.html): takes **destination** (some type that implements `&mut std::fmt::Write` or `&mut std::io::Write`) and `fmt::Arguments` and **writes** the **resulting formated string** to **destination**;

<br>

**Notes**:
1. Use `write!` instead  `fmt::write()`.
2. Use `format!` instead `fmt::format()`.
