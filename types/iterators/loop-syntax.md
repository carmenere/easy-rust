# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Types of loops](#types-of-loops)
  - [Iterator loops](#iterator-loops)
    - [Syntax](#syntax)
    - [Examples](#examples)
  - [Iterator loops with `enumeration`](#iterator-loops-with-enumeration)
    - [Examples](#examples-1)
  - [Predicate loops](#predicate-loops)
    - [Syntax](#syntax-1)
    - [Example](#example)
  - [Infinite loops](#infinite-loops)
    - [Syntax](#syntax-2)
    - [Example](#example-1)
- [Loop labels](#loop-labels)
  - [Example](#example-2)
- [Loop syntax](#loop-syntax)
<!-- TOC -->

<br>

# Types of loops
There are 4 loop types in Rust:
- iterator loops;
- iterator loops with enumeration;
- predicate loops;
- infinite loops;

<br>

## Iterator loops
There is `for` loop in Rust when *number of iterations* in **known**.

### Syntax
```rust
for var_name in expression {
    ...
}
```
where `expression` is an `iterator`.

Notes:
- The `iterator` allows to navigate through **collection**. 
- **Each element** of *collection* is **one** **iteration** of the loop. 
- **Each element** of *collection* is bound to the identifier **var_name**, which is **only valid inside** the loop.

<br>

### Examples
- Iterate over vector:
```rust
let v = &["apples", "cake", "coffee"];

for item in v {
    println!("I like {}.", item);
}
```

- Iterate over range:
```rust
for i in 1..6 {
    my_f();
}
```

<br>

## Iterator loops with `enumeration`
### Examples 
- Iterate over range with enumeration:
```rust
for (i, j) in (5..10).enumerate() {
    println!("i = {}; j = {}.", i, j);
}

Output:
    i = 0; j = 5.
    i = 1; j = 6.
    i = 2; j = 7.
    i = 3; j = 8.
    i = 4; j = 9.
```

<br>

## Predicate loops
There is `while` loop in Rust when *number of iterations* in **unknown**.

<br>

### Syntax
```rust
while expression {
    ...
}
```

where `expression` is `predicate`, i.e., returns `bool` type.

### Example
```rust
let mut i = 0;

while i < 10 {
    println!("foo");
    i = i + 1;
}
```

<br>

## Infinite loops
### Syntax
```rust
loop {
    ...
}
```

It is similar to `while true { ... }`. But from compiler point of view it is different cases and compiler uses **additional optimizations** for `loop {}` variant.

### Example
```rust
loop {
    println!("hello");
}
```

<br>

# Loop labels
By default, statements `break` and `continue` **refer** to the **current** *loop*.<br>
**Labels** allow to **apply** statements `break` and `continue` to the **corresponding** *outer loop*.

## Example
```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; }
        if y % 2 == 0 { continue 'inner; }
        println!("x: {}, y: {}", x, y);
    }
}
```

<br>

# Loop syntax
**Consider example**:
```rust
for item in collection {
    ...
}
```

In this example, after `for in` loop *collection* `collection` is become **invalid**.<br>
Note that `IntoIterator::into_iter(self)` **consumes** iterable because of `self`.<br>
Access to **collections** in loops uses **move semantics** by default.<br>

<br>

To make the `collection` **reusable after loop** use `&` to access to the `collection`:
```rust
for item in &collection {
    ...
}
```

<br>

To **modify item** *during* the loop use `&mut` to access to the `collection`:
```rust
for item in &mut collection {
    ...
}
```

<br>

The `for item in iterable` syntax requires **iterable**, so it works with both **iterators** (every iterator is iterable too) and **collections** that implement `IntoIterator`.<br>
The `for item in iterable` syntax is just a syntactic sugar for:
```rust
let mut iterator = iterable.into_iter();

loop {
    match iterator.next() {
        Some(x) => {
          // body
        },
        None => break,
    }
}
```

**here**:
- `let mut iterator = iterable.into_iter()` returns some **iterator**;
- `iterator.next()` is called repeatedly inside `loop`;

<br>

So, there are 3 variants of `for ... in ...` loop:
- `for item in iterable`
  - here `let mut iterator = iterable.into_iter();` returns an `Iterator<Item=T>` over items of `T` type, **move semantics**;
- `for item in &iterable`
  - here `let mut iterator = (&iterable).into_iter();` returns an `Iterator<Item=&T>` over items of `&T` type, this **allows** to **reuse** *original collection* **after** iteration;
- `for item in &mut iterable`
  - here `let mut iterator = (&mut iterable).into_iter();` returns an `Iterator<Item=&mut T>` over items of `&mut T` type, this **allows** to **reuse** *original collection* **after** iteration;;

<br>

To provide support for **all 3** variants of `for ... in ...` loop the **iterable** must implement `IntoIterator` for **all 3** cases: `T`, `&T` and `&mut T`:
- `T`
```rust
impl IntoIterator for SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=T> {
  }
}
```
- `&T`
```rust
impl IntoIterator for &SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&T> {
  }
}
```
- `&mut T`
```rust
impl IntoIterator for &mut SomeCollection<T> {
  fn into_iter(self) -> Iterator<Item=&mut T> {
  }
}
```

<br>