# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Strings](#strings)
  - [Types of `&str`](#types-of-str)
  - [UTF-8](#utf-8)
  - [Bytes. Chars. Vec](#bytes-chars-vec)
  - [String literals](#string-literals)
  - [`String`](#string)
- [`String` methods](#string-methods)
  - [Creating `String`](#creating-string)
  - [Get basic information from string slices](#get-basic-information-from-string-slices)
  - [Appending and inserting text to a `String`](#appending-and-inserting-text-to-a-string)
  - [Removing and replacing text](#removing-and-replacing-text)
  - [Searching and replacing](#searching-and-replacing)
  - [Iterating over text](#iterating-over-text)
  - [Trimming](#trimming)
  - [Case conversion for strings](#case-conversion-for-strings)
  - [Parsing](#parsing)
    - [Converting from strings to other types](#converting-from-strings-to-other-types)
    - [Converting other types to strings](#converting-other-types-to-strings)
  - [UTF-8](#utf-8-1)
    - [Accessing text as UTF-8](#accessing-text-as-utf-8)
    - [Producing text from UTF-8 data](#producing-text-from-utf-8-data)
- [`std::ffi::OsStr`, `std::ffi::OsString`, `std::path::Path` and `std::path::PathBuf`](#stdffiosstr-stdffiosstring-stdpathpath-and-stdpathpathbuf)
<!-- TOC -->

<br>

# Strings
Rust has two main types of strings: `String` and `&str`.
- `&str` is a simple string. It’s just a pointer to the data plus the length. It is also called a string slice. It might just be a **partial view** of the data owned by some
other variable, so just a **slice** of it.
    - `str` can be of **any length**;
    - `str` is a **dynamically sized type**. *Dynamically sized* means that the size can be different;
    - that's why we need an `&` because it makes a pointer, and **Rust knows the size of the pointer**;
- `String` is a pointer with data on the heap. A `String` is easy to grow, shrink, mutate, and so on.

The biggest difference is that a `String` **owns** its data, while a `&str` is a **slice**. Because you use a `&` to interact with a `str`, you **don’t** *own* it.<br>
But a `String` is an **owned type**.<br>

<br>

## Types of `&str`
There are 2 types of `&str`:
- **string literal**:
  - you make **string literals** when you write `let my_str = "I am a string literal";`;
  - they **last for the whole program** because they are **written directly into the binary**;
  - they have the type `&'static str`;
- **borrowed** `str`:
  - this is the regular `&str` form **without** a `'static` lifetime;
  - consider you pass `&String` to a function parameter that declared as `&str`, Rust will convert `&String` to a `&str` (thanks `Deref` trait);

<br>

## UTF-8
Both `str` and `String` contain **Unicode characters** encoded with **UTF-8**.<br>
In other words, string in Rust is a **UTF-8** encoded sequence of bytes.<br>
**Unicode characters** encoded with **UTF-8** have **variable length** from **1** to **4** bytes.<br>
**UTF-8** is backward-compatible with **ASCII**. The first 128 UTF-8 characters precisely match the first 128 ASCII characters, meaning that existing ASCII text is already **valid UTF-8**.<br>

We can see this with two functions:
- `size_of`, which shows the **size of a type** in bytes;
- `size_of_val`, which shows the **size of a value** in bytes pointed to;

There are many ways to make a `String`:
- `String::from("This is the string text")`
- `"This is the string text".to_string()`
- `format!("My name is {}.", name)`

Another way to make a `String` is to call `.into()`, but it is a bit different because `.into()` isn’t for making a `String`; it’s for converting from one type into another type.<br>

Some types can easily convert to and from another type using `From::` and `.into()`; if you have `From`, you also have `.into()`.
`From` is clearer because you already know the types: for example `String::from("Some str")` you know that `String` is from a `&str`.
But with `.into()`, sometimes the compiler doesn’t know.
**Code**:
```rust
fn main() {
let my_string = "Try to make this a String".into();
}
```
**Output**:
```rust
error[E0282]: type annotations needed
```

It's because **many types can be made from** a `&str`. It is possible to make `&str` into a lot of things, so which one do you want?

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

**Examples**:
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

## Appending and inserting text to a `String`
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
- `string.retain(f: F)`
  - **remove** all characters `c` such that `f(c)` returns `false`;
  - this method operates **in place**;

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

<br>

# `std::ffi::OsStr`, `std::ffi::OsString`, `std::path::Path` and `std::path::PathBuf`
Inconveniently, your operating system does not force filenames to be valid Unicode.<br>
This is why Rust has:
- `std::ffi::OsStr`
  - the `OsStr` is a string type that’s a superset of UTF-8;
  - its job is to be able to represent all filenames, command-line arguments, and environment variables on the current system, whether they’re valid Unicode or not;
- `std::ffi::OsString` owns a **heap-allocated** `OsStr`;
  - `.to_os_string()`: **converts** `OsStr` **to** `OsString`;
- `std::path::Path`
  - it is exactly like `OsStr`, but it adds many handy filename-related methods;
- `std::path::PathBuf` owns a **heap-allocated** `Path`;
  - `.to_path_buf()`: **converts** `Path` **to** `PathBuf`;

<br>

All three of these types `str`, `OsStr` and `Path` implement `AsRef<Path>`, so we can easily declare a generic function that accepts "any filename type" as an argument:
```rust
use std::path::Path;

fn foo<P>(path: P)
where P: AsRef<Path>
{
    let path = path.as_ref();
}
```

Use `Path` for both absolute and relative paths.<br>

<br>

Some of the functions in `std::fs` and their approximate equivalents on **POSIX**:
|`std::fs` function|**POSIX**|
|:-----------------|:---|
|`create_dir(path)`|`mkdir`|
|`create_dir_all(path)`|`mkdir -p`|
|`remove_dir(path)`|`rmdir`|
|`remove_dir_all(path)`|`rm -r`|
|`remove_file(path)`|`unlink`|
|`copy(src_path, dest_path) -> Result<u64>`|`cp -p`|
|`rename(src_path, dest_path)`|`rename`|
|`hard_link(src_path, dest_path)`|`link`|
|`canonicalize(path) -> Result<PathBuf>`|`realpath`|
|`metadata(path) -> Result<Metadata>`|`stat`|
|`symlink_metadata(path) -> Result<Metadata>`|`lstat`|
|`read_dir(path) -> Result<ReadDir>`|`opendir`|
|`read_link(path) -> Result<PathBuf>`|`readlink`|
|`set_permissions(path, perm)`|`chmod`|

<br>

The special directories `.` and `..` are **not** listed when reading a directory.<br>

As a convenience, the `Path` type has a few of these built in as methods:
- `path.metadata()`;
- `path.read_dir()`;

<br>

The `OsString` has an interesting method called `.into_string()` that **tries** to make it into a regular `String`. It **returns** a `Result`, but the `Err` part is just the **original** `OsString`:
```rust
pub fn into_string(self) -> Result<String, OsString>
```

So if it **doesn’t work**, you just get the previous `OsString` back. You **can’t call** `.unwrap()` because it **will panic**, but you **can use** `match` to get the `OsString` back:
```rust
use std::ffi::OsString;

fn osstr(s: OsString) {}
fn string(s: String) {}

fn main() {
  let os_string = OsString::from("This string works for your OS too.");
  match os_string.into_string() {
    Ok(valid) => {
      println!("String: {:?}", valid);
      string(valid)
    },
    Err(not_valid) => {
      println!("OsString: {:?}", not_valid);
      osstr(not_valid);
    }
  }
}
```
