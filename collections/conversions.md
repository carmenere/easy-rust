# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Snippets from std](#snippets-from-std)
  * [str](#str)
  * [Slice](#slice)
  * [String](#string)
<!-- TOC -->

<br>

# Snippets from std
## str
```rust
impl str {
    pub fn into_string(self: Box<str>) -> String {
        let slice = Box::<[u8]>::from(self);
        unsafe { String::from_utf8_unchecked(slice.into_vec()) }
    }
}

impl ToOwned for str {
    type Owned = String;

    #[inline]
    fn to_owned(&self) -> String {
        unsafe { String::from_utf8_unchecked(self.as_bytes().to_owned()) }
    }

    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { mem::transmute(self) }
    }
}

impl Display for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.pad(self)
    }
}

// str has .to_string() because of Display
impl<T: fmt::Display + ?Sized> ToString for T {
    fn to_string(&self) -> String {
        <Self as SpecToString>::spec_to_string(self)
    }
}
```

<br>

## Slice
```rust
impl<T> [T] {
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.to_vec_in(Global)
    }

    pub fn into_vec<A: Allocator>(self: Box<Self, A>) -> Vec<T, A> {
        hack::into_vec(self)
    }
}

impl<T: Clone> ToOwned for [T] {
    type Owned = Vec<T>;
    #[cfg(not(test))]
    fn to_owned(&self) -> Vec<T> {
        self.to_vec()
    }
}
```

<br>

## String
```rust
impl String {
    pub const fn into_bytes(self) -> Vec<u8> {
        self.vec
    }
}
```

<br>

```rust
fn main() {
  let string_1: String = String::from("abc");
  let str_1: &str = "abc";
  

}
```

<br>

# Conversions

![Conversions](/img/collections-conversions.png)

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