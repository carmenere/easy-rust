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
- [Type `char`](#type-char)
  - [Char methods](#char-methods)
    - [Handling digits](#handling-digits)
    - [Classifying characters](#classifying-characters)
  - [Get Unicode escapes](#get-unicode-escapes)
    - [Case conversion for characters](#case-conversion-for-characters)
    - [Conversions to and from integers](#conversions-to-and-from-integers)
- [`String` methods](#string-methods)
  - [Creating `String`](#creating-string)
  - [Get basic information from string slices](#get-basic-information-from-string-slices)
  - [appending and inserting text to a `String`](#appending-and-inserting-text-to-a-string)
  - [Removing and replacing text](#removing-and-replacing-text)
  - [Searching and replacing](#searching-and-replacing)
  - [Iterating over text](#iterating-over-text)
  - [Trimming](#trimming)
  - [Case conversion for strings](#case-conversion-for-strings)
  - [Parsing](#parsing)
    - [Converting from strings to other types](#converting-from-strings-to-other-types)
    - [Converting other types to strings](#converting-other-types-to-strings)
  - [UTF-8](#utf-8)
    - [Accessing text as UTF-8](#accessing-text-as-utf-8)
    - [Producing text from UTF-8 data](#producing-text-from-utf-8-data)
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

# Type `char`
## Char methods
### Handling digits
- `ch.to_digit(radix) -> u32`:
  - decides whether `ch` is a **single digit** in a **base** `radix`:
    - if it is, it returns `Some(num)`, where `num` is a `u32`;
    - otherwise, it returns `None`;
  - **panics** if given a radix smaller than **2** or larger than **36**;
- `std::char::from_digit(num, radix)`
  - converts the `u32` digit value `num` to a `char` if possible:
    - if `num` can be represented as a **single digit** in a **base** `radix`, `from_digit` returns `Some(ch)`;
    - otherwise, it returns `None`;

<br>

### Classifying characters
**Decimal digits** are the ten numerical symbols `0`, `1`, `2`, `3`, `4`, `5`, `6`, `7`, `8`, and `9` used in the **base-10** positional numeral system to represent all possible numbers.<br>

<br>

Unicode defines **3 general categories for numbers**:
- `Nd` or `Number, Decimal Digit`: covers characters that represent **digits** used in **decimal positional systems** (**base-10**), including 
  - standard **Latin** digits `0..=9`;
  - **Arabic-Indic** digits;
  - other **script-specific** decimal digits;
- `Nl` or `Number, Letter`: covers **letter-like characters** that represent numbers, for positional systems with **base** more than **10**;
- `No` or `Number; Other`: covers numbers that are not decimal digits or letters, e.g. fractions, and so on;

<br>

- `ch.is_numeric()` returns `true` if char `ch` belongs one of the Unicode general categories for numbers `Nd` or `Nl`, but **not** `No`;
- `ch.is_digit(radix)`
  - decides whether `ch` is a **single digit** in a **base** `radix`:
    - if it is, it returns `true`;
    - otherwise, it returns `false`;
  - this function only recognizes the characters **0-9**, **a-z** and **A-Z**, the valid set of characters is depending on `radix`;
  - **panics** if given a radix smaller than **2** or larger than **36**;
  - this is **equivalent** to `ch.to_digit(radix) != None`;

<br>

- `ch.is_alphabetic()` returns `true` for **alphabetic** *character*;
- `ch.is_alphanumeric()` returns `true` for **numeric** or **alphabetic**;
- `ch.is_control()` returns `true` for **control** (**not printable**) *character*;
- `ch.is_lowercase()` returns `true` for **lowercase alphabetic** *character*;
- `ch.is_uppercase()` returns `true` for **uppercase alphabetic** *character*;
- `ch.is_whitespace` returns `true` for **whitespace** *character*;

<br>

Also there is *set of methods* for **ASCII only** `char`, they return `false` for **any non-ASCII** `char`:
- `ch.is_ascii()` returns `true` if `ch` is an **ASCII** *character*, i.e. its *code point* falls between **0** and **127** inclusive;
- `ch.is_ascii_alphabetic()`
  - an **uppercase** or **lowercase ASCII letter**, i.e. **any** character in the **ranges** `A..=Z` or `a..=z`;
- `ch.is_ascii_alphanumeric()`
  - an **uppercase** or **lowercase ASCII letter**, i.e. **any** character in the **ranges** `0..=9`, `A..=Z` or `a..=z`;
- `ch.is_ascii_control()`
- `ch.is_ascii_digit()`
  - an **ASCII digit**, in the **range** `0..=9`;
- `ch.is_ascii_graphic()`
  - **any** ASCII character that **leaves ink** on the page;
- `ch.is_ascii_hexdigit()`
  - **any** character in the **ranges** `0..=9`, `A..=F`, or `a..=f`;
- `ch.is_ascii_lowercase()`
  - **any** ASCII **lowercase** letter;
- `ch.is_ascii_octdigit()`
- `ch.is_ascii_punctuation()`
- `ch.is_ascii_uppercase()`
  - **any** ASCII **uppercase** letter;
- `ch.is_ascii_whitespace()`

<br>

**Note**, **all** the `is_ascii_*` methods are also available on the `u8` type.<br>

<br>

- `ch.len_utf8()` returns **number of bytes** of char `ch` as if it would encoded in `UTF-8`;

<br>

## Get Unicode escapes
**Unicode escape** formats:
- **Fixed-length** formats represent the Unicode *code points* **with** *fixed padding*;
  - **4-digit hex** `\uXXXX`, it represents a **16-bit** *code points* and can **only** represent characters **up to** `\uFFFF`;
  - **8-digit hex** `\UXXXXXXXX`;
- **Braced hex** format `\u{X...X}` is **variable length**, i.e. represents the Unicode *code points* **without** *fixed padding*;

<br>

There is special method to get **Unicode escape**:
- `ch.escape_unicode()` returns an **iterator** that yields the **hexadecimal** *Unicode escape* in **braced hex** format `\u{NNNN}` for character `ch` as `char`s;

<br>

But you can manually print **any** *Unicode codepoint* in **any** *Unicode escape format*:<br>
**Code**:
```rust
fn main() {
  println!("\\u{:04X}", '행' as u32);
  println!("\\u{:04X}", 'H' as u32);
  println!("\\u{:04X}", '居' as u32);
  println!("\\u{:04X}", 'い' as u32);

  println!("\\U{:08X}", '행' as u32);
  println!("\\U{:08X}", 'H' as u32);
  println!("\\U{:08X}", '居' as u32);
  println!("\\U{:08X}", 'い' as u32);

  println!("\\u{{{:x}}}", '행' as u32);
  println!("\\u{{{:x}}}", 'H' as u32);
  println!("\\u{{{:x}}}", '居' as u32);
  println!("\\u{{{:x}}}", 'い' as u32);

  println!("{}", '행'.escape_unicode());
  println!("{}", 'H'.escape_unicode());
  println!("{}", '居'.escape_unicode());
  println!("{}", 'い'.escape_unicode());

  println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
  println!("\u{d589}, \u{48}, \u{5c45}, \u{3044}");
}
```
**Output**:
```bash
\uD589
\u0048
\u5C45
\u3044
\U0000D589
\U00000048
\U00005C45
\U00003044
\u{d589}
\u{48}
\u{5c45}
\u{3044}
\u{d589}
\u{48}
\u{5c45}
\u{3044}
행, H, 居, い
행, H, 居, い
```

<br>

### Case conversion for characters
- `ch.to_ascii_lowercase()`;
- `ch.to_ascii_uppercase()`;
- `to_lowercase()` returns **iterator** that produce the character of the **lowercase** equivalents of `ch`;
- `to_uppercase()` returns **iterator** that produce the character of the **uppercase** equivalents of `ch`;

<br>

### Conversions to and from integers
The `as` operator will convert a `char` to **any integer type**, but for types < `u32`/`i32` **upper bits are truncated**:
```rust
assert_eq!('B' as u32, 66);
assert_eq!('饂' as u8, 66); // upper bits truncated
```

The `as` operator will convert **any** `u8` value to a `char`, and `char` implements `From<u8>` as well.<br>
But **wider** integer types can represent **invalid** *code points*, so for those you must use `std::char::from_u32()`, which returns `Option<char>`:
```rust
assert_eq!(char::from(66),'B');
assert_eq!(std::char::from_u32(0x9942), Some('饂'));
```

<br>

# `String` methods
**Name convension**:
- `ch`: `char`
- `string`: `String`
- `slice`: `&str` or something that **dereferences** to `&str`, for example `String`;
- `pattern`: anythong that is **pattern type**: `char`, `String`, `&str`, `&[char]`, **closure as pattern** `FnMut(char) -> bool`

<br>

In the library’s own code, a **pattern** is any type that implements the `std::str::Pattern` trait. The details of `Pattern` are **not** **yet** **stable**, so you **can’t implement** it for
your own types in stable Rust.<br>

<br>

**Example** of using **closure as pattern**:
```rust
assert!("2017".starts_with(char::is_numeric));
```

<br>

## Creating `String`
- `String::new()`
  - returns a new, **empty** string;
  - this **has no** *heap-allocated buﬀer*, but will allocate one as needed;
- `String::with_capacity(n)`
  - returns a new, **empty** string with a *heap-allocated buﬀer* to hold **at least** `n` bytes;
- `str_slice.to_string()`
  - allocates a new `String` whose contents are a copy of `str_slice`;
- `iter.collect()`
- `slice.to_owned()`
  - allocates a new `String` whose contents are a copy of `slice`;

<br>

A **string slice** can produce an iterator over its characters with `slice.chars()`.<br>

<br>

## Get basic information from string slices
- `slice.len()`
  - returns the **length** of `slice`, **in bytes**;
- `slice.is_empty()`
  - returns `true` if `slice.len() == 0`;
- `slice.is_char_boundary(i)`
  - `true` if the **byte oﬀset** `i` falls **between character**;
- `slice.split_at(i)`
  - returns a **tuple of two shared slices** `(slice[..i], slice[i..])` borrowed from `slice`;

<br>

## appending and inserting text to a `String`
- `string.push(ch)`
  - **appends** the single character `ch` to the **end** of `string`;
- `string.push_str(slice)`
  - **appends** the full contents of `slice` to the **end** of `string`;
- `string.extend(iter)`
  - **appends** the items produced by the **iterator** `iter` to the **end** of `string`;
  - the **iterator** `iter` can produce `char`, `str`, or `String` values;
- `string.insert(i, ch)`
  - **inserts** the single character `ch` **at byte oﬀset** `i` in `string`;
  - this **entails shifting** over any characters **after** `i` to make room for `ch`;
- `string.insert_str(i, slice)`
  - **inserts** the full contents of `slice` **at byte oﬀset** `i` in `string`;
  - this **entails shifting** over any characters **after** `i` to make room for `ch`;

<br>

String implements std::fmt::Write, meaning that the `write!` and `writeln!` macros can append formatted text to `String`.
**Note**, `write!` and `writeln!` are designed for writing to output streams, they return a `Result`, which Rust complains if you ignore. But writing to a `String` is actually **infallible**.

<br>

## Removing and replacing text
- `string.clear()`
  - makes `string` **empty**;
- `string.truncate(n)`
  - **discards** all characters **after** the **byte oﬀset** `n`;
  - if `string` is **shorter** than `n` bytes, this has **no eﬀect**;
- `string.pop()`
  - **removes** the **last character** from `string`, if any, and returns it as an `Option<char>`;
- `string.remove(i)`
  - **removes** the character **at byte oﬀset** `i` from `string` and returns it, **shifting** any following characters toward the front;
- `string.drain(range)`
  - returns an **iterator** over the given range of byte indices `range` and **removes** the characters once the iterator is dropped;
- `string.replace_range(range, slice)`
  - replaces the given range `range` in `string` with the given replacement slice `slice`;
  - but if the `range` being replaced doesn't go to the end of `string`, that will require moving all remainig bytes **after** the end of the range;

<br>

**Examples**:
```rust
let mut choco = "chocolate".to_string();
assert_eq!(choco.drain(3..6).collect::<String>(),);
assert_eq!(choco,"choate");
```

<br>

## Searching and replacing
- `slice.contains(pattern)`
  - returns `true` if `slice` contains a **match** for `pattern`;
- `slice.starts_with(pattern)`
  - returns `true` if `slice` contains a **match** for `pattern` **at the begining**;
- `slice.ends_with(pattern)`
  - returns `true` if `slice` contains a **match** for `pattern` **at the end**;
- `slice.find(pattern)`
  - returns `Some(i)` if slice contains the **first match** for `pattern`;
    - where `i` is the **byte oﬀset** at which the `pattern` appears;
- `slice.rfind(pattern)`
  - returns `Some(i)` if slice contains the **last match** for `pattern`;
    - where `i` is the **byte oﬀset** at which the `pattern` appears;
- `slice.replace(pattern, replacement)`
  - returns a **new** `String` formed by replacing **all matches** for `pattern` with `replacement` slice;
- `slice.replacen(pattern, replacement, N)`
  - returns a **new** `String` formed by replacing **at most** the **first** `N` **matches** for `pattern` with `replacement` slice;

<br>

**Examples**:
```rust
fn main() {
    let quip = "We also know there are known unknowns";

    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));
    assert_eq!(quip.find("ya know"), None);
    assert_eq!(quip.rfind(char::is_uppercase), Some(0));

    // cuts all non is_alphanumeric characters
    assert_eq!("`Borrow` and `BorrowMut`".replace(|ch:char| !ch.is_alphanumeric(),""), "BorrowandBorrowMut");
}
```

<br>

The `.replace()`’s behavior on **overlapping matches** can be surprising. Consider string `"cabababababbage"`, it contains **4** instances of the pattern, `"aba"`, but the *second* and *fourth* **no longer match after** the *first* and *third* are replaced:
```rust
assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");
```

<br>

## Iterating over text
- `slice.chars()`
  - returns an **iterator over** `slice`’s **characters**;
- `slice.char_indices()`
  - returns an **iterator over tuples** `(i, ch)`, where `ch` is an next `slice`’s character and `i` its **byte oﬀset**;
  - **note**, this is **not equivalent** to `.chars().enumerate()`;
- `slice.bytes()`
  - returns an **iterator over** the **individual bytes** of `slice`
- `slice.lines()`
  - returns an **iterator over the lines** of `slice`;
  - **lines** are terminated by `\n` or `\r\n` and they **do not** include the terminating characters;
  - **each line** is a `&str` from slice;
- `slice.split(pattern)`
  - returns an **iterator over the portions** of `slice` separated by **matches** of `pattern`;
  - this produces **empty strings** it there is no character between **2** `pattern`s;
  - this produces **empty strings** for **matches** *at the very beginning* and *at the very end* of `slice`;
- `slice.rsplit(pattern)`
  - this is like the `split(pattern)`, **but** scans `slice` **from end to start**, producing matches **in that order**;
- `slice.split_terminator(pattern)`
  - if `pattern` matches **at the very end** of `slice`, it do not produce an **empty strings**
- `slice.rsplit_terminator(pattern)`
  - this is like the `split_terminator(pattern)`, **but** scans `slice` **from end to start**, producing matches **in that order**;
- `slice.splitn(n, pattern)`
  - this is like the `split(pattern)`, but **splits** the string into **at most** `n` slices;
- `slice.rsplitn(n, pattern)`
  - this is like the `rsplit(pattern)`, but **splits** the string into **at most** `n` slices;
- `slice.split_whitespace()`
  - returns an **iterator over the portions** of `slice` separated by *whitespace characters*;
  - **note**:
    - **adjacent multiple** *whitespace characters* are considered a **single separator**;
    - **trailing** *whitespace* is **ignored**;
- `slice.split_ascii_whitespace()`
  - this is like the `split_whitespace(pattern)`, but **recognizes only ASCII** *whitespace characters*;
- `slice.matches(pattern)`
  - returns an **iterator over the matches** for `pattern` in `slice`;
- `slice.rmatches(pattern)`
  - this is like the `matches(pattern)`, **but** scans `slice` **from end to start**, producing matches **in that order**;
- `slice.match_indices(pattern)`
  - this is like the `matches(pattern)`, **but** producs `(offset, match)` **pairs**, where `offset` is the **byte oﬀset** at which the **match begins**;
- `slice.rmatch_indices(pattern)`
  - this is like the `rmatches(pattern)`, **but** producs `(offset, match)` **pairs**, where `offset` is the **byte oﬀset** at which the **match begins**;

<br>

**Example**:
```rust
fn main() {
    assert_eq!("foo::bar".split(':').collect::<Vec<_>>(), vec!["foo", "", "bar"]);
    assert_eq!("::foo:bar::".split(':').collect::<Vec<_>>(), vec!["", "", "foo", "bar", "", ""]);
    assert_eq!("::foo:bar::".split_terminator(':').collect::<Vec<_>>(), vec!["", "", "foo", "bar", ""]);

    assert_eq!("foo::bar".rsplit(':').collect::<Vec<_>>(), vec!["bar", "", "foo"]);
    assert_eq!("::foo:bar::".rsplit(':').collect::<Vec<_>>(), vec!["", "", "bar", "foo", "", ""]);
    assert_eq!("::foo:bar::".rsplit_terminator(':').collect::<Vec<_>>(), vec!["", "bar", "foo", "", ""]);

    assert_eq!("    foo  bar    ".split_whitespace().collect::<Vec<_>>(), vec!["foo", "bar"]);

    assert_eq!("::foo:bar::".matches(':').collect::<Vec<_>>(), vec![":", ":", ":", ":", ":"]);
}
```

<br>

## Trimming
To **trim** a string is to **remove text**, *usually whitespace*, *from* the **beginning** or **end** of the string.<br>

<br>

- `slice.trim()`
  - returns a **subslice of slice** that *omits* **all leading** and **all trailing** *whitespaces*;
  - `slice.trim_start()`
    - *omits* **only all leading** *whitespaces*;
  - `slice.trim_end()`
    - *omits* **only all trailing** *whitespaces*;
- `slice.trim_matches(pattern)`
  - returns a **subslice of slice** that *omits* **all leading** and **all trailing** matches of  `pattern`;
  - `slice.trim_start_matches(pattern)`
    - *omits* **only all leading** matches of  `pattern`;
  - `slice.trim_end_matches(pattern)`
    - *omits* **only all trailing** matches of `pattern`;

<br>

**Example**:
```rust
fn main() {
    assert_eq!("0010990".trim_matches('0'), "1099");
    assert_eq!("0010990".trim_start_matches('0'), "10990");
    assert_eq!("0010990".trim_end_matches('0'), "001099");
}
```

<br>

## Case conversion for strings
- `slice.to_uppercase()`
- `slice.to_lowercase()`

<br>

## Parsing
### Converting from strings to other types
If a type implements the std::str::FromStr trait, then it provides a standard way to parse a value from a string slice:
```rust
pub trait FromStr: Sized {
  type Err;
  fn from_str(s: &str) -> Result<Self, Self::Err>;
}
```

<br>

All the usual machine types implement `FromStr`:
```rust
use std::net::IpAddr;
use std::str::FromStr;

fn main() {
    assert_eq!(usize::from_str("3628800"), Ok(3628800));
    assert_eq!(f64::from_str("128.5625"), Ok(128.5625));
    assert_eq!(bool::from_str("true"), Ok(true));
    assert!(f64::from_str("not a float at all").is_err());
    assert!(bool::from_str("TRUE").is_err());

    // The char type also implements FromStr, for strings with just one character:
    assert_eq!(char::from_str("é"), Ok('é'));
    assert!(char::from_str("abcdefg").is_err());

    // The std::net::IpAddr type, an enum holding either an IPv4 or
    // an IPv6 internet address, implements FromStr too:

    let address = IpAddr::from_str("fe80::0000:3ea9:f4ff:fe34:7a50").unwrap();
    assert_eq!(address,IpAddr::from([0xfe80, 0, 0, 0, 0x3ea9, 0xf4ff, 0xfe34, 0x7a50]));
}
```

<br>

### Converting other types to strings
There are **3 main ways** to convert non-textual values to strings:
- types that implement the `std::fmt::Display` trait, which lets you use the `{}` **format specifier** in the `format!`/`println!/...` macros;
- the **std** automatically implements the `std::str::ToString` trait for types that implement `Display`;
- **every public type** in the std implements `std::fmt::Debug`, which lets you use the `{:?}` **format specifier** in the `format!`/`println!/...` macros;

<br>

## UTF-8
### Accessing text as UTF-8
- `slice.as_bytes()`
  - borrows `slice`’s bytes as a `&[u8]`
  - since this is **not a mutable** reference, slice can assume its **bytes** will **remain** **well-formed UTF-8**;
- `string.into_bytes()`
  - **takes ownership** of `string` and returns a `Vec<u8>` of the `string`’s bytes **by value**;
  - this is a **cheap conversion**, as it simply hands over the `Vec<u8>` that the string had been using as its buﬀer;
  - since `string` **no longer exists**, the caller is free to modify the `Vec<u8>` as it pleases and the **result bytes** may become **ill-formed UTF-8**;
- 

<br>

### Producing text from UTF-8 data
`String`:
- `String::from_utf8(vec: Vec<u8>) -> Result<String, FromUtf8Error>`
  - tries to construct a **new** `String` from a vector of bytes `vec` passed **by value** and returns `Result`:
    - `Ok(string)` if `vec` holds **well-formed UTF-8**;
    - `Err(e)` where `e` is a `FromUtf8Error` error value;
      - the call `e.into_bytes()` gives you back the original vector `vec`;
      - so original `vec` is **not lost** when the **conversion fails**;
    - hence, when `vec` is **well-formed UTF-8**, **no heap allocation** or **copying** takes place;
- `String::from_utf8_lossy(byte_slice: &[u8]) -> Cow<'_, str>`
  - tries to construct a `String` or `&str` from a `&[u8]`;
  - this conversion **always succeeds**, replaces any **invalid UTF-8** sequences with *replacement characters*;
    - the Unicode *replacement characters* - �;
    - there is **constant** `std::char::REPLACEMENT_CHARACTER`:
      - `pub const REPLACEMENT_CHARACTER: char = '\u{FFFD}';`
  - it returns `Cow<str>` which
    - **borrows** a `&str` directly from `byte_slice` if it contains **well-formed UTF-8**;
    - **owns** a **new** `String` with *replacement characters* substituted for the **ill-formed** bytes;
  - hence, when `byte_slice` is **well-formed UTF-8**, **no heap allocation** or **copying** takes place;
- `String::from_utf8_unchecked(vec_bytes: Vec<u8>) -> String`
  - this simply wraps the `Vec<u8>` up as a `String` and **returns** `String`;
  - this function is **marked** `unsafe`, because it **doesn't check** that `vec_bytes` contains **well-formed UTF-8**;
  - if you sure that `vec_bytes` contains **well-formed UTF-8**, then you can call the `unsafe` function;

<br>

`str`:
- `str::from_utf8(byte_slice: &[u8]) -> Result<&str, Utf8Error>`
  - takes a `&[u8]` and **returns** `Result`:
    - `Ok(&str)` if `byte_slice` contains **well-formed UTF-8**;
    - `Err(e)` otherwise;
- `str::from_utf8_unchecked(byte_slice: &[u8]) -> &str`
  - takes a *slice of bytes* `&[u8]` and **returns** it as a *string slice* `&str`;
  - this function is **marked** `unsafe`, because it **doesn't check** that `byte_slice` contains **well-formed UTF-8**;
  - if you sure that `byte_slice` contains **well-formed UTF-8**, then you can call the `unsafe` function;
