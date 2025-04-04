# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Assertions](#assertions)
* [Comments in Rust](#comments-in-rust)
* [Control flow](#control-flow)
  * [Notation](#notation)
* [Functions](#functions)
  * [Function declaration](#function-declaration)
  * [Generic function declaration](#generic-function-declaration)
* [Variables declarations (aka let bindings)](#variables-declarations-aka-let-bindings)
  * [Notations](#notations)
  * [Examples](#examples)
* [Loops](#loops)
  * [Iterator loops](#iterator-loops)
    * [Syntax](#syntax)
    * [Examples](#examples-1)
  * [Iterator loops with `enumeration`](#iterator-loops-with-enumeration)
    * [Examples](#examples-)
  * [Predicate loops](#predicate-loops)
    * [Syntax](#syntax-1)
    * [Example](#example)
* [Infinite loops](#infinite-loops)
    * [Syntax](#syntax-2)
    * [Example](#example-1)
* [Loop labels](#loop-labels)
  * [Example](#example-2)
* [Operators](#operators)
  * [Arithmetic operators](#arithmetic-operators)
  * [Comparison operators](#comparison-operators)
  * [Logical operators:](#logical-operators)
  * [Bitwise operators (bit level logic):](#bitwise-operators-bit-level-logic)
    * [Examples](#examples-2)
* [Semicolon `;`](#semicolon-)
<!-- TOC -->

<br>

# Assertions
An **assertion** is a statement that enables you to test your assumptions about your program.<br>
If assertion is `false`, the program **crashes**.<br>

| Assertion macros           | Description                                                 |
|:---------------------------|:------------------------------------------------------------|
| `assert!(expr);`           | If `expr` is **false** then `panic!` is called.             |
| `assert_eq!(left, right);` | If `left` is **not equal** `right` then `panic!` is called. |

<br>

# Comments in Rust
There are 2 kinds of comments:
1. **Regular comments** which are ignored by the compiler:
- `//` **One line** comment which go to the end of the line;
- `/* ... */` **Multi line** comment which go to the closing delimiter;
2. **Documentation comments** (aka **doc comments**, **docstrings**):
- `///` **Outer documentation** comment:
  - the `///` syntax is used to document the *item* **next** to** `///`;
  - the `///` without any text after it is interpreted as **line break**;
- `//!` **Inner documentation** comment:
  - the `//!` syntax is used to document the *item* **enclosing** `//!`, i.e. `//!` syntax is used to document the *item* **that contains the comments** rather than to the items following the comments;
  - it is often used when documenting the `.rs` file **itself**, because nothing comes before it;
  - `//!` without any text after it is interpreted as **line break**.

<br>

**Documentation comments** support **Markdown** notation.<br>

Commonly used **sections** in **Documentation comments**:
- `# Examples`;
- `# Panics` This section describes the scenarios in which the function being documented could **panic**;
- `# Errors` If the function returns a `Result`, this section describes the kinds of **errors** that might occur;
- `# Safety` notes for **unsafe** code;

<br>

# Control flow
`if/else` expression allows to **branch code** depending on conditions.

## Notation
```rust
if expr1 {
    ...
} else if expr2 {
    ...
} else if expr3 {
    ...
} else {
    ...
}
```

<br>

# Functions
## Function declaration
```rust
fn name (a: i64) {
  ...
}
```

## Generic function declaration
```rust
fn name<T> (a: T) {
  ...
}
```

<br>

# Variables declarations (aka let bindings)
## Notations
Here "\[\]" means *optional*. <br>
- Declaration of **immutable** variables: `let <name>`\[`: <type>`\]\[`= <value>`\]`;`
- Declaration of **mutable** variables: `let`**`mut`**`<name>`\[`: <type>`\]\[`= <value>`\]`;`

<br>

## Examples
- Declarations of **immutable** variables:
```rust
let a: i32;
let b: i32 = 33;
let c = 33;
```
- Declarations of **mutable** variables:
```rust
let mut x: i32;
let mut y: i32 = 33;
let mut z = 33;
```

<br>

# Loops
There are 4 loop types in Rust:
- Iterator loops
- Iterator loops with enumeration
- Predicate loops
- Infinite loops

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

# Infinite loops
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

# Operators
## Arithmetic operators
- `+` addition
- `-` subtraction
- `*`	multiplication
- `/`	division
- `%`	modulus

<br>

## Comparison operators

- `<`
- `<=`
- `>`
- `>=`
- `==`
- `!=`

<br>

## Logical operators:
- `&&` **AND**
- `||` **OR**
- `!` **NOT**

<br>

## Bitwise operators (bit level logic):
- `&` Bitwise **AND**
- `|` Bitwise **OR**
- `^` Bitwise **XOR**
- `!` Bitwise **NOT**
- `<<` Left shift
- `>>` Right shift
### Examples
`10 << 1` – shift `10` left by **one position**, equals to multiplication on 2.

<br>

# Semicolon `;`
Rust is an **expression-oriented language**. This means that **most things are expressions**.

<br>

**Semicolon** `;` is used to **turn** an *expression* **into** *statement*.

<br>

*Semicolon* **drops** *left part* and **returns** *right part*. If there is **nothing** *on the right* *semicolon* `;` returns [**unit type**](./types.md).

<br>

*Semicolon* is **necessary** after **let bindings** and **assignments**.

<br>

Examples, when `;` is **necessary**:
- After **loop**, **control** and **match** blocks to drop their results:
```rust
for { … };
```
```rust
if/if else/else { … };
```
```rust
match { … };
```
- After **let bindings** and **assignments**:
```rust
let i = 5;
```
```rust
let mut a = 5;
a = 10;
```

<br>

Examples, when `;` can be **omitted**:
```rust
struct Foo {}
```
```rust
enum Bar {}
```
```rust
fn baz() {}
```