# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Grapheme clusters](#grapheme-clusters)
- [Char methods](#char-methods)
  - [Handling digits](#handling-digits)
  - [Classifying characters](#classifying-characters)
  - [Get Unicode escapes](#get-unicode-escapes)
  - [Case conversion for characters](#case-conversion-for-characters)
  - [Conversions to and from integers](#conversions-to-and-from-integers)
  - [Characters](#characters)
  - [Grapheme clusters](#grapheme-clusters-1)
  - [Conversions](#conversions)
<!-- TOC -->

<br>

# Grapheme clusters
A **grapheme cluster** is a sequence of one or more Unicode **code points** that should be treated as a **single unit**.<br>
Text editing software should generally allow placement of cursor only at grapheme cluster boundaries.<br>

<br>

There is **no** method in **std** to iterate over **grapheme clusters**.<br>
The [**unicode-segmentation**](https://crates.io/crates/unicode-segmentation) crate provides grapheme cluster. It provides special method `.graphemes(true)`.<br>

<br>

# Char methods
## Handling digits
- `ch.to_digit() -> -> Option<u32>`:
  - converts a **char** `ch` to a **single digit** in the given **base** (aka **radix**) `radix`;
  - a **digit** is defined to be only the following characters:
    - `0-9`
    - `a-z`
    - `A-Z`
  - returns `Some(num)`, where `num` is a `u32`, if the **char** `ch` **refers** to a **digit** in the given **radix**;
  - returns `None` if the **char** `ch` **does not refer** to a **digit** in the given **radix**;
  - **panics** if given a radix smaller than **2** or larger than **36**;
- `std::char::from_digit(num, radix)`
  - converts the `u32` digit value `num` to a `char` if possible:
    - if `num` can be represented as a **single digit** in a **base** `radix`, `from_digit` returns `Some(ch)`;
    - otherwise, it returns `None`;

<br>

**Example**:
```rust
assert_eq!('1'.to_digit(10), Some(1));
assert_eq!('f'.to_digit(16), Some(15));
```

<br>

## Classifying characters
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

## Case conversion for characters
- `ch.to_ascii_lowercase()`;
- `ch.to_ascii_uppercase()`;
- `to_lowercase()` returns **iterator** that produce the character of the **lowercase** equivalents of `ch`;
- `to_uppercase()` returns **iterator** that produce the character of the **uppercase** equivalents of `ch`;

<br>

## Conversions to and from integers
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


## Characters
**Characters** in Rust are called `char`. A `char` is always **one character** and uses `' '` (single quotes) instead of `" "` (double quotes).<br>
All **chars** use **4 bytes** of memory, since *4 bytes* are enough to hold any kind of *character*.<br>

Rust can **safely cast** a `u8` into a `char`, using `as`.<br>
We **can’t** cast `i32` as a `char`, but we **can** cast an `i32` as a `u8` and then cast `u8` to `char`:
- `let a = 100_i32 as u8 as char;`

<br>

**Strings** are different and **don’t** always use **4 bytes** per single *character*. When a *character* is part of a `string` (**not** the `char` type), the `string` is encoded to use the **least amount of memory** needed for each *character*.<br>

The `.len()` method returns the *number of bytes*, **not** the *number of letters* or *characters*.

<br>

**Code**:
```rust
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
```
**Output**:
```bash
Size of a char: 4
Size of a: 1
Size of ß: 2
Size of 国: 3
```

<br>

**Code**:
```rust
    let str1 = "Hello";
    println!("str1 is {} bytes.", str1.len());
    let str2 = "안녕"; // Korean for “Hi”
    println!("str2 is {} bytes.", str2.len());
```
**Output**:
```bash
str1 is 5 bytes.
str2 is 6 bytes.
```

<br>

What about the size in characters/letters? There is methods `.chars().count()` that return the number of characters:
**Code**:
```rust
    let str1 = "Hello";
    println!("'{}' consists of {} bytes and {} characters.", str1, str1.len(), str1.chars().count());
    let str2 = "안녕";
    println!("'{}' consists of {} bytes BUT {} characters.", str2, str2.len(), str2.chars().count());
```
**Output**:
```bash
'Hello' consists of 5 bytes and 5 characters.
'안녕' consists of 6 bytes BUT 2 characters.
```

<br>

Methods:


## Grapheme clusters
In Rust, a **grapheme cluster cannot** be a `char` because a `char` is defined as a **single** 4-byte code point of Unicode, whereas a **grapheme cluster** is a **sequence** of one or more **code points** that is displayed as a **single character**.<br>

**Namaste** (/na-ma-stay/) is the most common **greeting** in **Hindi**, suitable for both **formal** and **informal** situations.<br>
*Namaste* in **Sanskrit** is नमस्ते.<br>

The नमस्ते consists of **3 graphemes**:
- `न` (*na*): grapheme 1 and it corresponds to **1 code point**;
- `म` (*ma*): grapheme 2 and it corresponds to **1 code point**;
- `स्ते` (*ste*): grapheme 3 and it is comprised of **4 code point**: `स` + `्` + `त` + `े`;

<br>

**Example**:
```rust
fn main() {
    let namaste = "नमस्ते";

    println!("namaste.as_bytes() = {:?}", namaste.as_bytes());
    println!("namaste.len() = {}, namaste.as_bytes().len() = {}, namaste.chars().count() = {}", namaste.len(), namaste.as_bytes().len(), namaste.chars().count());
    namaste.chars().for_each(|ch| println!(r#"{} = {}, {:?}"#, ch, ch.escape_unicode(), ch.to_string().as_bytes()));
}
```
**Output**:
```rust
namaste.as_bytes() = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
namaste.len() = 18, namaste.as_bytes().len() = 18, namaste.chars().count() = 6
न = \u{928}, [224, 164, 168]
म = \u{92e}, [224, 164, 174]
स = \u{938}, [224, 164, 184]
् = \u{94d}, [224, 165, 141]
त = \u{924}, [224, 164, 164]
े = \u{947}, [224, 165, 135]
```

<br>

So, the नमस्ते consists of:
- **3** *graphemes*;
- **6** *chars* (or **6** *code points*);
- **18** *bytes*;

<br>

The following image displays the **bytes**, **code points** and **grapheme clusters** for the same word written in English (`hello`) and Hindi (`नमस्ते`):
![bytes-points-graphemes](/img/bytes-points-graphemes-2.png)

<br>

<br>

## Conversions
The `char` type in Rust **doesn't** have an `as_bytes` method because a `char` represents *fixed-size 4-byte* **Unicode codepoint** value before it has been encoded into some format, like **UTF-8**.<br>
But `char` has `to_string()` and `encode_utf8()` methods and thus `char` can be converted to **bytes** through `String` or `&str`:<br>
**Code**:
```rust
    let ch = '녕';

    let mut buf1 = [0u8; 4];
    let str = ch.encode_utf8(&mut buf1);
    let slice1 = str.as_bytes();

    let s1 = ch.to_string();
    let slice2 = s1.as_bytes();

    let s2 = ch.to_string();
    let vec = s2.into_bytes();

    println!("'{}' as bytes (ch.encode_utf8 -> &str -> s.as_bytes -> &[u8]): {:?}", ch, slice1);
    println!("'{}' as bytes (ch.to_string -> String -> s.as_bytes -> &[u8]): {:?}", ch, slice2);
    println!("'{}' as bytes (ch.to_string -> String -> s.into_bytes -> Vec<u8>): {:?}", ch, vec);
```
**Output**:
```bash
'녕' as bytes (ch.encode_utf8 -> &str -> s.as_bytes -> &[u8]): [235, 133, 149]
'녕' as bytes (ch.to_string -> String -> s.as_bytes -> &[u8]): [235, 133, 149]
'녕' as bytes (ch.to_string -> String -> s.into_bytes -> Vec<u8>): [235, 133, 149]
```

<br>