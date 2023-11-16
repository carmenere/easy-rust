# Strings
A **string** is a sequence of `Unicode` scalar values encoded as a stream of `UTF-8` bytes.<br>

Rust has two main types of strings: `&str` and `String`.

<br>

## `&str`
The `&str` type is called **string literal** or **string slice**.<br>
*Strings* of `&str` type are **statically allocated**, i.e., they are hardcoded into binary and exists while programme is running.<br>
*Strings* of `&str` type have a **fixed-size** and **cannot be mutated**, i.e., they are **immutable**.<br>

<br>

### Examples
```Rust
let s: &str = "ABC";
```

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

## Conversions between string types
Types `OsStr` and `OsString` must be imported explicitly:
```rust
#![allow(unused_variables)]

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStringExt, OsStrExt};

fn main() {
    let s: String = String::from("abc");
    let s1: String = String::from("abc");
    let s2: String = String::from("abc");
    let st: &str = "abc";
    let u: &[u8] = "abc".as_bytes();
    let b: &[u8; 6] = b"foobar";
    let bb: [u8; 6] = b"foobar".to_owned();
    let v: Vec<u8> = String::from("abc").into_bytes();
    let v1: Vec<u8> = String::from("abc").into_bytes();
    let v2: Vec<u8> = String::from("abc").into_bytes();
    let ost: &OsStr = OsStr::new("abc");
    let os: OsString = OsString::from("abc");

    //////////////////////////////////////////////////
    
    // &str -> String
    let r: String = String::from(st);
    let r: String = st.to_string();
    let r: String = st.to_owned();

    // &str -> &[u8]
    let r: &[u8] = st.as_bytes();
    
    // &str -> Vec<u8>
    let r: Vec<u8> = st.as_bytes().to_owned();
    
    // &str -> &OsStr
    let r: &OsStr = OsStr::new(st);

    //////////////////////////////////////////////////
    
    // String -> &str
    let r: &str = s.as_str();

    // String -> &[u8]
    let r: &[u8] = s.as_bytes();

    // String -> Vec<u8>
    let r: Vec<u8> = s1.into_bytes();

    // String -> OsString
    let r: OsString = OsString::from(s);

    //////////////////////////////////////////////////

    // &[u8] -> String
    let r: String = String::from_utf8(v1).unwrap();
    let r: String = String::from_utf8(u.to_vec()).unwrap();

    // &[u8] -> &str
    let r: &str = std::str::from_utf8(u).unwrap();
    let r: &str = std::str::from_utf8(&v).unwrap();

    // &[u8] -> Vec<u8>
    let r: Vec<u8> = u.to_owned();
    let r: Vec<u8> = u.to_vec();

    // &[u8] -> &OsStr
    let r: &OsStr = OsStr::from_bytes(u); // this requires os::unix::ffi::OsStrExt
    
    //////////////////////////////////////////////////
    
    // &[u8; 6] -> &[u8]
    let r: &[u8] = st.as_bytes();
    let r: &[u8] = &b[..];
    let r: &[u8] = &b"abc"[..];

    // &[u8; 6] -> [u8; 6]
    let r: [u8; 6] = b.to_owned();

    //////////////////////////////////////////////////
    
    // [u8; 6] -> Vec<u8>
    let r: Vec<u8> = bb.to_vec();
    
    // [u8; 6] -> &[u8; 6]
    let r: &[u8; 6] = &bb;

    // [u8; 6] -> &[u8]
    let r: &[u8] = bb.as_ref();
    let r: &[u8] = bb.as_slice();

    //////////////////////////////////////////////////
    
    // Vec<u8> -> &str
    let r: Vec<u8> = v.as_slice().to_vec();
    let r: &str = std::str::from_utf8(&v).unwrap();
    
    // Vec<u8> -> String
    let r: String = String::from_utf8(v2).unwrap();

    // Vec<u8> -> &[u8]
    let r: &[u8] = v.as_slice();

    // Vec<u8> -> OsString
    let r: OsString = OsString::from_vec(v); // this requires os::unix::ffi::OsStringExt
}
```
