# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Vectors](#vectors)
  - [*Initialization* syntax](#initialization-syntax)
    - [Syntax options for *pre initialized* vectors:](#syntax-options-for-pre-initialized-vectors)
    - [Syntax options for *empty* vectors:](#syntax-options-for-empty-vectors)
  - [*Type declaration* syntax](#type-declaration-syntax)
  - [Slices](#slices)
  - [Reallocation](#reallocation)
  - [Conversions](#conversions)
  - [Methods](#methods)
<!-- TOC -->

<br>

# Vectors
A **vector** is *collection* of elements of **the same type** that is allowed to **grow** or **shrink** *in size* **at runtime**.<br>
Vectors are **allocated** on the **heap**.<br>
`Vec` is a type for **vector** provided by the **standard library**.<br>
`capacity` is the number of elements the `Vec` can hold without reallocating.<br>

<br>

Vectors have some pretty strict rules:
- vectors must only contain **elements of the same type**;
- vectors **can change their size** *at the runtime*;

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* vectors:
- **Comma-delimited** by `vec!` macros: explicit enumeration of values within square brackets \[\]:
```Rust
let v = vec![0, 1, 2];
```

- **Repeat expression** by `vec!` macros: \[`V`; `N`\], where the **value** `V` is **repeated** `N` times:
```Rust
let v = vec![100; 5];
```

### Syntax options for *empty* vectors:
- **Vector type constructor** (`new` or `with_capacity`):
```Rust
let v1: Vec<i64> = Vec::with_capacity(10);
let mut v2 = Vec::new();
```
- **Repeat expression** where `N` = 0:
```Rust
let v = vec![100; 0];
```

<br>

If type was **not** declared, then Rust will **infer** type from a **value** of the **first** `.push(value)`.<br>
Also it is possible to set type **explicitly**:
```rust
let mut my_vec: Vec<String> = Vec::new();
```

<br>

Another way to create a `Vec` is with the `vec!` macro:
```rust
let mut my_vec = vec![8, 10, 10];
```

<br>

## *Type declaration* syntax
- `Vec<T>`
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```

<br>

## Slices
You can **slice a vector** too, just like an array:
```rust
fn main() {
    let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_four = &vec[2..5]; // the type of two_to_four is &[u8]
    let start_at_one = &vec[1..]; // the type of start_at_one is &[u8]
    let end_at_four = &vec[..5]; // the type of end_at_four is &[u8]
    let end_at_five = &vec[..=5]; // the type of end_at_five is &[u8]
    let slice_of_all = &vec[..]; // the type of slice_of_all is &[u8]
    let ref_to_vec = &vec; // the type of ref_to_vec is &Vec<u8>, NOT &[u8]
    println!("&vec[2..5]: {two_to_four:?}, len: {},
&vec[1..]: {start_at_one:?}, len: {},
&vec[..5]: {end_at_four:?}, len: {},
&vec[..=5]: {end_at_five:?}, len: {},
&vec[..]: {slice_of_all:?}, len: {}
&vec: {ref_to_vec:?}, len: {}", 
    two_to_four.len(), start_at_one.len(), end_at_four.len(), end_at_five.len(), slice_of_all.len(), ref_to_vec.len());
    take_slice(two_to_four);
    take_slice(ref_to_vec);
}

fn take_slice(s: &[u8]) {
    println!("s = {:?}", s);
}
```

<br>

## Reallocation
A `Vec` has a **capacity**, which means the **amount of memory** given to the `Vec`.<br>
Every time `Vec` reaches its **capacity** it **reallocates** a new memory space that can hold *old capacity* + **1** items.<br>
Every time vector **reallocates** a new memory it **double** its *old capacity*.<br>

By default `Vec::new()` creates empty vector with capacity **0**.<br>
When you add first element to such vector with capacity **0** its sets its capacity to **4**.<br>
And then it will **double** its capacity at each reallocation.<br>

Consider example:
```rust
fn main() {
    let mut num_vec = Vec::new();
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
}
```
**Output**:
```bash
0
4
4
8
```

This vector has two reallocations:
- **0** to **4**;
- **4** to **8**;

<br>

We can make it **more efficient** by giving it a capacity of **8** to start:
```rust
fn main() {
    let mut num_vec = Vec::with_capacity(8);
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    println!("{}", num_vec.capacity());
    num_vec.push('a');
    num_vec.push('a');
    println!("{}", num_vec.capacity());
}
```
**Output**:
```bash
8
8
8
8
```

<br>

## Conversions
You can use `.into()` to make an **array** into a `Vec`:
```rust
fn main() {
    let my_vec: Vec<u8> = [1, 2, 3].into();
    let my_vec2: Vec<_> = [9, 0, 10].into(); // This makes a Vec<i32>
    println!("The type of `my_vec` is: {}", std::any::type_name_of_val(&my_vec));
    println!("The type of `my_vec2` is: {}", std::any::type_name_of_val(&my_vec2));
}
```

- `std::any::type_name_of_val` prints type of value;
- `Vec<_>` means that Rust **infers** type of element for `Vec`;


<br>

## Methods
- `.sort()`
  - sorts the slice in ascending order, **preserving initial order** of equal elements;
  - this sort is **stable** (i.e., *does not reorder equal elements*) and **in-place** (i.e., *does not allocate*);
- `.sort_unstable()`
  - sorts the slice in ascending order **without preserving the initial order** of equal elements;
  - this sort is **unstable** (i.e., *may reorder equal elements*) and **in-place** (i.e., *does not allocate*);
- `.dedup()`
  - **removes** items that are the **same** in a vector, but **only** if they are **next to each other**;
  - so, if you want to use `.dedup()` **to remove every duplicate**, just `.sort()` first;
- `.split_at()`
  - divides into **two slices** at the `mid` index: `[0, mid)` and `[mid, len)`;
- `.split_at_mut(mid: usize)`
  - divides into **two mutable slices** at the `mid` index: `[0, mid)` and `[mid, len)`;
- `.drain()`
  - lets you **pull a range of values out of** a `Vec`, giving you an iterator;
  - this iterator keeps a mutable borrow on the original `Vec` so doing something like collecting it into another `Vec` or outright using the `drop()` method will let you **access** the **original** `Vec` **again**;

<br>

**Code**:
```rust
fn main() {
  let mut original_vec = ('A'..'K').collect::<Vec<_>>();

  println!("The 'original_vec' before drain: {original_vec:?}");
  for i in  original_vec.drain(2..=5) {
    println!("  the char '{i}' is pulled out");
  }
  println!("The 'original_vec' after first drain: {original_vec:?}");

  let drain_two = original_vec.drain(2..=4).collect::<Vec<_>>();
  println!("The 'original_vec' after second drain: {original_vec:?}");
  println!("The 'drain_two' after second drain: {drain_two:?}");
}
```

**Output**:
```bash
The 'original_vec' before drain: ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J']
  the char 'C' is pulled out
  the char 'D' is pulled out
  the char 'E' is pulled out
  the char 'F' is pulled out
The 'original_vec' after first drain: ['A', 'B', 'G', 'H', 'I', 'J']
The 'original_vec' after second drain: ['A', 'B', 'J']
The 'drain_two' after second drain: ['G', 'H', 'I']
```

<br>