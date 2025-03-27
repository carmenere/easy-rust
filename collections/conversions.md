# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Conversions](#conversions)
  * [`&str` -> `String`|`&[u8]`|`Vec<u8>`](#str---stringu8vecu8)
  * [`String` -> `&str`|`&[u8]`|`Vec<u8>`](#string---stru8vecu8)
  * [`&[u8]` -> `String`|`&str`|`Vec<u8>`](#u8---stringstrvecu8)
  * [`Vec<u8>` -> `&str`|`String`|`&[u8]`](#vecu8---strstringu8)
<!-- TOC -->

<br>

# Conversions
<br>

![Conversions](/img/collections-conversions.png)

<br>

**Methods** of `Vec`:
- [**as_slice**](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice) - extracts a **slice** containing the entire vector, equivalent to `&s[..]`;

**Methods** of `String`:
- [**as_bytes**(&self)](https://doc.rust-lang.org/std/string/struct.String.html#method.as_bytes) - converts `String` to a **byte slice** `&[u8]`;
- [**from_utf8**(vec: Vec<u8>)](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8) - converts `Vec<u8>` to a `String`, returns `Err` if the slice is **not** _UTF-8_;
- [**into_bytes**(self)](https://doc.rust-lang.org/std/string/struct.String.html#method.into_bytes) - converts a `String` into a `Vec<u8>`;

**Methods** of `&str`:
- [**as_bytes**](https://doc.rust-lang.org/std/primitive.str.html#method.as_bytes) - converts a **string slice** `&str` to a **byte slice**;
- [**to_owned**](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned) - converts a **slice of bytes** `&[u8]` to a `Vec<u8>`;
- [**bytes**(&self)](https://doc.rust-lang.org/std/primitive.str.html#method.bytes) - returns an **iterator** over the **bytes** `u8` of a `&str`;
- [**chars**(&self)](https://doc.rust-lang.org/std/string/struct.String.html#method.chars) - returns an **iterator** over the `char` of a `&str`;

**Functions** of `std::str` module:
- [**str::from_utf8(v: &[u8])**](https://doc.rust-lang.org/std/str/fn.from_utf8.html) - converts a **slice of bytes** `&[u8]` to a **string slice** `&str`;

**Methods** of `[u8]`:
- [**to_owned**](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html#tymethod.to_owned) - converts a **slice of bytes** `&[u8]` to a `Vec<u8>`;
- [**to_vec**](https://doc.rust-lang.org/std/primitive.slice.html#method.to_vec) - converts a **slice of bytes** `&[u8]` to a `Vec<u8>`;

<br>

## `&str` -> `String`|`&[u8]`|`Vec<u8>`
```rust
fn main() {
  let str_1: &str = "abc";
  
  // &str -> String
  let r: String = String::from(str_1);
  let r: String = str_1.to_string();
  let r: String = str_1.to_owned();
  // &str -> &[u8]
  let r: &[u8] = str_1.as_bytes();
  // &str -> Vec<u8>
  let r: Vec<u8> = str_1.as_bytes().to_owned();
}
```

<br>

## `String` -> `&str`|`&[u8]`|`Vec<u8>`
```rust
fn main() {
  let string_1: String = String::from("abc");
  
  // String -> &str
  let r: &str = string_1.as_str();
  // String -> &[u8]
  let r: &[u8] = string_1.as_bytes();
  // String -> Vec<u8>
  let r: Vec<u8> = string_1.into_bytes();
}
```

<br>

## `&[u8]` -> `String`|`&str`|`Vec<u8>`
```rust
fn main() {
  let slice_1: &[u8] = "abc".as_bytes();
  
  // &[u8] -> String
  let r: String = String::from_utf8(slice_1.to_vec()).unwrap();
  // &[u8] -> &str
  let r: &str = std::str::from_utf8(slice_1).unwrap();
  // &[u8] -> Vec<u8>
  let r: Vec<u8> = slice_1.to_owned();
  let r: Vec<u8> = slice_1.to_vec();
}
```

<br>

## `Vec<u8>` -> `&str`|`String`|`&[u8]`
```rust
fn main() {
  let vec_1: Vec<u8> = String::from("abc").into_bytes();
  let vec_2: Vec<u8> = String::from("abc").into_bytes();

  // Vec<u8> -> &str
  let r: &str = std::str::from_utf8(vec_1.as_slice()).unwrap();
  // Vec<u8> -> String
  let r: String = String::from_utf8(vec_1).unwrap();
  // Vec<u8> -> &[u8]
  let r: &[u8] = vec_2.as_slice();
}
```