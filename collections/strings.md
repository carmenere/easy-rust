# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Strings](#strings)
  * [String literals](#string-literals)
  * [`String`](#string)
    * [Examples](#examples)
  * [Methods](#methods)
  * [Bytes. Chars. Vec<u8>](#bytes-chars-vecu8)
* [Grapheme clusters](#grapheme-clusters)
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