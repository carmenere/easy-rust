# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Chapter 01](#chapter-01)
  - [Introducing Rust](#introducing-rust)
  - [Comments](#comments)
  - [Integers](#integers)
    - [Casting integers](#casting-integers)
  - [Characters](#characters)
  - [Grapheme clusters](#grapheme-clusters)
    - [Conversions](#conversions)
  - [Type inference](#type-inference)
  - [Floats](#floats)
  - [Declaring vars](#declaring-vars)
  - [Display and Debug](#display-and-debug)
  - [Shadowing](#shadowing)
- [Chapter 02](#chapter-02)
  - [Variables](#variables)
  - [Pointers and references](#pointers-and-references)
    - [Mutable references](#mutable-references)
  - [Strings](#strings)
  - [const and static](#const-and-static)
  - [Shadowing again](#shadowing-again)
  - [Passing to functions](#passing-to-functions)
  - [Copy types](#copy-types)
  - [Uninitialized variables](#uninitialized-variables)
  - [More about printing](#more-about-printing)
- [Chapter 03](#chapter-03)
  - [Arrays](#arrays)
    - [Slices](#slices)
  - [Vectors](#vectors)
    - [Slices](#slices-1)
    - [Reallocation](#reallocation)
    - [Conversions](#conversions-1)
    - [Tuples](#tuples)
- [Control flow](#control-flow)
  - [Basic control flow](#basic-control-flow)
  - [Loops](#loops)
    - [Labels](#labels)
- [Chapter 04](#chapter-04)
  - [Structs](#structs)
  - [Enums](#enums)
    - [Discriminants](#discriminants)
    - [Casting enums into integers](#casting-enums-into-integers)
    - [Importing enum variants](#importing-enum-variants)
    - [Holding different types in collection](#holding-different-types-in-collection)
    - [Implementing structs and enums](#implementing-structs-and-enums)
    - [References and the dot operator](#references-and-the-dot-operator)
- [Chapter 05](#chapter-05)
- [Chapter 06](#chapter-06)
  - [HashMap](#hashmap)
    - [The .entry() api](#the-entry-api)
  - [BTreeMap](#btreemap)
  - [HashSet and BTreeSet](#hashset-and-btreeset)
  - [BinaryHeap](#binaryheap)
  - [VecDeque](#vecdeque)
  - [The `?` operator](#the--operator)
- [Chapter 07](#chapter-07)
  - [`From` trait](#from-trait)
  - [The orphan rule](#the-orphan-rule)
  - [`AsRef` trait](#asref-trait)
- [Chapter 08](#chapter-08)
  - [Iterators and loops](#iterators-and-loops)
  - [Closures and closures inside iterators](#closures-and-closures-inside-iterators)
  - [Closures: lazy and fast](#closures-lazy-and-fast)
  - [`|_|` in a closure](#_-in-a-closure)
- [Chapter 09](#chapter-09)
  - [Option/Result methods](#optionresult-methods)
  - [`.unwrap_or()` and  `.unwrap_or_else()`](#unwrap_or-and--unwrap_or_else)
  - [Iterator methods chaining](#iterator-methods-chaining)
  - [`.for_each()`](#for_each)
  - [`.skip()` and `.take()`](#skip-and-take)
    - [.enumerate()](#enumerate)
  - [`.filter_map()` and .ok()](#filter_map-and-ok)
  - [`.ok_or(error)`](#ok_orerror)
  - [`.ok_or_else(error)`](#ok_or_elseerror)
  - [`.and_then()`](#and_then)
  - [`.and()`](#and)
  - [`.zip()`](#zip)
    - [`.zip()` and `.cycle()`](#zip-and-cycle)
    - [`.zip()` and `.collect()` into HashMap](#zip-and-collect-into-hashmap)
  - [`.fold()` numbers](#fold-numbers)
  - [`.fold()` events](#fold-events)
  - [`.by_ref()`](#by_ref)
  - [`.chunks()` and `.windows()`](#chunks-and-windows)
  - [`.match_indices()`](#match_indices)
  - [`.peekable()` and `.peek()`](#peekable-and-peek)
  - [`flatten()`](#flatten)
    - [Flattening a single nested `Result` or `Option`](#flattening-a-single-nested-result-or-option)
    - [Flattening an `Iterator` that yields `Result` or `Option`](#flattening-an-iterator-that-yields-result-or-option)
    - [Collecting a `Vec<Result<T, E>>` into a `Result<Vec<T>, E>`](#collecting-a-vecresultt-e-into-a-resultvect-e)
  - [`.inspect()`](#inspect)
  - [The `dbg!` macro](#the-dbg-macro)
    - [Suppress `dbg!` in release builds](#suppress-dbg-in-release-builds)
      - [Attribute `debug_assertions`](#attribute-debug_assertions)
      - [Defining a custom macro](#defining-a-custom-macro)
- [Chapter 10](#chapter-10)
- [Chapter 11](#chapter-11)
- [Chapter 12](#chapter-12)
- [Chapter 13](#chapter-13)
- [Chapter 14](#chapter-14)
- [Chapter 15](#chapter-15)
- [Chapter 16](#chapter-16)
<!-- TOC -->

<br>

# Chapter 01
## Introducing Rust
So Rust is a language that is famously difficult to learn. But I don’t agree that Rust is difficult. Programming itself is difficult. Rust simply shows you these difficulties *when you are writing your code, not after you start running it*. That’s where this saying comes from: “**In Rust, you get the hangover first.**” In many other languages, the party starts first: your code compiles and looks great! Then you run your code, and there’s a lot to debug. That’s when you get the hangover.

<br>

## Comments
Comments help other people understand your code. It’s also good to help you understand your code later.

Line comments:
- `//` line comment;
- `//!` **inner** line *doc comment* (documentation comment);
- `///` **outer** line *doc comment*;

You need to write line comments (`//`, `//!`, `///`) for every line.

Block comments:
- `/*...*/` block comment;
- `/*!...*/` **inner** block *doc comment*;
- `/**...*/` **outer** block *doc comment*;

Rust Style Guide:
- prefer line comments `//` to block comments `/* ... */`.
- prefer line comments `///` to block comments `/** ... */`.

The `/* */` form is is useful to write **in the middle** of your code: `let some_number/*: i16*/ = 100;`.

A **doc comment** can be automatically made into documentation for your code.

<br>

## Integers
The simplest types in Rust are called primitive types (primitive = very basic).<br>
There are two types of integers: **signed integers** and **unsigned integers**.<br>
Rust always chooses `i32` for integers if you don’t tell it to use a different type.<br>
A `usize` is the best size for **indexing** because.<br>

<br>

### Casting integers
When you cast a **large number** into a **smaller type**, the **result** is the value of `large_number modulo smaller_type`.<br>
In the example below, `232 = 1000 mod 256`:
**Code**:
```rust
fn main() {
    let a = -1000_i16;
    let b = 1000_u16;
    println!("`{}` as u8 = {}", a, a as u8);
    println!("`{}` as u8 = {}", b, b as u8);
}
```
**Output**:
```bash
`-1000` as u8 = 24
`1000` as u8 = 232
```

<br>

For conversions between integer types where the value might not fit, the `TryFrom` trait returns a `Result` type, forcing you to handle potential errors:
```rust
use std::convert::TryFrom;

fn checked_cast_example(value: i32) {
    let result_u8 = u8::try_from(value);

    match result_u8 {
        Ok(u8_value) => {
            println!("Conversion successful: {}", u8_value);
        }
        Err(e) => {
            // Handle the error, e.g., print the error, return from the function, or panic.
            println!("Conversion failed: {}", e); 
        }
    }
}
```

<br>

## Characters
**Characters** in Rust are called `char`. A `char` is always **one character** and uses `' '` (single quotes) instead of `" "` (double quotes).<br>
All **chars** use **4 bytes** of memory, since *4 bytes* are enough to hold any kind of *character*.<br>

Rust can **safely cast** a `u8` into a `char`, using `as`.<br>
We **can’t** cast `i32` as a `char`, but we **can** cast an `i32` as a `u8` and then cast `u8` to `char`:
- `let a = 100_i32 as u8 as char;`

<br>

**Strings** are different and **don’t** always use **4 bytes** per single *character*. When a *character* is part of a `string` (**not** the `char` type), the `string` is encoded to use the **least amount of memory** needed for each *character*.<br>

The `.len()` method returns the *number of bytes*, **not** the *number of letters* or *characters*.

<br>

**Code**:
```rust
    println!("Size of a char: {}", std::mem::size_of::<char>());
    println!("Size of a: {}", "a".len());
    println!("Size of ß: {}", "ß".len());
    println!("Size of 国: {}", "国".len());
```
**Output**:
```bash
Size of a char: 4
Size of a: 1
Size of ß: 2
Size of 国: 3
```

<br>

**Code**:
```rust
    let str1 = "Hello";
    println!("str1 is {} bytes.", str1.len());
    let str2 = "안녕"; // Korean for “Hi”
    println!("str2 is {} bytes.", str2.len());
```
**Output**:
```bash
str1 is 5 bytes.
str2 is 6 bytes.
```

<br>

What about the size in characters/letters? There is methods `.chars().count()` that return the number of characters:
**Code**:
```rust
    let str1 = "Hello";
    println!("'{}' consists of {} bytes and {} characters.", str1, str1.len(), str1.chars().count());
    let str2 = "안녕";
    println!("'{}' consists of {} bytes BUT {} characters.", str2, str2.len(), str2.chars().count());
```
**Output**:
```bash
'Hello' consists of 5 bytes and 5 characters.
'안녕' consists of 6 bytes BUT 2 characters.
```

<br>

## Grapheme clusters
In Rust, a **grapheme cluster** **cannot** be a `char` because a `char` is defined as a **single** 4-byte code point of Unicode, whereas a **grapheme cluster** is a **sequence** of one or more **code points** that is displayed as a **single character**.<br>

**Namaste** (/na-ma-stay/) is the most common **greeting** in **Hindi**, suitable for both **formal** and **informal** situations.<br>
*Namaste* in **Sanskrit** is नमस्ते.<br>

The नमस्ते consists of **3 graphemes**:
- `न` (*na*): grapheme 1 and it corresponds to **1 code point**;
- `म` (*ma*): grapheme 2 and it corresponds to **1 code point**;
- `स्ते` (*ste*): grapheme 3 and it is comprised of **4 code point**: `स` + `्` + `त` + `े`;

<br>

**Example**:
```rust
fn main() {
    let namaste = "नमस्ते";

    println!("namaste.as_bytes() = {:?}", namaste.as_bytes());
    println!("namaste.len() = {}, namaste.as_bytes().len() = {}, namaste.chars().count() = {}", namaste.len(), namaste.as_bytes().len(), namaste.chars().count());
    namaste.chars().for_each(|ch| println!(r#"{} = {}, {:?}"#, ch, ch.escape_unicode(), ch.to_string().as_bytes()));
}
```
**Output**:
```rust
namaste.as_bytes() = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
namaste.len() = 18, namaste.as_bytes().len() = 18, namaste.chars().count() = 6
न = \u{928}, [224, 164, 168]
म = \u{92e}, [224, 164, 174]
स = \u{938}, [224, 164, 184]
् = \u{94d}, [224, 165, 141]
त = \u{924}, [224, 164, 164]
े = \u{947}, [224, 165, 135]
```

<br>

So, the नमस्ते consists of:
- **3** *graphemes*;
- **6** *chars* (or **6** *code points*);
- **18** *bytes*;

<br>

The following image displays the **bytes**, **code points** and **grapheme clusters** for the same word written in English (`hello`) and Hindi (`नमस्ते`):
![bytes-points-graphemes](/img/bytes-points-graphemes-2.png)

<br>

<br>

### Conversions
The `char` type in Rust **doesn't** have an `as_bytes` method because a `char` represents *fixed-size 4-byte* **Unicode codepoint** value before it has been encoded into some format, like **UTF-8**.<br>
But `char` has `to_string()` and `encode_utf8()` methods and thus `char` can be converted to **bytes** through `String` or `&str`:<br>
**Code**:
```rust
    let ch = '녕';

    let mut buf1 = [0u8; 4];
    let str = ch.encode_utf8(&mut buf1);
    let slice1 = str.as_bytes();

    let s1 = ch.to_string();
    let slice2 = s1.as_bytes();

    let s2 = ch.to_string();
    let vec = s2.into_bytes();

    println!("'{}' as bytes (ch.encode_utf8 -> &str -> s.as_bytes -> &[u8]): {:?}", ch, slice1);
    println!("'{}' as bytes (ch.to_string -> String -> s.as_bytes -> &[u8]): {:?}", ch, slice2);
    println!("'{}' as bytes (ch.to_string -> String -> s.into_bytes -> Vec<u8>): {:?}", ch, vec);
```
**Output**:
```bash
'녕' as bytes (ch.encode_utf8 -> &str -> s.as_bytes -> &[u8]): [235, 133, 149]
'녕' as bytes (ch.to_string -> String -> s.as_bytes -> &[u8]): [235, 133, 149]
'녕' as bytes (ch.to_string -> String -> s.into_bytes -> Vec<u8>): [235, 133, 149]
```

<br>

## Type inference
The compiler always needs to know the type of variables you are using, but most of the time, you don’t need to tell it.<br>
But the compiler is smart enough and it can usually infer the types that you are using.<br>

To specify a type, add a **colon** after the variable name and **type**:
```rust
fn main() {
  let small_number: u8 = 10;
}
```

For numbers, you can add the type after the number - just type it right after the number:
```rust
fn main() {
  let small_number = 10u8;
}
```

You can also add `_` if you want to make the number easy to read:
```rust
fn main() {
let small_number = 10_u8;
let big_number = 100_000_000_i32;
}
```

<br>

## Floats
**Floats** are numbers with decimal points. `5.5` is a float, `5.0` is also a float, and even `5.` is a float.<br>
Rust always chooses `f64` for **floats** unless you tell it to use an `f32`.<br>

<br>

## Declaring vars
We use the let keyword to declare a variable.

You can capture variables inside the `{}` of `println!`:
```rust
fn main() {
  let my_number = 8;
  println!("Hello, number {my_number}");
}
```

<br>

## Display and Debug
Simple variables in Rust can be printed with `{}` inside `println!`. This is called *Display printing*.<br>
But some variables won’t be able to use `{}` to print, and you need *Debug printing*.<br>

- `{}` *Display printing*
- `{:?}` *Debug printing*
- `{:#?}` for *Debug pretty printing* (*Pretty* means easier to read)

<br>

## Shadowing
**Shadowing** means using `let` to declare a **new variable** possibly of another type with the **same name**:
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

So, when you **shadow** a variable with a new variable with the same name, you **don’t** destroy the first one. You **block it**.<br>

<br>

# Chapter 02
## Variables
Any *variable* **lives as long as** *its code block*.<br>

<br>

## Pointers and references
The pointer you usually see in Rust is called a **reference**, which you can think of as a **memory-safe pointer**.<br>
A reference points to the memory of another value. A **reference** means you **borrow** the value, but you **don’t own it**.<br>
You can even have a reference to a reference or any number of references: thye are all different types.

To reach the place where the value is, we use `*`.<br>
Using `*` lets you **read** the **value behind** the **any** *reference*.<br>
Using `*` lets you **change** the **value behind** the *mutable reference*.<br>

Using `&` is called **referencing**, using `*` is called **dereferencing**.<br>

<br>

### Mutable references
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

## Strings
Rust has two main types of strings: `String` and `&str`.
- A `&str` is a simple string. It’s just a pointer to the data plus the length. It is also called a string slice. It might just be a **partial view** of the data owned by some
other variable, so just a **slice** of it.
    - `str` can be of **any length**;
    - `str` is a **dynamically sized type**. *Dynamically sized* means that the size can be different;
    - that's why we need an `&` because it makes a pointer, and **Rust knows the size of the pointer**;
- A `String` is a pointer with data on the heap. A `String` is easy to grow, shrink, mutate, and so on.

The biggest difference is that a `String` **owns** its data, while a `&str` is a **slice**. Because you use a `&` to interact with a `str`, you **don’t** *own* it.<br>
But a `String` is an **owned type**.<br>

Both `&str` and `String` are encoded with `UTF-8`.<br>

We can see this with two functions:
- `size_of`, which shows the **size of a type** in bytes;
- `size_of_val`, which shows the **size of a value** in bytes pointed to;

There are many ways to make a `String`:
- `String::from("This is the string text")`
- `"This is the string text".to_string()`
- `format!("My name is {}.", name)`

Another way to make a `String` is to call `.into()`, but it is a bit different because `.into()` isn’t for making a `String`; it’s for converting from one type into another type.<br>

Some types can easily convert to and from another type using `From::` and `.into()`; if you have `From`, you also have `.into()`.
`From` is clearer because you already know the types: for example `String::from("Some str")` you know that `String` is from a `&str`.
But with `.into()`, sometimes the compiler doesn’t know.
**Code**:
```rust
fn main() {
let my_string = "Try to make this a String".into();
}
```
**Output**:
```rust
error[E0282]: type annotations needed
```

It's because **many types can be made from** a `&str`. It is possible to make `&str` into a lot of things, so which one do you want?

<br>

## const and static
These are for values that **don’t change** (`const` means constant). Well, technically, `static` can change.<br>
The two main differences are
- `const` is for values that don’t change and are created at compile time.
- `static` is similar to `const` but
  - it has a **fixed memory location**;
  - it **might not be created** at compile time;

You write them with `ALL CAPITAL LETTERS` and **outside** of `main` so that they can **live for the whole program**.<br>

These types (`const` and `static`) are made at compile time and have restriction: they **can't** use the *heap* during compile time because the program needs to perform a memory allocation.<br>

<br>

## Shadowing again
Shadowing **doesn’t destroy** a value but **blocks** it? We can prove this:<br>
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

## Passing to functions
- `fn function_name(variable: String)` takes a `String` and **owns** it. If it **doesn’t return it**, then the variable **dies inside** the function. The value is **moved** into function.
- `fn function_name(variable: &String)` borrows a `String` and can **read** it. The variable **doesn’t die** inside the function.
- `fn function_name(variable: &mut String)` borrows a `String` and can **change** it. The variable **doesn’t die** inside the function.

<br>

It is possible to declare **mutable parameters** for functions `mut` before parameter name: `fn function_name(mut variable: &mut String)`.<br>

<br>

## Copy types
Rust’s simplest types are known as `Copy` types. They are all on the stack, and the **compiler knows their size**. That means that they are **very easy (cheap) to copy**, so the compiler
always copies their data when you send these types to a function. `Copy` types are **so small** and **easy** that there's no reason not to.<br>

`Copy` types are so **cheap** that you don’t need to worry about ownership.<br>

You also see the word **trivial** to talk about `Copy` types a lot, such as “It’s **trivial to copy them**.” That means: it’s so easy to copy them that there is no reason not to copy them.<br>
`Copy` types include `integers`, `floats`, `booleans` (true and false), `char`, and others.<br>

If it is a `Copy` type, the data would be **copied**, **not** **moved**.<br>
`Clone` is similar to `Copy` but usually needs more memory.<br>

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

## More about printing
Sometimes you end up using too many escape characters and just want Rust to print a string as you see it. To do this, you can add `r#` to the beginning and `#` to the end. The
`r` here stands for **raw**:
```rust
println!(r#"He said, "A =  "B";"#);
```

But what if `#` marks the end of the string and you need to print text with a `#"` inside? In that case, you can start with `r##` and end with `##`.<br>
You can **keep adding** `#` to the beginning and end if you have longer instances of the `#` symbol in your text.<br>

So when you add a `b` to print as follows,
```rust
fn main() {
    println!("{:?}", b"This will look like numbers");
}
```
you will get an output that shows all the bytes:
```bash
[84, 104, 105, 115, 32, 119, 105, 108, 108, 32, 108, 111, 111, 107, 32, 108,
105, 107, 101, 32, 110, 117, 109, 98, 101, 114, 115]
```

You can also put `b` and `r` **together** if you need to:
```rust
fn main() {
    println!("{:?}", br##"I like to write "#"."##);
}
```

There is also a *Unicode* **escape** that lets you print any *Unicode* **codepoint** inside a string: `\u{}`:<br>
**Code**:
```rust
fn main() {
println!("{:X}", '행' as u32);
println!("{:X}", 'H' as u32);
println!("{:X}", '居' as u32);
println!("{:X}", 'い' as u32);
println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");
}
```
**Output**:
```bash
D589
48
5C45
3044
행, H, 居, い
```

<br>


If you have a **reference**, you can use `{:p}` to print the **pointer address**:<br>
**Code**:
```rust
fn main() {
    let number = 9;
    let number_ref = &number;
    println!("{:p}", number_ref);
}
```
**Output**:
```bash
0x16f3aa214
```

<br>

You can print **binary**, **hexadecimal**, and **octal**:<br>
**Code**:
```rust
fn main() {
    let number = 555;
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", number, number, number);
}
```
**Output**:
```bash
Binary: 1000101011, hexadecimal: 22b, octal: 1053
```

<br>

You can also add **indexes** inside `{N}` to change the order of what gets printed. The **first** variable following the string will be in index **0**, the **second** in index **1**, and so on:
```rust
println!("This is {1} {2}, son of {0} {2}.", father_name, son_name, family_name);
```

You can also use a **name** instead of an index value to do the same thing:
```rust
fn main() {
    println!("{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
        city1 = "Seoul",
        city2 = "Busan",
        city3 = "Tokyo",
        country = "Korea"
);
}
```

<br>

**Complex printing** is also possible in Rust if you want to use it. The format is:
- `{variable:padding alignment width.precise}`

- Do you want a variable name? Write variable name **first**, like when we wrote `{country}`. Then add a `:` after it if you want to do more things.
- Do you want a **padding** character?
- What **alignment** (**left**/**middle**/**right**) do you want for the **padding**?
- A **width** define **max** length for the **padding**;
- A **precise** define **max** numbers to print **after** the **dot** for *floats*;
- Then, at the end, you can add a **question mark** `?` if you want to `Debug` print.

<br>

**Code**:
```rust
fn main() {
    let letter = "a";
    println!("{:-^1}", letter);
}
```
**Output**:
```bash
a
```

<br>

**Code**:
```rust
fn main() {
    let letter = "a";
    println!("{:-^2}", letter);
}
```
**Output**:
```bash
a-
```

<br>

**Code**:
```rust
fn main() {
    let letter = "a";
    println!("{:-^6}", letter);
}
```
**Output**:
```bash
--a---
```

<br>

**Code**:
```rust
fn main() {
    let letter = "a";
    println!("{:-^8.3}", 77.123456789);
    println!("{:-^6.3}", 77.123456789);
}
```
**Output**:
```bash
-77.123-
77.123
```

<br>

**Code**:
```rust
fn main() {
    let title = "TODAY'S NEWS";
    println!("{:-^30}", title);
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar);
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
```
**Output**:
```bash
---------TODAY'S NEWS---------
|                            |
SEOUL--------------------TOKYO
```

<br>

# Chapter 03
## Arrays
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

### Slices
To get slice of an array `arr` you need a `&` and range `..`: `&arr[start..end]`.<br>
The range like `&arr[start..end]` **doesn't** include index `end`, it's called **exclusive**.<br>
The range like `&arr[start..=end]` includes index `end`, it's called **inclusive**.<br>
The range like `&arr[..]` means **slice the whole array** and this is **NOT equivalent** to reference to array: `&arr`.<br>

**Example**:
```rust
fn main() {
    let arr = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let two_to_four = &arr[2..5]; // the type of two_to_four is &[u8]
    let start_at_one = &arr[1..]; // the type of start_at_one is &[u8]
    let end_at_four = &arr[..5]; // the type of end_at_four is &[u8]
    let end_at_five = &arr[..=5]; // the type of end_at_five is &[u8]
    let slice_of_all = &arr[..]; // the type of slice_of_all is &[u8]
    let ref_to_arr = &arr; // the type of ref_to_arr is &[u8; 10], NOT &[u8]
    println!("&arr[2..5]: {two_to_four:?}, len: {},
&arr[1..]: {start_at_one:?}, len: {},
&arr[..5]: {end_at_four:?}, len: {},
&arr[..=5]: {end_at_five:?}, len: {},
&arr[..]: {slice_of_all:?}, len: {}
&arr: {ref_to_arr:?}, len: {}", 
    two_to_four.len(), start_at_one.len(), end_at_four.len(), end_at_five.len(), slice_of_all.len(), ref_to_arr.len());
    take_slice(two_to_four);
    take_slice(ref_to_arr);
}

fn take_slice(s: &[u8]) {
    println!("s = {:?}", s);
}
```
**Output**:
```bash
&arr[2..5]: [2, 3, 4], len: 3,
&arr[1..]: [1, 2, 3, 4, 5, 6, 7, 8, 9], len: 9,
&arr[..5]: [0, 1, 2, 3, 4], len: 5,
&arr[..=5]: [0, 1, 2, 3, 4, 5], len: 6,
&arr[..]: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], len: 10
&arr: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], len: 10
s = [2, 3, 4]
s = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
```

<br>

## Vectors
Vectors have some pretty strict rules:
- vectors must only contain **elements of the same type**;
- vectors **can change their size** at the runtime;

There are **two** main ways to declare a vector.<br>

One is using constructors: `new` or `with_capacity`:
```rust
let mut my_vec = Vec::new();
my_vec.push(name1);
my_vec.push(name2);
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

### Slices
You can slice a vector too, just like an array:
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

### Reallocation
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

### Conversions
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

### Tuples
In Rust, this empty tuple `()` is called the **unit type**.<br>

Example of tuple definition:
```rust
fn main() {
    let random_tuple: (&'static str, i32, Vec<char>, char, [i32; 3], f64) = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
}
```

- `("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7)` is a **tuple**;
- the **tupe of tuple** `(&'static str, i32, Vec<char>, char, [i32; 3], f64)` depends on the types of the items inside it;

<br>

# Control flow
## Basic control flow
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

- *each branch* of a `if ... else` has to **return** the **same type**;

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

# Chapter 04
- use `struct` when you want *one thing* **and** *another thing*, i.e. if you have a *lot of things to group together*;
- use `enum` when you want *one thing* **or** *another thing*, i.e. if you have a *lot of choices and need to select one*;

The name of a `struct` and `enum` should be in **UpperCamelCase** (capital letter for each word with no spaces).<br>

<br>

## Structs
There are **three types** of structs:
- **unit struct**
  - **unit** means “doesn’t have anything” (like the **unit type**);
  - you **must write a semicolon** after a *unit struct*
- **tuple struct**
  - in *tuple struct* you **only** declare **types** inside a `()`;
  - *tuple struct* **doesn't** containt fields;
  - you **must write a semicolon** after a *tuple struct*
- **named struct**
  - in *named struct* you declare **field names** and **types** inside a `{}` code block;
  - you **don’t write a semicolon** after a *named struct* because there is a whole code block `{}` after it;
  - you **separate fields by commas** in a *named struct*, too, for the **last field**, you can add a comma or not — it’s up to you;

```rust
// unit struct
struct FileDirectory;

// tuple struct
struct ColorRgb(u8, u8, u8);

// named struct
struct Foo {
    age: u32,
    name: String,
}
```

<br>

**Instantiating structs**:
```rust
struct Foo {
    age: u32,
    name: String,
}

fn main() {
    // using values
    let foo1 = Foo {
        age: 10,
        name: "Anton".to_string(),
    };

    // using variables
    let age = 10;
    let name = "Anton".to_string();

    let foo1 = Foo {
        age: age,
        name: name.clone(),
    };

    // if the field name and variable name are the same, you don’t have to write both
    let foo2 = Foo {
        age,
        name,
    };
}
```

<br>

One nice convenience in Rust is that if the **field name** and **variable name** are the **same**, you **don’t have to write both**.<br>

<br>

## Enums
[**Enums reference**](https://doc.rust-lang.org/reference/items/enumerations.html).<br>

**Enumerations** are **declared** with the keyword `enum`.<br>
The `enum` consist of **enum variants**. There are **three types** of *enum variants*:
- **unit variants**
- **tuple variants** aka just **enum variant** - can be instantiated with a **struct expression**;
- **struct variants** (also called **struct-like variants** or **named-field variants**) - can be instantiated with a **struct expression**;

<br>

They differ in how they store data and how they are instantiated and pattern matched:
```rust
enum Examples {
    UnitLike,
    TupleLike(i32),
    StructLike { value: i32 },
}

use Examples::*; // Creates aliases to all variants.
let x = UnitLike; // Path expression of the const item.
let x = UnitLike {}; // Struct expression.
let y = TupleLike(123); // Call expression.
let y = TupleLike { 0: 123 }; // Struct expression using integer field names.
let z = StructLike { value: 123 }; // Struct expression.
```

<br>

**Example** (**struct variants**):
```rust
enum Student { Junior { id: u32 } }
let me = Student::Junior { id: 5 };
match me {
    Student::Junior { id: id_val @ 0..=10 } => println!("Junior in range: {}", id_val),
    _ => println!("Other student"),
}
```

<br>

A **unit-only enum** is an `enum` with **no** *tuple* and *struct* variants:
```rust
enum UnitOnlyEnum {
    Foo,
    Bar,
    Baz,
}
```

<br>

A **fieldless enum** is an **unit-only enum** with empty *tuple* and *struct* variants:
```rust
enum Fieldless {
    Tuple(),
    Struct {},
    Unit,
}
```

<br>

### Discriminants
Each **enum** instance has a **discriminant**: an integer logically associated with each variant of the **enum**.<br>
Under the Rust representation, the discriminant is interpreted as an `isize` value. However, the *compiler is allowed to use a smaller type* in its actual memory layout.<br>

- if a *discriminant* for a variant is not specified, then it is set to **one higher** than the *discriminant* of the **previous** variant in the declaration;
- if a *discriminant* of the **first** variant in the declaration is **unspecified**, then it is set to **zero**;
- a *discriminant* of a variant may be explicitly set for **unit-only enums**;
```rust
enum Foo {
    Bar,            // 0
    Baz = 123,      // 123
    Quux,           // 124
}
```

<br>

### Casting enums into integers
If an enumeration is **unit-only** (with no tuple and struct variants), then its *discriminant* can be directly accessed through **cast**:
```rust
fn main() {
    enum Enum {
        Foo,
        Bar = 10000,
        Baz,
    }

    assert_eq!(0, Enum::Foo as isize);
    assert_eq!(10000, Enum::Bar as u32);
    assert_eq!(10001, Enum::Baz as u16);
}
```



```rust
enum Animal {
    Pet,
    Dog (String, f64),
    Cat { name: String, weight: f64 },
}
```

<br>

### Importing enum variants
With the `use` keyword, you can also “import” an enum variants inside code block or function.<br>
Consider `Mood` is a enum, then `use Mood::*;` brings all its variants into currect scope:
```rust
enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry,
}

fn match_mood(mood: &Mood) -> i32 {
    match mood {
        Mood::Happy => 10,
        Mood::Sleepy => 6,
        Mood::NotBad => 7,
        Mood::Angry => 2,
    }
}

fn match_mood2(mood: &Mood) -> i32 {
    use Mood::*;   // This imports every variant directly inside current scope
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2,
    }
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {happiness_level}");
    let happiness_level = match_mood2(&Mood::Happy);
    println!("Out of 1 to 10, my happiness is {happiness_level}");
}
```

<br>

### Holding different types in collection
Enum allows to hold **different types** *in one collection*.<br>

**Example**:
```rust
enum Number {
    I32(i32),
    U32(u32),
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input),
    }
}

fn main() {
    let v = vec![get_number(-100), get_number(100)];

    for n in v {
        match n {
            Number::I32(n) => println!("Type i32, value {}", n),
            Number::U32(n) => println!("Type u32, value {}", n),
        }
    }
}
```

<br>

### Implementing structs and enums
There are **two kinds** of methods in an `impl` block:
- **methods**
  - they all take `self` in some form (`&self` or `&mut self` or `self`);
  - *regular methods* are called by `.`;
    - example: `a.clone()`;
- **associated functions** (aka **static methods** in some languages)
  - they d**o not take** `self`;
  - *associated functions* are called by typing `::` in *between* the **type name** and the **function name**;
    - examples: `String::from()` and `Vec::new()`;

<br>

### References and the dot operator
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

# Chapter 05

<br>

# Chapter 06
All **collections** are contained in the `std::collections` module in the *standard library*.<br>

- `HashMap`
  - it is a *collection* made out of *keys* and *values*;
  - `HashMap` is an **unordered** *collection*, i.e. it **doesn't** order its *keys*;
- `BTreeMap`
  - if you want the **keys to be ordered** use `BTreeMap`;
  - `BTreeMap` is like an **ordered** `HashMap`, i.e. it **orders** its *keys*;
- `HashSet`
  - it is like a `HashMap` but **without** *values*;
  - it is implemented as a `HashMap` where the *value* is `()`;
  - `HashSet` is an **unordered** *collection*, i.e. it **doesn't** order its *keys*;
- `BTreeSet`
  - if you want the **keys to be ordered** use `BTreeSet`;
  - `BTreeSet` is like an **ordered** `HashSet`, i.e. it **orders** its *keys*;
- `BinaryHeap`
  - it is a **priority queue**;
  - a `BinaryHeap` always has the **largest value** *at the front*, everything else is **unsorted**;
- `VecDeque` (pronounced /vec-deck/)
  - it is like a `Vec` that is **optimized for** *popping items both off* the **front** and the **back**;

<br>

## HashMap
```rust
use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

fn main() {
    let mut foo = City {
        name: "foo".to_string(),
        population: HashMap::new(),
    };
    foo.population.insert(2020, 437_619);
    foo.population.insert(1372, 3_250);
    foo.population.insert(1851, 24_000);
    for (year, population) in foo.population {
        println!("In {year}, Foo had a population of {population}.");
    }
}
```

<br>

The simplest way to get a value in a `HashMap` is by putting the key in `[]` square brackets, **similar** to a `Vec`.<br>
But be careful because the program will **crash** if there is **no** key, just like when indexing a `Vec`.<br>
If you are not sure there will be a key, you can use `.get()`, which returns a **reference** *to the value corresponding to the key* (`Option<&V>`):
- if *key* **exists**, it will be `Some(&value)`
- if **not**, you will get `None`;

<br>

If a `HashMap` already has a *key* when you try to put it in, using `.insert()` will **overwrite** its *value*.<br>
Also the `.insert()` returns an `Option` that holds the **old** value **if** the value was overwritten.<br>

```rust
use std::collections::HashMap;
fn main() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "foo");
    book_hashmap.insert(1, "bar");
    book_hashmap.insert(1, "hello");
    println!("{:?}", book_hashmap.get(&1));
}
```
**Output**:
```bash
Some("hello")
```

<br>

In the next sample, we will have a `Vec` that will **hold any old values** that have been returned by the `.insert()`:
```rust
use std::collections::HashMap;
fn main() {
    let mut book_hashmap = HashMap::new();
    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "foo"),
        (1, "bar"),
        (1, "hello"),
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting `{old_value}` with `{value}`!");
            old_hashmap_values.push(old_value);
        }
    }
    println!("All old values: {old_hashmap_values:?}");
}
```

<br>

To **prevent** *overwriting* we **must check** *whether an entry exists*:
```rust
use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();
    // book_hashmap.insert(1, "foo");
    let key = 1;

    // variant 1
    match book_hashmap.get(&key) {
        Some(val) => (),
        None => {book_hashmap.insert(1, "bar");},
    }

    // variant 2
    if let None = book_hashmap.get(&key) {
        book_hashmap.insert(1, "hello");
    }

    println!("{:?}", book_hashmap.get(&1));
}
```

<br>

### The .entry() api
With `.entry()`, you can try to make an entry and then another method like `.or_insert()` to insert a **default** *value* if there is **no** *key*.<br>
The `.or_insert()` method either returns a **mutable reference** to the *existing value*, or it *inserts the default value* and then returns a **mutable reference** to it.<br>
That means you can use `let` to **assign** the *mutable reference* to a *variable name* and **change** the *value* in the `HashMap`.<br>

<br>

**Count the words example**:
```rust
use std::collections::HashMap;

fn main() {
    let words = vec![
        "foo",
        "bar",
        "hello",
        "hello",
    ];
    let mut words_map = HashMap::new();
    for word in words {
        let counter = words_map.entry(word).or_insert(0_u32);
        *counter += 1;
    }
    for (word, counter) in words_map {
        println!("{word}: {counter}");
    }
}
```

<br>

**Join answers per gender and print all answers per gender**:
```rust
use std::collections::HashMap;

fn main() {
    let data = vec![
        ("male", 1),
        ("female", 2),
        ("male", 3),
        ("female", 3),
        ("male", 5),
        ("female", 8),
    ];
    let mut survey = HashMap::new();
    for item in data {
        let counter = survey.entry(item.0).or_insert(Vec::with_capacity(8)).push(item.1);
    }
    for (gender, answers) in survey {
        println!("{gender}: {answers:?}");
    }
}
```
**Output**:
```rust
female: [2, 3, 8]
male: [1, 3, 5]
```

<br>

The `.entry()` only takes a *key* and then returns an *enum* called `Entry`, `K` means *key*, and `V` means *value*:
```rust
pub fn entry(&mut self, key: K) -> Entry<K, V>

enum Entry<K, V> {
    Occupied(OccupiedEntry<K, V>),
    Vacant(VacantEntry<K, V>),
}
```

<br>

The *next method*, `.or_insert()`, is a method on the `Entry` *enum*: it looks at the *enum* and decides what to do:
```rust
fn or_insert(self, default: V) -> &mut V {
    match self {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => entry.insert(default),
    }
}
```

<br>

## BTreeMap
We can quickly change our `HashMap` to a `BTreeMap`, because their **methods** and **signatures** are very **similar**:
```rust
use std::collections::BTreeMap;

struct City {
    name: String,
    population: BTreeMap<i32, i32>,
}

fn main() {
    let mut foo = City {
        name: "foo".to_string(),
        population: BTreeMap::new(),
    };
    foo.population.insert(2020, 437_619);
    foo.population.insert(1372, 3_250);
    foo.population.insert(1851, 24_000);
    for (year, population) in foo.population {
        println!("In {year}, Foo had a population of {population}.");
    }
}
```

<br>

## HashSet and BTreeSet
```rust
use std::collections::HashSet;

const TOTAL: usize = 50;

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21, 20, 38, 16, 48, 39,
        31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9, 46, 33,
    ];
    println!("How many numbers in the Vec? {}", many_numbers.len());
    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!("There are {hashset_length} unique numbers, so we are missing {}.", TOTAL - hashset_length);

    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            println!("{number} is missing");
        }
    }
}
```

<br>

## BinaryHeap
**Tuples** are compared **element by element**, **from left to right**. The comparison **stops** at the **first** *differing element*:
```rust
fn main() {
    let t1 = (1, "x");
    let t2 = (2, "a");
    let t3 = (2, "a");
    let t4 = (1, "z");

    assert_eq!(t2, t3);
    assert!(t4 < t2); // the first element in t4 is LESS THAN in t1: 1 < 2
    assert!(t4 > t1); // the first elements are equal: 1 = 1 and the second element in t4 is GREATER THAN in t1: 'z' > 'x' 
}
```

<br>

A good way to use a `BinaryHeap` is for jobs/tasks.<br>
By default, `BinaryHeap` is a **max-heap**, meaning the element with the **highest priority** (**greatest value**) is popped first.<br>
For **tuples**, *highest priority* means the one that is considered **greatest** based on sequential comparison of its elements.<br>

<br>

The `.pop()` removes item with the **greater** value and returns `Option<Item>`:
```rust
use std::{collections::BinaryHeap};

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "foo"));
    jobs.push((80, "bar"));
    jobs.push((5, "xyz"));
    jobs.push((70, "abc"));
    jobs.push((30, "qwerty"));

    while let Some((priority, payload)) = jobs.pop() {
        println!("Job with priority {}, payload: {}", priority, payload);
    }
}
```

<br>

The `.ppeekop()` gets item with the **greater** value and returns `Option<Item>`:
```rust
use std::{collections::BinaryHeap};

fn main() {
    let mut jobs = BinaryHeap::new();

    jobs.push((100, "foo"));
    jobs.push((80, "bar"));
    jobs.push((5, "xyz"));
    jobs.push((70, "abc"));
    jobs.push((30, "qwerty"));

    if let Some((priority, payload)) = jobs.peek() {
        println!("Job with priority {}, payload: {}", priority, payload);
    }
}
```

<br>

## VecDeque
The `Vec::remove(index)` shifts over the remaining elements one step left and it has a worst-case performance of **O(n)**.<br>
The `VecDeque::remove(index)` is much **faster** than *Vec's* `.remove(index)` and it is safe, it returns `None` if `index` is **out of bounds**.<br>
The `VecDeque::pop_front()` is much **faster** than *Vec's* `.remove(0)`.<br>

**Example**:
```rust
use std::{collections::VecDeque, time::{Duration, Instant}};

fn main() {
    let mut my_vec = Vec::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.remove(0);
        }
    });
    println!("Overall time for Vec::remove(): {:#?}", r.0);
    let mut my_vec = Vec::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.pop();
        }
    });
    println!("Overall time for Vec::pop(): {:#?}", r.0);
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.remove(100000);
        }
    });
    println!("Overall time for VecDeque::remove(): {:#?}", r.0);
    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    let r= time(|| {
        for i in 0..600_000 {
            my_vec.pop_front();
        }
    });
    println!("Overall time for VecDeque::pop_front(): {:#?}", r.0);
}

pub fn time<F, T>(f: F) -> (Duration, T)
where F: FnOnce() -> T 
{
  let now = Instant::now();
  let res = f();
  let elapsed = now.elapsed();
  (elapsed, res)
}
```
**Output**:
```bash
Vec::remove(0): 14.531206s
Vec::pop(): 288.75µs
VecDeque::remove(0): 937.875µs
VecDeque::pop_front(): 749.583µs
```

<br>

## The `?` operator
After anything that returns a `Result` or `Option`, you can add `?`. This will:
- automatically **pulls out** the `Ok` value from a `Result`;
- if the value inside `Result` is `Err` it will **exit the function early** (**early return**) and return `Err` of the `Result` of function's returning type

<br>

We **don’t** need to write `std::result::Result` because `Result` is **always in scope**.<br>

<br>

```rust
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<u32, ParseIntError> 
{
    let parsed_number = input.parse::<u32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn main() {
    let input = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in input {
        let parsed = parse_str(item);
        println!("{parsed:?}");
    }
}
```

<br>

Imagine that you want to *take some bytes*, turn *them into* a `String`, and then *parse* it into a *number*. **First**, you need to successfully create a `String` from the bytes using a method called `String::from_utf8()`. **And then** it needs to successfully parse into a number.<br>

**The problem is the return type**:
- if `String::from_utf8()` **fails**, it will return `Err(FromUtf8Error)`;
- and if `string.parse()` **fails**, it will return an `Err(ParseIntError)`;
- but we **can’t return** a `Result<i32, ParseIntError or FromUtf8Error>`;

What must be in the place of `????`:
```rust
use std::num::ParseIntError;
use std::string::FromUtf8Error;

fn turn_into_string_and_parse(bytes: Vec<u8>) -> Result<i32, ????> {
    let num = String::from_utf8(bytes)?.parse::<i32>()?; // Two possible errors can be returned here
    Ok(num)
}
```

<br>

# Chapter 07
Rust uses a special syntax called **attributes** to automatically implement traits like `Debug` because they are so common.<br>
```rust
#[derive(Debug)]
struct MyStruct {
    number: usize,
}
```

<br>

But other traits are more difficult for the compiler to guess, so you **can’t** use `derive` to implement them. Those traits **need to be manually implemented** with the
`impl` keyword. A good example is the `Add` trait (found at `std::ops::Add`), which is used to add two things. Any type that implements the `Add` trait can use the `+` operator to add.<br>

<br>

To make a *trait*, write `trait` and then create some methods for it.<br>
You can **just** write the function **signature** when making a *trait* or provide **default implementation** of method which **can be overwritten further**.<br>
*Traits* can be **empty**, aka **marker traits**:
```rust
trait X {}
trait Y {}
```

So when you create a trait, you must think: *Which methods should I write? And which ones should the user write?*:
- if you think **most users will use the methods the same way every time**, it makes sense for you to write a **default method** inside the trait;
- but if you think that **users will use the methods differently every time**, write the **signature**;

<br>

We can pass `&self` inside methods, but we **can’t** do much with it. That’s because Rust **doesn’t** know what type is going to use it. For example, we **can't** access any field on the `self` inside **default implementation** of method.<br>

But we can add *trait bounds* to the trait:
```rust
trait A: B {

}
```

The above code means any type that implements `A` must implement `B` and it allows to call `B`'s methods on `self` in the **default implementations**.<br>

<br>

## `From` trait
With `From`, you can make a `String` from a `&str`, but you can make many types from many other types.<br>

<br>

## The orphan rule
The **orphan rule**:
- you **can** implement **your** *trait* on **someone else’s** *type*;
- you **can** implement **someone else’s** *trait* on **your** *type*;
- **however**, you **can’t** implement **someone else’s** *trait* on **someone else’s** *type*;

<br>

The best way to **get around** the *orphan rule* is to **wrap** *someone else’s type* in a **tuple struct**, thereby creating an entirely **new type**.<br>
This is called the **newtype pattern**.<br>

<br>

## `AsRef` trait
The `AsRef` trait is used to *give a reference* **from** *one type* **to** *another type*.<br>

Both `String` and `str` implement `AsRef<str>`. Here is how they do it:
```rust
impl AsRef<str> for str {
    fn as_ref(&self) -> &str {
        self
    }
}
```

```rust
impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        self
    }
}
```

<br>

Example: a function that can take **both** a `String` and a `&str`:
```rust
fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
```
**Output**:
```rust
Here is the error: error[E0277]: `T` doesn't implement `std::fmt::Display`.
```

We got this **error** because `T` is a type that implements `AsRef<str>`, but `T` **may** or **may not** implement `Display`.<br>
But we can turn it into a reference to a `str`, thanks to the `AsRef` trait. To do that, call the **trait’s method**: `.as_ref()`.<br>
```rust
fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref())
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
```

<br>

# Chapter 08
## Iterators and loops
A `for` loop can receive **iterator** or **iterable** (because every **iterator** is **iterable**):
- `.into_iter()` for an **iterator of owned values**;
  - the `for num in vector` is the same as writing `for num in vector1.into_iter()` - it iterates over **owned values**, and `vector1` **no longer exists** after this for loop is done;
- `.iter()` for an **iterator of references**;
  - the `for num in mut vector` is the same as writing `for num in vector.iter()` - it iterates over **immutable references**, so `vector` **still exists** after it is over;
- `.iter_mut()` for an **iterator of mutable references**;
  - the `for num in &mut vector` is the same as writing `for num in vector.iter_mut()` - it iterates over **mutable references**, so `vector` **still exists** after it is over;

<br>

The core of every iterator is a method called `.next()`, which returns an `Option`. When you use an iterator, it calls `.next()` over and over:
- if `.next()` returns `Some`, there are still items left, and the iterator **keeps going**;
- if `None` is returned, the iteration is **finished**;

An iterator gives out a bunch of `Somes` until it is out of items, and then it **only** gives `None`. This is how the `for` loop knows when to stop.<br>

If you wish, you can also **manually call** `.next()` on an iterator:
```rust
fn main() {
    let my_vec = vec!['a', 'b', 'c', 'd'];
    let mut my_vec_iter = my_vec.iter();
    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'c'));
    assert_eq!(my_vec_iter.next(), Some(&'d'));
    assert_eq!(my_vec_iter.next(), None); // Now the iterator is out of items, so it returns None.
    assert_eq!(my_vec_iter.next(), None); // You can keep calling .next() on the iterator, and it will simply return None every time.
}
```

<br>

It is possible to make iterators that **never** return `None`, **only** return `None`, and so on.<br>

Here’s an iterator that just gives the number 1 forever:
```rust
struct GivesOne;
impl Iterator for GivesOne {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        Some(1)
    }
}

fn main() {
    let mut my_vec_iter = GivesOne;

    assert_eq!(my_vec_iter.next(), Some(1));
    assert_eq!(my_vec_iter.next(), Some(1));
    assert_eq!(my_vec_iter.next(), None); // this panics!
}
```

But you can use the `.take(N)` method to only call it `N` times.<br>

**Note** that the `GivesOne` struct **doesn’t hold anything**. It’s a good example of one of the ways that an *iterator* **differs** from a *collections*. In this case, the `GivesOne` struct is just an **empty struct** that implements the `Iterator` trait.<br>

<br>

## Closures and closures inside iterators
**Closures** are functions that don’t need a name — in other words, **anonymous functions**. Sometimes they are called **lambdas** in other languages.<br>

You can **bind a closure to a variable**, and then it looks exactly like a function when you use it:
```rust
fn main() {
    let my_closure = || println!("This is a closure");
    my_closure();
}
```

In the above example closure takes nothing: `||`.<br>

In between the `||`, we can add **signature of closure** for its **input variables**:
```rust
fn main() {
    let my_closure = |x: i32| println!("{x}");
    my_closure(5);
    my_closure(5+5);
}
```

<br>

For longer *closures*, you can add a **code block**: `|| {}`.<br>
Also you can add a **returning value** in the signature: `|| -> u64 {}` or `|x: i32| -> u64 {}`.<br>

One thing that makes *closures* special is that they **can capture variables** from their environment that are outside the closure, even if you only write `||`.<br>
You can think of a closure as a standalone type that can hold references in the same way that a struct can.<br>

<br>

Usually you see *closures* in Rust inside of methods because it is very convenient to have a closure inside. The convenience comes from the fact that the user can write the **body** of the *closure* **differently** each time, *depending on the situation*.<br>

<br>

## Closures: lazy and fast
A classic example of using `.map()` to make a new `Vec` from an existing `Vec`:
```rust
fn main() {
    let num_vec = vec![2, 4, 6];
    let double_vec: Vec<i32> = num_vec
    .iter()
    .map(|num| num * 2)
    .collect();
}
```

That was pretty easy and prints `[4, 8, 12]`. But let’s see what happens when we **don’t collect** into a `Vec`: the compiler **issues is a warning**:
```rust
fn main() {
    let num_vec = vec![2, 4, 6];
    num_vec
    .iter()
    .enumerate()
    .map(|(index, num)| format!("Index {index} is {num}"));
}
```
**Output**:
```rust
warning: unused `Map` that must be used
 --> chapter_03/src/main.rs:3:5
  |
3 | /     num_vec
4 | |     .iter()
5 | |     .enumerate()
6 | |     .map(|(index, num)| format!("Index {index} is {num}"));
  | |__________________________________________________________^
  |
  = note: iterators are lazy and do nothing unless consumed
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
3 |     let _ = num_vec
  |     +++++++
```

<br>

Consider **chain of 3 methods**:
```rust
num_vec.iter().enumerate().map()
```

<br>

Rust avoids this sort of operation:
- **iterate** over all the items in the `Vec`;
- **enumerate** over all the items from the iterator;
- **map** over all the enumerated i32s;

<br>

Instead, an iterator with a *method* and another *method* and another *method* simply *creates* a **single structure** and waits until we decide what to do with it:
- `let num_vec = vec![2, 4, 6];` it is a `Vec<i32>`;
- `.iter()` now it is an `Iter<i32>`;
- `.enumerate()` now it is an `Enumerate<Iter<i32>>`;
- `.map()` now it is a `Map<Enumerate<Iter<i32>>>`;

If we add `.collect::<Vec<i32>>()`, it knows what to do.<br>

This is one of the ways that Rust keeps even fancy functional-looking code **as fast as** any other kind of code. This is an example of an idea in Rust called **zero-cost abstractions**.<br>

<br>

## `|_|` in a closure
The `|_|` in a closure means that the closure needs to take an argument, but you don't want to use it.<br>

<br>

# Chapter 09
## Option/Result methods
- `.ok()` it converts **from** `Result<T, E>` **to** `Option<T>`
  - What if you have a function that returns a `Result` but you must to return an `Option`. No problem: there is a method called `.ok()`;
  - all that can be passed on from a `Result` to an `Option` is the information inside an `Ok` variant and because `None` **doesn’t hold** any information any `.ok()` **drops any error** inside `Err` variant;
- `.ok_or(error)` and `.ok_or_else()`:
  - both **turn** an `Option` **into** `Result`;
  - because `None` in an `Option` **doesn’t have** any information, but `Result` can have `Ok` **or** `Err`, so you have to let it know what the `error` **value** will be for `None`;
  - `ok_or_else(|| ...)` receives the **closure**;
- `.unwrap_or(value)` returns a default `value` if an `Option` is a `None` or `Result` is an `Err`;
- `.unwrap_or_else(|| ... )` allows us to give a default value, but it uses a **closure** that we can use to write some more **complex logic**;

<br>

## `.unwrap_or()` and  `.unwrap_or_else()`
Consider example:
```rust
fn main() {
    let v = vec![1, 2, 3];

    let fourth = v.get(3).unwrap_or_else(|| {
        if let Some(val) = v.get(2) {
            val
        }
        else {
            &0
        }
    });
    println!("{}", fourth);
}
```

*First*, we try to get an item at index **3**, *then* if it was `None` we try to get item one index back and *finally* we return a `&0` in case **no items have been found** at either index.<br>

<br>

<br>

## Iterator methods chaining
- `.filter()` it receives **clousre** that returns `bool` (aka **predicate**) and creates iterator that **yields only** the **items** for which the **predicate** returns `true`;
- `.filter_map()` it receives **closure** that returns `Option<T>` and creates iterator that **yields only** the `value`s for which the closure returns `Some(value)` and **filters out** everything that is `None`;
- `.and_then()` this method’s input is an `Option`, and its output is also an `Option`;
  - take **value** if `Some`, do something to the value, and wrap again to `Some`;
- `.and()` matches many `Option`s in the chain
  - if they are **all** `Some`, it will give the **last one**;
  - if **one** of them is a `None`, it will give `None`;
- the `.flatten()` method is a convenient way to **filter out** `None` or `Err` values in an iterator and only return the successful values;
  - `.flatten()` method of `Iterator` requires its items to implement `IntoIterator`;
- `.any(predicate)` returns a `true` if a **predicate** closure returns `true` for **any** of the item;
  - `.any()` only checks **until** it finds the **first** matching item, and then it **stops** — there’s no need to check the rest of the items;
- `.all(predicate)` returns a `true` if a **predicate** closure returns `true` for **all** of the item;
- `.rev()` you can use `.rev()` after `.iter()` to **reverse the iterator**;
- `.find()` returns an `Option` with the **item** inside or `None`;
- `.position()` finds an item that matches condition and returns an `Option` with the **index of item** inside or `None`;
- `.cycle()` creates an iterator that **loops forever**;
- `.zip()` joins two iterators and returns tuple of two values;
- `.fold()` is similar to `.for_each()` except that it returns a **final value** at the end;
  - when using `.fold()`, you first add a **starting value**, then a comma, and then the **closure**;
  - the *closure* has you **two inputs**: the **total so far** and the **next item**;
  - `.fold()` **not only **for adding numbers, it is possible to use **complex structs** in `.fold()`;
- `.cloned()` makes a clone inside the iterator; this **turns** a *reference* **into** a *value*;
- many other **_while** methods:
  - `.take_while(predicate)` takes item **as long as** predicate returns `true`;
  - `.skip_while()` skips item **as long as** predicate returns `true`;
  - `.map_while()` 
- `.sum()` adds everything together
- `.by_ref()` the most of methods take `self` and **consume the whole iterator**, however `.into_iter().by_ref()` takes `&self`;
  - `.by_ref()` allows to avoid error `error[E0382]: use of moved value: `foo``
- `.chunks(N)`
  - `.chunks(0)` will **panic** if you give it **0**;
  - let’s say you have a **vector** `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]`, then `.chunks(3)` will give you **4** slices:
    - `[0, 1, 2]`
    - `[3, 4, 5]`
    - `[6, 7, 8]`
    - `[9]`
- `.windows(N)` let’s say you have a **vector** `[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]` then `.windows(3)` will give you **8** slices:
  - `[0, 1, 2]`
  - `[1, 2, 3]`
  - `[2, 3, 4]`
  - `[3, 4, 5]`
  - `[4, 5, 6]`
  - `[5, 6, 7]`
  - `[6, 7, 8]`
  - `[7, 8, 9]`
- `.match_indices()` it is similar to `.enumerate()` because it returns a **tuple** with two items **index** and **matched value**;
  - lets you pull out everything inside a  that **matches your input** and gives you the **index**, too;
- `.peekable()` and `.peek()`
  - `.peekable()` creates a type of iterator called **peekable**, which has the `.peek()` method (**regular** iterators **can’t** use `.peek()`);
  - `.peek()` allows you get the **next** item **without** moving iterator futher, i.e. it returns `Option` like `.next()` except that the iterator **doesn’t move**, so you can use it as many times as you want;
- `.inspect()` is similar to `dbg!`, but it is used in iterators in a similar fashion to `.map()`: it simply gives you the item to look at, which lets you print it or do whatever you want;
- `.collect()` transforms iterator to collection of some type;
- `.skip(N)` skips over `N` items;
- `.take(N)` takes the first `N` items;
- `.map()` lets you do something to every item (including turning it into a different type) and then pass it on to make a new iterator;
- `.for_each()` lets you do something with every item **without** creating a new iterator;
  - it allows **modify items** in the **original** `Vec` **without** nedding to make a **new** `Vec`;
- `.enumerate()` zips items with their indices, in other words it coverts original collection to collection of tuples `(usize, item)`;

<br>

The `.iter()`/`.iter_mut()` plus `.for_each()` is basically a `for` loop.<br>

## `.for_each()`
In the example below we don’t need to use `.collect()` to create a new `Vec`, because we change items directly in the original collection:
```rust
vector2.iter_mut().for_each(|x| *x +=100);
```
Thus, `vector2` is still there after the iterator is over.

<br>

## `.skip()` and `.take()`
```rust
fn main() {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let new_vec = my_vec.into_iter().skip(2).take(2).collect::<Vec<i32>>();
    println!("{new_vec:?}");

    let v = (1..).skip(10).take(5).collect::<Vec<i32>>();
    println!("{v:?}");
}
```
**Output**:
```bash
[2, 3]
[11, 12, 13, 14, 15]
```

<br>

### .enumerate()
**Example**:
```rust
fn main() {
    let char_vec = vec!['z', 'y', 'x'];
    char_vec
    .iter()
    .enumerate()
    .for_each(|(index, c)| println!("Index {index} is: {c}"));
}
```
**Output**:
```bash
Index 0 is: z
Index 1 is: y
Index 2 is: x
```

<br>

## `.filter_map()` and .ok()
Anything that returns an `Err` becomes `None` after the `.ok()` method and then **gets filtered out** by `.filter_map()`:
```rust
fn main() {
    let user_input = vec![
        "8.9",
        "Nine point nine five",
        "8.0",
        "7.6",
        "eleventy-twelve",
    ];
    let successful_numbers = user_input
        .iter()
        .filter_map(|input| input.parse::<f32>().ok())
        .collect::<Vec<f32>>();
    println!("{:?}", successful_numbers);
}
```
**Output**:
```python
[8.9, 8.0, 7.6]
```

<br>

## `.ok_or(error)`
```rust
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Foo", "A"),
        Company::new("Bar", "B"),
        Company::new("Fizzbazz", ""),
    ];

    let results: Vec<Result<String, &str>> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or("No CEO found"))
        .collect();

    for item in results {
        println!("{:?}", item);
    }
}
```
**Output**:
```python
Ok("A")
Ok("B")
Err("No CEO found")
```

<br>

The `.map(|company| company.get_ceo().ok_or("No CEO found"))` means
- if `.get_ceo()` returns a `Some(value)`, put the `value` as is inside `Ok`;
- if `.get_ceo()` returns a `None`, put the `No CEO found` inside `Err`;

<br>

## `.ok_or_else(error)`
```rust
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Foo", "A"),
        Company::new("Bar", "B"),
        Company::new("Fizzbazz", ""),
    ];

    let results: Vec<_> = company_vec
        .iter()
        .map(|company| company.get_ceo().ok_or_else(|| {
            let err_msg = format!("No CEO found for {}", company.name);
            println!("[[ERROR]]: {}", err_msg);
            err_msg
        }))
        .collect();

    results
        .iter()
        .filter(|i| i.is_ok())
        .for_each(|i| println!("{:?}", i));
}
```
**Output**:
```python
[[ERROR]]: No CEO found for Fizzbazz
Ok("A")
Ok("B")
```

<br>

## `.and_then()`
```rust
fn main() {
    let num_array = ["8", "9", "Hi", "9898989898"];
    let mut char_vec = vec![];
    for index in 0..5 {
        char_vec.push(
        num_array
            .get(index)
            .and_then(|number| number.parse::<u32>().ok())
            .and_then(|number| char::try_from(number).ok())
        );
    }
    println!("{:?}", char_vec);
}
```
**Output**:
```python
[Some('\u{8}'), Some('\t'), None, None, None]
```

<br>

## `.and()`
```rust
fn main() {
    let try_1 = [1,2,3];
    let try_2 = ["a", "b", "c"];
    let try_3 = [111, 222];
    for i in 0..try_1.len() {
        println!("{:?}", try_1.get(i).and(try_2.get(i)).and(try_3.get(i)));
    }
}
```
**Output**:
```bash
Some(111)
Some(222)
None
```

<br>

## `.zip()`
### `.zip()` and `.cycle()`
```rust
fn main() {
    let inf_iter = ["even", "odd"].into_iter().cycle();
    let even_odd: Vec<(i32, &str)> = (0..=5)
        .zip(inf_iter)
        .collect();
    println!("{:?}", even_odd);
}
```
**Output**:
```python
[(0, "even"), (1, "odd"), (2, "even"), (3, "odd"), (4, "even"), (5, "odd")]
```

<br>

This iterator `["even", "odd"].into_iter().cycle()` will return `Some("even")` and `Some("odd")` **forever**. It will never return `None`.<br>
Even though `inf_iter` will never end, the other iterator only runs **six times** and thus the final `Vec` also only has six items.<br>

<br>

### `.zip()` and `.collect()` into HashMap
```rust
use std::collections::HashMap;
fn main() {
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values)
        .collect::<HashMap<_, _>>();
    println!("The value at key 2 is: {}", number_word_hashmap.get(&2).unwrap());
}
```

<br>

You can see that we wrote `<HashMap<_, _>>` because that is enough information for Rust to decide on the type `HashMap<i32, &str>`.<br>
You can write `.collect::<HashMap<i32, &str>>()` if you want.<br>

<br>

## `.fold()` numbers
```rust
fn main() {
    let some_numbers = vec![9, 6, 9, 10, 11];
    println!("{}", some_numbers
        .iter()
        .fold(0, |total_so_far, next_number| total_so_far + next_number)
    );
}
```

<br>

## `.fold()` events
```rust
#[derive(Debug)]
struct CombinedEvents {
    num_of_events: u32,
    data: Vec<String>,
}

fn main() {
    let events = [
    "Went to grocery store",
    "Came home",
    "Fed cat",
    "Fed cat again",
    ];

    let empty_events = CombinedEvents {
        num_of_events: 0,
        data: vec![]
    };
    let combined_events =
    events
        .iter()
        .fold(empty_events, |mut total_events, next_event| {
            total_events.num_of_events += 1;
            total_events.data.push(next_event.to_string());
            total_events
    });
    println!("{combined_events:#?}");
}
```
**Output**:
```python
CombinedEvents {
    num_of_events: 4,
    data: [
        "Went to grocery store",
        "Came home",
        "Fed cat",
        "Fed cat again",
    ],
}
```

<br>

## `.by_ref()`
```rust
fn main() {
    let mut number_iter = [7, 8, 9, 10].into_iter();
    let first = number_iter.by_ref().take(2).collect::<Vec<_>>();
    let second = number_iter.take(2).collect::<Vec<_>>();
}
```

<br>

## `.chunks()` and `.windows()`
```rust
fn main() {
    let num_vec = vec![1, 2, 3, 4, 5, 6, 7];
    println!("chunks:");
    for chunk in num_vec.chunks(3) {
        println!("  {:?}", chunk);
    }
    println!("windows:");
    for window in num_vec.windows(3) {
        println!("  {:?}", window);
    }
}
```
**Output**:
```python
chunks:
  [1, 2, 3]
  [4, 5, 6]
  [7]
windows:
  [1, 2, 3]
  [2, 3, 4]
  [3, 4, 5]
  [4, 5, 6]
  [5, 6, 7]
```

<br>

## `.match_indices()`
```rust
fn main() {
    let some_str = "Er ist noch nicht erklärt. Aber es gibt Krieg. Verlaß dich drauf.";
    for (index, item) in some_str.match_indices(|c| c > 'z') {
        println!("{item} at {index}");
    }
    for (index, item) in some_str.match_indices(". ") {
        println!("'{item}' at index {index}");
    }
}
```
**Output**:
```python
ä at 22
ß at 53
'. ' at index 26
'. ' at index 46
```

<br>

## `.peekable()` and `.peek()`
```rust
fn main() {
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();
    for i in 0..3 {
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        println!("Iteration number {}, current item is {}", i, number_iter.peek().unwrap());
        number_iter.next();
    }
}
```

<br>

## `flatten()`
The `.flatten()` can be applied to `Result`/`Option` in different ways:
- for **single nested** `Result`/`Option` value;
- for **iterator** of `Result`/`Option` values;
- for **collecting** a `Vec<Result<T, E>>` into a `Result<Vec<T, E>>`;

<br>

### Flattening a single nested `Result` or `Option`
To **flatten** a **single instance** of a nested `Result` or `Option`, you can use the `.flatten()` method.<br>

The `Option<Option<T>>` implements `.flatten()` itself:
```rust
impl<T> Option<Option<T>> {
    pub const fn flatten(self) -> Option<T> {
        match self {
            Some(inner) => inner,
            None => None,
        }
    }
}
```

The `.flatten()` method called on the `Option<Option<T>>` **converts** from `Option<Option<T>>` to `Option<T>`:
```rust
let x: Option<Option<u32>> = Some(Some(6));
assert_eq!(Some(6), x.flatten());

let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
assert_eq!(Some(Some(6)), x.flatten());
assert_eq!(Some(6), x.flatten().flatten());
```

<br>

The `Result<Result<T, E>, E>` implements `.flatten()` itself:
```rust
impl<T, E> Result<Result<T, E>, E> {
    pub const fn flatten(self) -> Result<T, E> {
        match self {
            Ok(inner) => inner,
            Err(e) => Err(e),
        }
    }
}
```

The `.flatten()` method called on the `Result<Result<T, E>, E>` **converts** from `Result<Result<T, E>, E>` to `Result<T, E>`:
```rust
let x: Result<Result<&'static str, u32>, u32> = Ok(Ok("hello"));
assert_eq!(Ok("hello"), x.flatten());
```

<br>

### Flattening an `Iterator` that yields `Result` or `Option`
The `Option` and `Result` both implement `IntoIterator`.<br>

```rust
impl<T> IntoIterator for Option<T>
```

```rust
impl<T, E> IntoIterator for Result<T, E>
```

The `Result<T, E>` and `Option` both can be treated as an `Iterator` over **one** or **zero** items.<br>

`Option` implements `IntoIterator`, which yields:
- **one** item (the `T` **value**) in the case of `Some(T)`;
- **zero** items in the case of `None`;

`Result<T, E>` implements `IntoIterator`, which yields:
- **one** item (the `T` **value**) in the case of `Ok(T)`;
- **zero** items in the case of `Err(E)`;

<br>

**Examples**:
```rust
let x = Some("string");
let v: Vec<&str> = x.into_iter().collect();
assert_eq!(v, ["string"]);

let x = None;
let v: Vec<&str> = x.into_iter().collect();
assert!(v.is_empty());
```

<br>

**Examples**:
```rust
let x: Result<u32, &str> = Ok(5);
let v: Vec<u32> = x.into_iter().collect();
assert_eq!(v, [5]);

let x: Result<u32, &str> = Err("nothing!");
let v: Vec<u32> = x.into_iter().collect();
assert_eq!(v, []);
```

<br>

**Consider example**:
```rust
fn main() {
    let v = ["9", "nine", "ninety-nine", "9.9"];
    for num in v.into_iter().map(|num| num.parse::<f32>()) {
        println!("{num:?}");
    }
}
```
**Output**:
```python
Ok(9.0)
Err(ParseFloatError { kind: Invalid })
Err(ParseFloatError { kind: Invalid })
Ok(9.9)
```
<br>

But if we **don’t care** about the `Err` values, we can add `.flatten()`:
```rust
fn main() {
    let v = ["9", "nine", "ninety-nine", "9.9"];
    for num in v.into_iter().map(|num| num.parse::<f32>()).flatten() {
        println!("{num:?}");
    }
}
```
**Output**:
```python
9.0
9.9
```

<br>

### Collecting a `Vec<Result<T, E>>` into a `Result<Vec<T>, E>`
A common use case is to process an **iterator** of **fallible** operations and collect all **successful** results into a `Vec<T>` or **stops** on the **first** `Err` and **returns it**.<br>

Example:
```rust
fn main() {
    let results: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let transposed: Result<Vec<i32>, &str> = results.into_iter().collect();
    println!("{:?}", transposed); // Output: Ok([1, 2, 3])

    let results_err: Vec<Result<i32, &str>> = vec![Ok(1), Err("error encountered"), Ok(3)];
    let transposed_err: Result<Vec<i32>, &str> = results_err.into_iter().collect();
    println!("{:?}", transposed_err); // Output: Err("error encountered")
}
```
**Output**:
```python
Ok([1, 2, 3])
Err("error encountered")
```

<br>

## `.inspect()`
```rust
fn main() {
    let a = [1, 4, 2, 3];
    let sum = a.iter()
        .cloned()
        .inspect(|x| println!("current item: {x}"))
        .filter(|x| x % 2 == 0)
        .inspect(|x| println!("item after filter: {x}"))
        .fold(0, |sum, i| sum + i);
    println!("{sum}");
}
```
**Output**:
```python
current item: 1
current item: 4
item after filter: 4
current item: 2
item after filter: 2
current item: 3
6
```

<br>

## The `dbg!` macro
The `dbg!` prints to **stderr**.<br>
The `dbg!(val)` **moves** `val` if `val` is **not** `Copy`.<br>

You can put `dbg!` in the middle of expression:
```rust
fn main() {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);
    let new_vec = dbg!(vec![8, 9, 10]);
    let double_vec = dbg!(new_vec.iter().map(|x| x *2).collect::<Vec<i32>>());
    dbg!(double_vec);
}
```
**Output**:
```python
[src/main.rs:2:25] 9 = 9
[src/main.rs:3:5] my_number += 10 = ()
[src/main.rs:4:19] vec![8, 9, 10] = [
    8,
    9,
    10,
]
[src/main.rs:5:22] new_vec.iter().map(|x| x *2).collect::<Vec<i32>>() = [
    16,
    18,
    20,
]
[src/main.rs:6:5] double_vec = [
    16,
    18,
    20,
]
```

<br>

### Suppress `dbg!` in release builds
The standard Rust `dbg!` macro is **not** automatically disabled in **release** builds.<br>

<br>

#### Attribute `debug_assertions`
```rust
fn main() {
    #[cfg(debug_assertions)]
    let v =  dbg!(10);
}
```

- the `debug_assertions` flag is *automatically* **enabled** for **debug profile**;
- the `debug_assertions` flag is *automatically* **disabled**  for **release profile**, i.e. when using `cargo build --release`;

<br>

#### Defining a custom macro
```rust
macro_rules! dbg {
    () => {
        #[cfg(debug_assertions)]
        std::dbg!();
    };
    ($val:expr) => {
        #[cfg(debug_assertions)]
        std::dbg!($val);
        #[cfg(not(debug_assertions))]
        {
            // In release, the expression is still evaluated but nothing is printed.
            // This is useful if the expression has side effects.
            ($val)
        }
    };
    // Add more cases for multiple arguments if needed.
    ($($val:expr),+) => {
        #[cfg(debug_assertions)]
        std::dbg!($($val),+);
        #[cfg(not(debug_assertions))]
        {
            // Evaluate all expressions in release mode to avoid dead code warnings
            // if they have side effects.
            ($(($val)),*)
        }
    };
}
```

<br>

# Chapter 10

<br>

# Chapter 11

<br>

# Chapter 12

<br>

# Chapter 13

<br>

# Chapter 14

<br>

# Chapter 15

<br>

# Chapter 16

<br>
