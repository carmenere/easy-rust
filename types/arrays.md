# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Arrays](#arrays)
  - [*Initialization* syntax](#initialization-syntax)
    - [Syntax options for *pre initialized* arrays](#syntax-options-for-pre-initialized-arrays)
    - [`from_fn`](#from_fn)
    - [Syntax options for *empty* arrays](#syntax-options-for-empty-arrays)
  - [*Type declaration* syntax](#type-declaration-syntax)
<!-- TOC -->

<br>

# Arrays
An **array** is **fixed-size** *collection* of elements of **the same type**.<br>
Arrays are **allocated** on the **stack**.<br>

<br>

Arrays have some pretty strict rules:
- arrays must only contain **elements of the same type**;
- arrays **cannot change their size**;

<br>

Arrays have type: `[T; N]`.<br>

For example, the array `["One", "Two"]` is `[&'static str; 2]`, while the array `["One"]` is `[&'static str; 1]`, and they are *two arrays* are of **different types**.<br>

<br>

If you want an array with all the same value, you can declare it by `[value; N]`:<br>
**Code**:
```rust
fn main() {
    let my_array = ["a"; 5];
    println!("{:?}", my_array);
}
```
**Output**:
```bash
["a", "a", "a", "a", "a"]
```

<br>

This method is used a lot to create **byte buffers**. For example, `let mut buffer = [0u8; 1024]` creates an array of **1024** bytes of **zeroes**. Its type will then be `[u8; 640]`.<br>

<br>

When you use `b` in `println!`, it turns a `&str` into a **array of bytes** `[u8, N]`.<br>
But `[T; N]` and `[T]` **doesn't** implement `std::fmt::Display` and it is needed to use `{:?}` instead of `{}`.<br>

**Example**:
```rust
fn main() {
    println!("{:?}", b"Hello there");
}
```
**Output**:
```bash
[72, 101, 108, 108, 111, 32, 116, 104, 101, 114, 101]
```

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* arrays
- **Comma-delimited**: explicit enumeration of values within square brackets \[\]:
```Rust
let arr = [0, 1, 2];
```

- **Repeat expression**: \[`V`; `N`\], where the **value** `V` is **repeated** `N`times:
```Rust
let arr = [100; 5];
```

<br>

### `from_fn`
**Code**:
```rust
fn main() {
  let arr: [_; 10] = std::array::from_fn(|i| i as u32);
  println!("{:#?}", arr);
}
```

**Output**:
```bash
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

<br>

**Note**:
- when using `from_fn()` for an array, you can pull in the index of each item if you want to use it or use `|_|` if you don’t need it;
- most of the time, you will have to tell the compiler the length of the array;
- type of element can be inferred and you can use `_`: `[_;N]`;

<br>

### Syntax options for *empty* arrays
- **Repeat expression** where `N` = 0:
```Rust
let a = [100; 0];
println!("len of 'a' is {}.", a.len());

Output:
len of a is 0.
```

<br>

## *Type declaration* syntax
- **Repeat expression**: \[`T`; `N`\], where the value of a **type** `T` is **repeated** `N` times:
```Rust
let arr1: [u64; 3] = [0, 1, 2];

let arr2: [u64; 3] = [100; 3];
```
