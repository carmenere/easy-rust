# Table of contents
- [Table of contents](#table-of-contents)
- [Slice type](#slice-type)
    - [Slice notation](#slice-notation)
    - [Slice rules](#slice-rules)
    - [String slice](#string-slice)
    - [`impl str`](#impl-str)
    - [String slice as type of function parameter](#string-slice-as-type-of-function-parameter)
    - [Get last element of dynamically growing collection](#get-last-element-of-dynamically-growing-collection)

<br>

# Slice type
A **slice** is a **DST** representing a *view* into a **contiguous sequence of elements** of type `T`.<br>
The **slice type** is written as `[T]`, **without** specifying the **length**.<br>

**DST** means slices **don't** implement the trait `Sized` and therefore slices **can’t** be stored directly in variables or passed as function arguments, e.g. `let s: [u64];` causes to error.<br>

**Slice types** are generally **used** through **pointer types**, that's why we often refer to all of `[T]`, `&[T]`, and `&mut [T]` as a **slice**:
- `&[T]`: **shared reference** to a **slice** (aka **shared slice**);
- `&mut [T]`: **mutable reference** to a **slice** (aka **mutable slice**);
- `Box<[T]>`: a **boxed slice**;

<br>

Illustration:
```bash
+---+---+---+---+
|Pointer| Length|  &[T] (or &str)
+---+---+---+---+
    |
    V
    +---+---+---+---+---+
    | D | A | T | A | . |  [T] (or str)
    +---+---+---+---+---+
```

<br>

Internally, the **reference to a slice** (aka **slice reference**) is a **fat pointer** that contains 2 objects:
- **pointer** to the slice’s **first element**;
- **number of elements** in the slice (the **length** of the slice);

<br>

**Slice references** a good choice when you want to write a function that operates on either an **array** or a **vector**.<br>

<br>

## Slice notation
Slice notation: `& <collection>[start..end]`, where:
- `start..end` is **range operator**, by default it **excludes upper bound** and is equal to `[start, end)`;
    - to **include** upper bound use `=`, example: `start..=end`;
- `<collection>` name of some collection;

Note that **length of the slice**: `length = end – start`.<br>

Notes:
- if *slice* **includes** the **first element**, you can omit the `start`:
```Rust
let slice = &s[..2];
let slice = &s[0..2];
```
- if *slice* **includes** the **last element**, you can omit `end`:
```Rust
let slice = &s[3..];
let len = s.len();
let slice = &s[3..len];
```

<br>

## Slice rules
Slices **don’t** allow to **change state** of **collection** as they are **references**.<br>

How slices prevent from bugs, consider following example:
```Rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // word is now totally invalid!
}
```

This code compiles without any errors.<br>
But, because `word` **isn’t connected** to the **state** of `s` at all, `word` still contains the value 5 after calling `s.clear()`.<br>
We could use that value 5 with the variable `s` to try to extract the first word out, but this would be a bug because the contents of `s` have changed since we saved 5 in `word`.<br>

<br>

## String slice
The type `str` is a **string slice**. Semantically `str` can be represented as
```rust
struct str([u8])
```

You can think of a `str` as a `[u8]` which has **additional guarantees** that **sequence of bytes** `[u8]` contains valid **UTF-8** encoded **Unicode chars**.<br>

Because `str` is a **slice** it used in its borrowed form: `&str`. The type `&str` is a **reference to a string slice** with some **lifetime** and `&str` is also called **string slice**. That's why `&str` is also called **string slice**.<br>
**String literals** are **statically allocated**, i.e., they are hardcoded into binary and exists while programme is running and have type `&'static str`.<br>
So, all **string literals** are `&strs`, but **not** all `&strs` are **string literal**.<br>

<br>

Example:
```Rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
```

Range indices for `String` slice must occur at **valid** UTF-8 character boundaries.<br>
If you attempt to create a `String` slice in the middle of a multibyte character, your program will exit with an error.<br>

<br>

## `impl str`
The type `str` is **builtin type** and defined inside compiler. But `impl str` is defined in `std`:
```rust
#[cfg(not(test))]
impl str {
    pub const fn len(&self) -> usize {
        self.as_bytes().len()
    }
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: const sound because we transmute two types with the same layout
        unsafe { mem::transmute(self) }
    }
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *(self as *mut str as *mut [u8]) }
    }
}
```

<br>

## String slice as type of function parameter
Consider signature: `fn f1(s: &String) -> &str { }`. This function can only receive `&String` type.<br>
Let’s rewrite it to: `fn f2(s: &str) -> &str { }`. This function can receive both `&String` and `&str` types.<br>

Because **string literals** are **string slices** already, it is possible to pass value of `&str` (**string literal**) to `f2` directly.

<br>

## Get last element of dynamically growing collection
There are 2 ways to access to the last lement of collection:
1. Calling `last` on **slice**:
```Rust
fn main() {
    let mut v = vec![1,2,3,4,5];
    v.push(6);
    println!("{:?}", &v[..].last());
    v.push(7);
    println!("{:?}", &v[..].last());
}
```
2. Calling `last` or `last_mut` on `Vec` directly:
```Rust
fn main() {
    let v = &mut vec![0, 1, 2];

    if let Some(last) = v.last_mut() {
        *last = 10;
    }
    println!("{:?}", v.last_mut());
}
```

<br>

Notes:
- `last_mut` method returns a **mutable pointer** to the **last item** in the collection;
- signature of `last_mut` method: `pub fn last_mut(&mut self) -> Option<&mut T>`.
