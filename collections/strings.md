# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Strings](#strings)
  - [String literals](#string-literals)
  - [`String`](#string)
    - [Examples](#examples)
  - [Methods](#methods)
  - [Bytes. Chars. Vec](#bytes-chars-vec)
- [Grapheme clusters](#grapheme-clusters)
- [Char methods](#char-methods)
  - [Classifying characters](#classifying-characters)
  - [Case conversion for characters](#case-conversion-for-characters)
  - [Conversions to and from integers](#conversions-to-and-from-integers)
<!-- TOC -->

<br>

# Strings
Rust has 2 types for strings: `str` and `String`. Both `str` and `String` contain **Unicode characters** encoded with **UTF-8**.<br>
In other words, string in Rust is a **UTF-8** encoded sequence of bytes.<br>
**Unicode characters** encoded with **UTF-8** have **variable length** from **1** to **4** bytes.<br>
**UTF-8** is backward-compatible with **ASCII**. The first 128 UTF-8 characters precisely match the first 128 ASCII characters, meaning that existing ASCII text is already valid UTF-8.<br>

<br>

## String literals
[**More about string slices here**](../ownership-borrowing/slices.md).<br>
**String literals** are **statically allocated**, i.e., they are hardcoded into binary and exists while programme is running and have type `&'static str`.<br>

Example:
```Rust
let s: &str = "ABC";
```

<br>

Types of string literal:
- `"..."`	**string literal**, some special symbols:
    - `\n` becomes new line;
    - `\r`
    - `\t`
    - `\0`
    - `\\` becomes slash;
    - `\u{7fff}` becomes symbol;
- `r"..."` **raw string literal**, it **doesn't** interpret **common escapes**;
- `r#"..."#` **raw string literal** that can also contain `"`;
- `c"..."` **C string literal**, i.e. a **NUL-terminated** `&'static CStr` **for FFI**;
- `cr"..."` **raw C string literal**;
- `cr#"..."#` **raw C string literal** that can also contain `"`;
- `b"..."` **byte string literal**; it **constructs ASCII-only** `&'static [u8; N]`.
- `br"..."` **raw byte string literal**;
- `br#"..."#` **raw byte string literal** that can also contain `"`;
- `b'x'` **ASCII byte literal**, it is a **single u8 byte**;
- `'A'` **character literal**, it is **fixed 4 byte unicode char**;

<br>

## `String`
The `String` is a sequence that is allowed to **grow** or **shrink** *in size* **at runtime** and is provided by Rust's standard library.

### Examples
- Instantiating `String` variables by `String` **constructor** (`new()`):
```Rust
let s: String = String::new();
```
- Instantiating `String` variables from `&str` values:
```Rust
let s1: String = String::from("ABC");
```
```Rust
let s2: String = "ABC".to_string();
```

<br>

## Methods
|Method|Description|
|:-----|:----------|
|`.len()`|Returns **length** of string.|
|`.push('c')`|**Append** *one character* to string.|
|`.push_str("abc")`|**Append** *substring* to string.|
|`.replace(from, to)`|**Replace** *substring* `from` to substring `to`.|
|`.split(sep)`|Splits string by separator|

<br>

## Bytes. Chars. Vec<u8>
```rust
#![allow(unused_variables)]

fn main() {
    // String
    let s = String::from("你好");
    println!("Len of s: {}", s.len());
    let s = String::from_utf8("你好".as_bytes().to_vec()).unwrap();
    println!("Len of s: {}", s.len());

    // Vec<u8>
    let v: Vec<u8> = s.as_bytes().to_owned();
    println!("Len of Vec<u8>: {}", v.len());
    println!("Vec<u8>:");
    for item in v {
        println!("  {}", item);
    }

    // std::str::Bytes<'_>
    let bytes: std::str::Bytes<'_> = s.bytes();
    println!("Len of bytes: {}", bytes.len());
    println!("Bytes:");
    for b in bytes {
        println!("  {}", b);
    }

    // std::str::Chars<'_>
    let chars: std::str::Chars<'_> = s.chars();
    println!("Chars:");
    for ch in chars {
        println!("  {}", ch);
    }
}
```

<br>

# Grapheme clusters
A **grapheme cluster** is a sequence of one or more Unicode **code points** that should be treated as a **single unit**.<br>
Text editing software should generally allow placement of cursor only at grapheme cluster boundaries.<br>

<br>

There is **no** method in **std** to iterate over **grapheme clusters**.<br>
The [**unicode-segmentation**](https://crates.io/crates/unicode-segmentation) crate provides grapheme cluster. It provides special method `.graphemes(true)`.<br>

<br>

# Char methods
## Classifying characters
- `ch.is_numeric()` returns `true` if `ch` is a **numeric** *character*;
  - if you want to parse **ASCII decimal digits** (**0-9**) or **ASCII base-N**, use `is_ascii_digit` or `is_digit` instead;
- `ch.to_digit()`:
  - decides whether `ch` is a **digit** in **base** `radix`:
    - if it is, it returns `Some(num)`, where `num` is a `u32`;
    - otherwise, it returns `None`;
  - **panics** if given a radix smaller than **2** or larger than **36**;
- `ch.is_digit(radix)`
  - decides whether `ch` is a **digit** in **base** `radix`:
    - if it is, it returns `true`;
    - otherwise, it returns `false`;
  - this function only recognizes the characters **0-9**, **a-z** and **A-Z**, the valid set of characters is depending on `radix`;
  - **panics** if given a radix smaller than **2** or larger than **36**;
  - this is **equivalent** to `ch.to_digit(radix) != None`;
- `ch.is_alphabetic` returns `true` if `ch` is an **alphabetic** *character*;
- `ch.is_alphanumeric` returns `true` if `ch` either **numeric** or **alphabetic**;
- `ch.is_whitespace` returns `true` if `ch` is a **whitespace** *character*;
- `ch.is_lowercase()` returns `true` if `ch` is a **lowercase alphabetic** *character*;
- `ch.is_uppercase()` returns `true` if `ch` is a **uppercase alphabetic** *character*;
- `ch.is_control` returns `true` if `ch` is a **control** (**not printable**) *character*;
- `ch.is_ascii()` returns `true` if `ch` is an **ASCII** *character*, i.e. its *code point* falls between **0** and **127** inclusive;
- `ch.is_ascii_alphabetic()`
- `ch.is_ascii_alphanumeric()`
- `ch.is_ascii_control()`
- `ch.is_ascii_digit()`
- `ch.is_ascii_graphic()`
- `ch.is_ascii_hexdigit()`
- `ch.is_ascii_lowercase()`
- `ch.is_ascii_octdigit()`
- `ch.is_ascii_punctuation()`
- `ch.is_ascii_uppercase()`
- `ch.is_ascii_whitespace()`

<br>

**All** the `is_ascii_*` methods are also available on the `u8` type.<br>

<br>

- `ch.len_utf8()` returns **number of bytes** of char `ch` as if it would encoded in `UTF-8`;
- `escape_unicode()` returns an **iterator** that yields the **hexadecimal Unicode escape** `\u{NNNNNN}` a character `ch` as `char`s.<br>

<br>

## Case conversion for characters
- `ch.to_ascii_lowercase()`
- `ch.to_ascii_uppercase()`
- `to_lowercase()` returns **iterator** that produce the character of the **lowercase** equivalents of `ch`;
- `to_uppercase()` returns **iterator** that produce the character of the **uppercase** equivalents of `ch`;

<br>

## Conversions to and from integers
The `as` operator will convert a `char` to **any integer type**, but for types < `u32`/`i32` **upper bits are truncated**.<br>
```rust
assert_eq!('B' as u32, 66);
assert_eq!('饂' as u8, 66); // upper bits truncated
```

The `as` operator will convert any `u8` value to a `char`, and `char` implements `From<u8>` as well, but wider integer types can represent invalid code points, so for those you must use `std::char::from_u32`, which returns `Option<char>`:
```rust
assert_eq!(char::from(66),'B');
assert_eq!(std::char::from_u32(0x9942), Some('饂'));
```

