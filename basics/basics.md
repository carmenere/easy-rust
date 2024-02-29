# Table of contents
- [Table of contents](#table-of-contents)
- [Assertions](#assertions)
- [Closures](#closures)
  - [Notation](#notation)
  - [Various closures declarations](#various-closures-declarations)
- [Comments in Rust](#comments-in-rust)
- [Control flow](#control-flow)
  - [Notation](#notation-1)
- [Functions](#functions)
  - [Function declaration](#function-declaration)
  - [Generic function declaration](#generic-function-declaration)
- [Iterators](#iterators)
- [Variables declarations (aka let bindings)](#variables-declarations-aka-let-bindings)
  - [Notations](#notations)
  - [Examples](#examples)
- [Loops](#loops)
  - [Iterator loops](#iterator-loops)
    - [Syntax](#syntax)
    - [Examples](#examples-1)
  - [Iterator loops with `enumeration`](#iterator-loops-with-enumeration)
    - [Examples](#examples-2)
  - [Predicate loops](#predicate-loops)
    - [Syntax](#syntax-1)
    - [Example](#example)
- [Infinite loops](#infinite-loops)
    - [Syntax](#syntax-2)
    - [Example](#example-1)
- [Loop labels](#loop-labels)
  - [Example](#example-2)
- [Operators](#operators)
  - [Arithmetic operators](#arithmetic-operators)
  - [Comparison operators](#comparison-operators)
  - [Logical operators:](#logical-operators)
  - [Bitwise operators (bit level logic):](#bitwise-operators-bit-level-logic)
    - [Examples](#examples-3)
- [Semicolon `;`](#semicolon-)

<br>

# Assertions
An **assertion** is a statement that enables you to test your assumptions about your program.

If assertion is `false`, the program **crashes**.

<table>
    <tr>
        <td><b>Assertion macros</b></td>
        <td><b>Description</b></td>
    <tr>
<tr>
<td>

```Rust
assert!(expr)
```

</td>
<td>If <code>expr</code> is <b>false</b> then <code>panic!</code> is called.</td>
<tr></tr>
<tr>
<td>

```Rust
assert_eq!(left, right)
```

</td>
<td>If <code>left</code> is <b>not equal</b> <code>right</code> then <code>panic!</code> is called.</td>
</tr>
</table>

<br>

# Closures
## Notation
`|| -> { ... }`

`||` defines **arguments**, **mandatory**.

`->` defines **returning type**, **optional**.

`{}` defines **body**, **optional**

<br>

## Various closures declarations
```Rust
let x: i32 = || -> i32 { … };
let x: ()  = || {};
let x: ()  = |a, b| { … };
let x: i32 = |a, b| a + b;
```

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
```Rust
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
```Rust
fn name (a: i64) {
  ...
}
```

## Generic function declaration
```Rust
fn name<T> (a: T) {
  ...
}
```

<br>

# Iterators
Consider example:
```Rust
for item in collection {
    ...
}
```

In this example, after `for` loop *collection* `collection` is become **invalid**.<br>

Access to **collections** in loops uses `move semantics` by default.

<br>

To make the `collection` **reusable after loop** use `immutable reference` to access to the `collection`:
```Rust
for item in &collection {
    ...
}
```

<br>

To **modify item** *during* the loop use `mutable reference` to access to the `collection`:
```Rust
for item in &mut collection {
    ...
}
```

Iterator syntax variants:
<table>
<tr>
<td><b>Shorthand</b></td>
<td><b>Equivalent</b></td>
<tr>
<tr></tr>
<tr>
<td>

```Rust
for item in collection
```
</td>
<td>

```Rust
for item in IntoIterator::into_iter(collection)
```
</td>
</tr>
<tr></tr>
<tr>
<td>
        
```Rust
for item in &collection
```
</td>
        <td>

```Rust
for item in collection.iter()
```
</td>
</tr>
<tr></tr>
<tr>
<td>

```Rust
for item in &mut collection
```
</td>
<td>

```Rust
for item in collection.iter_mut()
```
</td>
    </tr>
</table>

<br>

# Variables declarations (aka let bindings)
## Notations
Here "\[\]" means *optional*. <br>
- Declaration of **immutable** variables: `let <name>`\[`: <type>`\]\[`= <value>`\]`;`
- Declaration of **mutable** variables: `let`**`mut`**`<name>`\[`: <type>`\]\[`= <value>`\]`;`

<br>

## Examples
- Declarations of **immutable** variables:
```Rust
let a: i32;
let b: i32 = 33;
let c = 33;
```
- Declarations of **mutable** variables:
```Rust
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
```Rust
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
```Rust
let v = &["apples", "cake", "coffee"];

for item in v {
    println!("I like {}.", item);
}
```

- Iterate over range:
```Rust
for i in 1..6 {
    my_f();
}
```

<br>

## Iterator loops with `enumeration`
### Examples 
- Iterate over range with enumeration:
```Rust
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
```Rust
while expression {
    ...
}
```

where `expression` is `predicate`, i.e., returns `bool` type.

### Example
```Rust
let mut i = 0;

while i < 10 {
    println!("foo");
    i = i + 1;
}
```

<br>

# Infinite loops
### Syntax
```Rust
loop {
    ...
}
```

It is similar to `while true { ... }`. But from compiler point of view it is different cases and compiler uses **additional optimizations** for `loop {}` variant.

### Example
```Rust
loop {
    println!("hello");
}
```

<br>

# Loop labels
By default, statements `break` and `continue` **refer** to the **current** *loop*.<br>
**Labels** allow to **apply** statements `break` and `continue` to the **corresponding** *outer loop*.

## Example
```Rust
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

*Semicolon* **drops** *left part* and **returns** *right part*. If there is **nothing** *on the right* *semicolon* `;` returns [**unit type**](../types/unit.md).

<br>

*Semicolon* is **necessary** after **let bindings** and **assignments**.

<br>

Examples, when `;` is **necessary**:
- After **loop**, **control** and **match** blocks to drop their results:
```Rust
for { … };
```
```Rust
if/if else/else { … };
```
```Rust
match { … };
```
- After **let bindings** and **assignments**:
```Rust
let i = 5;
```
```Rust
let mut a = 5;
a = 10;
```

<br>

Examples, when `;` can be **omitted**:
```Rust
struct Foo {}
```
```Rust
enum Bar {}
```
```Rust
fn baz() {}
```