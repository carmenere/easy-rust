# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Assertions](#assertions)
- [Comments in Rust](#comments-in-rust)
- [Control flow](#control-flow)
  - [`if/else`](#ifelse)
  - [Loops](#loops)
    - [Labels](#labels)
- [Functions](#functions)
  - [Function declaration](#function-declaration)
  - [Generic function declaration](#generic-function-declaration)
  - [Importing and renaming inside a function](#importing-and-renaming-inside-a-function)
  - [Passing to functions](#passing-to-functions)
- [Variables declarations (aka let bindings)](#variables-declarations-aka-let-bindings)
  - [Notations](#notations)
  - [Examples](#examples)
- [Variables](#variables)
  - [Uninitialized variables](#uninitialized-variables)
- [Pointers and references](#pointers-and-references)
  - [Mutable references](#mutable-references)
  - [References and the dot operator](#references-and-the-dot-operator)
- [Operators](#operators)
  - [Arithmetic operators](#arithmetic-operators)
  - [Comparison operators](#comparison-operators)
  - [Logical operators:](#logical-operators)
  - [Bitwise operators (bit level logic):](#bitwise-operators-bit-level-logic)
    - [Examples](#examples-1)
- [Semicolon `;`](#semicolon-)
- [Shadowing](#shadowing)
- [Range operator](#range-operator)
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
Comments help other people understand your code. It’s also good to help you understand your code later.<br>

<br>

There are several kinds of comments:
1. **Regular comments** which are ignored by the compiler:
   - `//` **one line** comment which go to the end of the line;
   - `/* ... */` **multi line** comment (aka **block comment**) which go to the closing delimiter;
     - the `/* ... */` form is useful to write **in the middle** of your code: `let some_number/*: i16*/ = 100;`;
2. **Documentation comments** (aka **doc comments**, **docstrings**):
   - `///` **outer documentation** comment:
     - the `///` syntax is used to document the *item* **next** to** `///`;
     - the `///` without any text after it is interpreted as **line break**;
   - `//!` **inner documentation** comment:
     - the `//!` syntax is used to document the *item* **enclosing** `//!`, i.e. `//!` syntax is used to document the *item* **that contains the comments** rather than to the items following the comments;
     - it is often used when documenting the `.rs` file **itself**, because nothing comes before it;
     - `//!` without any text after it is interpreted as **line break**.
   - **multi line documentation comments**:
     - `/*!...*/` **inner** *block doc comment*;
     - `/**...*/` **outer** *block doc comment*;

<br>

You need to write *line comments* (`//`, `//!`, `///`) for every line.<br>

<br>

**Rust Style Guide**:
- prefer *line comments* `//` to *block comments* `/* ... */`.
- prefer *line comments* `///` to *block comments* `/** ... */`.

<br>

The *doc comments* can be automatically made into documentation for your code. The *doc comments* support **CommonMark**.<br>
**CommonMark** and **Markdown** are essentially the same language, but *CommonMark* is a **specification** of **Markdown**, i.e. it is a **standardized**, **unambiguous version** of the original *Markdown*.<br>

<br>

Commonly used **sections** in *doc comments*:
- `# Examples`;
- `# Panics` This section describes the scenarios in which the function being documented could **panic**;
- `# Errors` If the function returns a `Result`, this section describes the kinds of **errors** that might occur;
- `# Safety` notes for **unsafe** code;

<br>

# Control flow
## `if/else`
`if/else` expression allows to **branch code** depending on conditions. **Each branch** of a `if ... else` has to **return** the **same type**.<br>

**Example**:
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

**Example**:
```rust
fn main() {
    let my_number = 5;
    if my_number % 2 == 1 && my_number > 0 {
        println!("It's a positive odd number");
    } else if my_number == 6 {
        println!("It's six")
    } else {
        println!("It's a different number")
    }
}
```

<br>

## Loops
- a `loop` expression denotes an **infinite loop**;
- a `while` expression **loops until a predicate is false**;
- a `for` expression extracts values from an iterator, **loops until the iterator is empty**;

<br>

There **3 variants** of loop in rust:
- `loop { }` -  a `loop` is an **infinity loop** and it can only be stopped by calling `break` inside loop:
```rust
fn main() {
    let mut counter = 0;
    loop {
        counter +=1;
        println!("The counter is now: {counter}");
        if counter == 5 {
            break;
        }
    }
}
```
- `while condition { }` - a `while` loop is a loop that continues while `condition` is `true`:
```rust
fn main() {
    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("The counter is now: {counter}");
    }
}
```
- `for pattern in collection { }` - a `for .. in ...` is a loop that **iterates over collection or range**:
```rust
fn main() {
    for number in 0..3 {
        println!("The number is: {}", number);
    }
    for number in 0..=3 {
        println!("The next number is: {}", number);
    }
}
```

The `pattern` in `for` loop can be complex, like in `match` arms.<br>


If you **don’t need a variable name**, use `_` in `for` loop: `for _ in 0..3` or use `_` before name of variable: `for _number in 0..3`.<br>

<br>

You can also use `break` **to return a value** from loop:
```rust
fn main() {
    let mut counter = 5;
    let my_number = loop {
        counter +=1;
        if counter % 53 == 3 {
            break counter;
        }
    };
    println!("{my_number}");
}
```

<br>

### Labels
Rust allows you to give a **labels** for loops, which allows you **break** the **topmost loop** *from nested loop*.<br>
Format of **loop's label**: `'loop_label: ...`:
```rust
fn main() {
    let mut counter = 5;
    'a: loop {
        'b: loop {
            if counter > 10 {
                break 'a
            }
        }
    }
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

It is possible to declare **mutable parameters** for functions `mut` before parameter name:
```rust
fn function_name(mut variable: &mut String)
```

<br>

## Generic function declaration
```rust
fn name<T> (a: T) {
  ...
}
```

<br>

## Importing and renaming inside a function
It is possible to import items inside function. That means that inside the function you can simply write `A`, `B` and so on:
```rust
enum Foo {
    A,
    B,
    C,
}

fn foo(direction: &Foo) {
    use Foo::*; // Imports everything in Foo
    match direction {
        A => (),
        B => (),
        C => (),
    }
}

fn main() {
    foo(&Foo::A)
}
```

<br>

## Passing to functions
- `fn function_name(variable: String)` takes a `String` and **owns** it. If it **doesn’t return it**, then the variable **dies inside** the function. The value is **moved** into function.
- `fn function_name(variable: &String)` borrows a `String` and can **read** it. The variable **doesn’t die** inside the function.
- `fn function_name(variable: &mut String)` borrows a `String` and can **change** it. The variable **doesn’t die** inside the function.

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

# Variables
Any *variable* **lives as long as** *its code block*.<br>

We use the `let` keyword to declare a variable.<br>

You can capture variables inside the `{}` of `println!`:
```rust
fn main() {
  let my_number = 8;
  println!("Hello, number {my_number}");
}
```

<br>

## Uninitialized variables
**Uninitialized variable**: just write **let** and then the **variable name** and (if necessary) the **type**:
```rust
fn main() {
  let my_variable: i32;
}
```

<br>

```rust
fn main() {
    let my_number;
    {
        let calculation_result = {
            57
        };
        my_number = calculation_result;
        println!("{my_number}");
    }
}
```

<br>

# Pointers and references
The **pointer** you usually see in Rust is called a **reference**, which you can think of as a **memory-safe pointer**.<br>
A **reference** points to the memory of another value. A **reference** means you **borrow** the value, but you **don’t own it**.<br>
You can even have a **reference to a reference** or any number of references: thye are all different types.

To reach the place where the value is, we use `*`.<br>
Using `*` lets you **read** the **value behind** the **any** *reference*.<br>
Using `*` lets you **change** the **value behind** the *mutable reference*.<br>

Using `&` is called **referencing**, using `*` is called **dereferencing**.<br>

<br>

## Mutable references
For a **mutable reference**, you write `&mut` instead of `&`:
```rust
fn main() {
    let mut my_number = 8; // Don’t forget to write mut here!
    let num_ref = &mut my_number;
}
```

<br>

**Changing** the **value behind** the *mutable reference*:
```rust
fn main() {
    let mut my_number = 8;
    let num_ref = &mut my_number;
    *num_ref += 10; // Use * to change the i32 value.
    println!("{}", my_number);
    let second_number = 800;
    let triple_reference = &&&second_number;
    println!("Are they equal? {}", second_number == ***triple_reference);
}
```

<br>

## References and the dot operator
To compare value behind reference you must **dereference explicitly**.<br>

**Example**:
```rust
fn main() {
    let my_number = 9;
    let reference = &my_number;
    println!("{}", my_number == reference);
}
```
**Output**:
```rust
error[E0277]: can't compare `{integer}` with `&{integer}`
```

But this works:
```rust
fn main() {
    let my_number = 9;
    let reference = &my_number;
    println!("{}", my_number == *reference);
}
```

<br>

Also you **cannot** compare reference with double reference to the same type, e.g. you **cannot** compare compare a `&String` with a `&&String`.<br>

But you can call methods of type behind reference to value of this type. That’s because when you use a method, Rust will dereference for you **until** it **reaches** the **original type**.<br>
The `.` in a method is called the **dot operator**, and it does dereferencing **until** it **reaches** the **original type**.<br>

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

<br>

# Shadowing
**Shadowing** means using `let` to declare a **new variable** possibly of another type **with** the **same name**:
```rust
    let my_number = 8;
    println!("{}", my_number);
    let my_number = 9.2;
    println!("{}", my_number);
```

<br>

In the above example, the second variable `my_number` points to a completely different value.<br>
Is the first `my_number` destroyed? No. Because they are in the same code block `{ }`, we **can’t** see the **first** `my_number` anymore.
But if they are in **different blocks**, we **can** see both.<br>

<br>

**Code**:
```rust
{
    let my_number = 100;
    println!("outer block: {}", my_number); // this prints 100
    {
        let my_number = 9.2;
        println!("inner block: {}", my_number); // this prints 9.2
    }
    println!("outer block: {}", my_number); // this prints 100
}
```
**Output**:
```bash
outer block: 100
inner block: 9.2
outer block: 100
```

<br>

So, *shadowing* **doesn’t destroy** a value but **blocks**:<br>
**Code**:
```rust
fn main() {
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8; // It blocks the original String, but the original String is not destroyed.
    println!("{country_ref} {country}"); // The reference still points to the original String.
}
```
**Output**:
```bash
Austria, 8
```

<br>

# Range operator
Type of ranges:
- `a..b` **right-exclusive range**, e.g. `1..3` means `1, 2`;
- `..b`	**right-exclusive range to** *without starting* point;
- `a..=b`	**inclusive range**, e.g. `1..=3` means `1, 2, 3`;
- `..=b` **inclusive range to** *without starting* point;
- `a..`	**range from** *without ending* point;
- `..` **full range** means the whole collection;

<br>
