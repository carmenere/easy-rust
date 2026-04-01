# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Data types](#data-types)
- [Scalars](#scalars)
  - [Integers](#integers)
  - [Integer literals](#integer-literals)
  - [Arithmetic operations and traits](#arithmetic-operations-and-traits)
  - [Casting integers](#casting-integers)
  - [Integer overflow](#integer-overflow)
  - [Floats](#floats)
  - [Boolean](#boolean)
  - [Copy types](#copy-types)
- [Type inference](#type-inference)
- [Unit type `()`](#unit-type-)
- [DST](#dst)
- [Structs vs. Enums](#structs-vs-enums)
- [Enums](#enums)
  - [Syntax](#syntax)
  - [Access to `enum` variant](#access-to-enum-variant)
    - [Discriminants](#discriminants)
    - [Casting enums into integers](#casting-enums-into-integers)
    - [Importing enum variants](#importing-enum-variants)
    - [Holding different types in collection](#holding-different-types-in-collection)
- [Structs](#structs)
  - [Syntax](#syntax-1)
    - [*Type declaration* syntax](#type-declaration-syntax)
    - [*Initialization* syntax](#initialization-syntax)
      - [`Struct` constructor](#struct-constructor)
        - [Example](#example)
      - [Method `new()`](#method-new)
  - [Range operator in structs](#range-operator-in-structs)
      - [Example](#example-1)
- [Tuple structs](#tuple-structs)
  - [Syntax](#syntax-2)
    - [*Type declaration* syntax](#type-declaration-syntax-1)
      - [Examples](#examples)
    - [*Initialization* syntax](#initialization-syntax-1)
      - [Examples](#examples-1)
- [Implementing structs and enums](#implementing-structs-and-enums)
- [Tuples](#tuples)
  - [*Initialization* syntax](#initialization-syntax-2)
    - [Syntax options for *pre initialized* tuples:](#syntax-options-for-pre-initialized-tuples)
  - [*Type declaration* syntax](#type-declaration-syntax-2)
  - [Example of tuple definition](#example-of-tuple-definition)
  - [Access to fields of a tuple](#access-to-fields-of-a-tuple)
- [Unit-like structs](#unit-like-structs)
  - [Syntax](#syntax-3)
    - [*Type declaration* syntax](#type-declaration-syntax-3)
      - [Examples](#examples-2)
    - [*Initialization* syntax](#initialization-syntax-3)
      - [Examples](#examples-3)
- [Type aliases](#type-aliases)
- [Newtype pattern](#newtype-pattern)
  - [Syntax](#syntax-4)
      - [Example](#example-2)
- [Infallible](#infallible)
  - [`try_`](#try_)
  - [Blanket impl](#blanket-impl)
  - [never type `!`](#never-type-)
<!-- TOC -->

<br>

# Data types
Rust has **2 categories** of types:
- **scalar types**: represent a **single value**;
- **compound types**: represent a **many values** *as a single whole*: arrays/tuples/structs/enums/slices/...;

<br>

The **simplest types** in Rust are called **primitive types** (primitive = very basic).<br>

<br>

# Scalars
Rust has **4** scalar type groups:
- **integers**;
- **floats**;
- **booleans**;
- **chars**;

<br>

## Integers
An **integer** is a number without a fractional component. There are two types of integers: **signed integers** and **unsigned integers**.<br>
Rust **by default** infers `i32` for *integers* if you don’t tell explicit type.<br>
A `usize` is the best size for **indexing** because.<br>

<br>

Each **signed** integer of length $`N`$ bits has following allowed values: [$`-2^{N-1}`$, $`2^{N-1}-1`$].<br>
Each **unsigned** integer of length $`N`$ bits has following allowed values: [$`0`$, $`2^{N}-1`$].<br>

<br>

| Scalar type | Length in bits           | Allowed values                    |
|:------------|:-------------------------|:----------------------------------|
| `u8`        | **8** bits               | [$`0`$, $`+255`$]                 |
| `i8`        | **8** bits               | [$`-128`$, $`+127`$]              |
| `u16`       | **16** bits              | [$`0`$, $`2^{16}-1`$]             |
| `i16`       | **16** bits              | [$`-2^{16-1}`$, $`2^{16-1}-1`$]   |
| `u32`       | **32** bits              | [$`0`$, $`2^{32}-1`$]             |
| `i32`       | **32** bits              | [$`-2^{32-1}`$, $`2^{32-1}-1`$]   |
| `u64`       | **64** bits              | [$`0`$, $`2^{64}-1`$]             |
| `i64`       | **64** bits              | [$`-2^{64-1}`$, $`2^{64-1}-1`$]   |
| `u128`      | **128** bits             | [$`0`$, $`2^{128}-1`$]            |
| `i128`      | **128** bits             | [$`-2^{128-1}`$, $`2^{128-1}-1`$] |
| `usize`     | depends on **arch**      |                                   |
| `isize`     | depends on **arch**      |                                   |
| `f32`       | **32** bits              |                                   |
| `f64`       | **64** bits              |                                   |
| `boolean`   | **8** bits / **1** byte  | `true`, `false`                   |
| `char`      | **32** bits/ **4** bytes | Any single Unicode character      |

<br>

When the **type cannot be inferred** from the context rust **by default** assigns:
- `i32` for **integers**: `let x = 5 // x is of i32`;
- `f64` for **floats**: `let x = 5.5 // x is of f64`;

<br>

## Integer literals
There are several **formats** for **integer literals**:
- **Decimal**: `55_55`;
- **Hexadecimal**: `0xaabb`;
- **Octal**: `0o20`;
- **Binary**: `0b1010_1000`;
- **Bytes**: `b'AB'`;

<br>

The `_` symbol is called **number separator** and is used in integer **literals**:
```rust
let a = 1_000_000;
```

<br>

It is possible to **add type at the of literal**:
```rust
let b = 1u64;
let c = 1_u64;
```

<br>

## Arithmetic operations and traits
There are **8 traits** for **arithmetic operations**:
- operator `+`, corresponding trait `Add`;
- operator `-`, corresponding trait `Sub`;
- operator `/`, corresponding trait `Div`;
- operator `*`, corresponding trait `Mul`;
- operator `+=`, corresponding trait `AddAssign`;
- operator `-=`, corresponding trait `SubAssign`;
- operator `/=`, corresponding trait `DivAssign`;
- operator `*=`, corresponding trait `MulAssign`;

More traits here [**std::ops**](https://doc.rust-lang.org/std/ops/index.html).<br>

<br>

## Casting integers
```Rust
let v = true;
let flag = v as i32;
```

<br>

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

## Integer overflow
`rustc` provides flag `-C overflow-checks=yes|no` that controls the behavior of **runtime integer overflow** ([RFC 560](https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md)):
- when this flag is **enabled** `overflow-checks=yes` a **panic** will occur on **overflow** (e.g., `255 + 1` causes to **panic**);
- when this flag is **disabled** `overflow-checks=no` a **two’s complement** (aka **wrap around** or **wraparound arithmetic**) is used (e.g., `255_u8 + 1` becomes `0`);

<br>

The compiler **won't compile** if it **knows at compile** time that a **number will overflow**.<br>
But if a **number isn't known at compile time**, the behaviour will be different:
- in **debug mode** `overflow-checks=yes` by default and the program **will panic**;
- in **release mode** `overflow-checks=no` by default and the program **will overflow**;

<br>

**Examples**:
```Rust
RUSTFLAGS="-C overflow-checks=yes|no" cargo run --release
RUSTFLAGS="-C overflow-checks=yes|no" cargo run
```

<br>

The **std** provides **4** sets of methods for explicit handling of overflow:
- **wrapping_**:
  - `x.wrapping_abs()`
  - `x.wrapping_add(y)`
  - `x.wrapping_add_unsigned(y)`
  - `x.wrapping_div(y)`
  - `x.wrapping_div_euclid(y)`
  - `x.wrapping_mul(y)`
  - `x.wrapping_neg()`
  - `x.wrapping_pow(y)`
  - `x.wrapping_rem(y)`
  - `x.wrapping_rem_euclid(y)`
  - `x.wrapping_shl(y)`
  - `x.wrapping_shr(y)`
  - `x.wrapping_sub(y)`
  - `x.wrapping_sub_unsigned(y)`
- **saturating_**:
  - `x.saturating_abs(y)`
  - `x.saturating_add(y)`
  - `x.saturating_add_unsigned(y)`
  - `x.saturating_div(y)`
  - `x.saturating_mul(y)`
  - `x.saturating_neg(y)`
  - `x.saturating_pow(y)`
  - `x.saturating_sub(y)`
  - `x.saturating_sub_unsigned(y)`
- **overflowing_**: returns a tuple `(T, bool)` that consists of **two's complement result** and a **boolean** indicating if an overflow occurred:
  - `x.overflowing_abs()`
  - `x.overflowing_add(y)`
  - `x.overflowing_add_unsigned(y)`
  - `x.overflowing_div(y)`
  - `x.overflowing_div_euclid(y)`
  - `x.overflowing_mul(y)`
  - `x.overflowing_neg()`
  - `x.overflowing_pow(y)`
  - `x.overflowing_rem(y)`
  - `x.overflowing_rem_euclid(y)`
  - `x.overflowing_shl(y)`
  - `x.overflowing_shr(y)`
  - `x.overflowing_sub(y)`
  - `x.overflowing_sub_unsigned(y)`
- **checked_**: return `Option<T>` that is `None` when overflow occurred:
  - `x.checked_abs()`
  - `x.checked_add(y)`
  - `x.checked_add_unsigned(y)`
  - `x.checked_div(y)`
  - `x.checked_div_euclid(y)`
  - `x.checked_ilog(y)`
  - `x.checked_ilog10(y)`
  - `x.checked_ilog2(y)`
  - `x.checked_isqrt(y)`
  - `x.checked_mul(y)`
  - `x.checked_neg()`
  - `x.checked_next_multiple_of(y)`
  - `x.checked_pow(y)`
  - `x.checked_rem(y)`
  - `x.checked_rem_euclid(y)`
  - `x.checked_shl(y)`
  - `x.checked_shr(y)`
  - `x.checked_sub(y)`
  - `x.checked_sub_unsigned(y)`

<br>

**Example**:
  - `5u8.wrapping_add(10)`
  - `u8::MAX.wrapping_add(2)`
  - `i8::MAX.wrapping_add(2)`
  - `5u8.wrapping_sub(10)`
  - `u8::MAX.wrapping_sub(2)`
  - `i8::MAX.wrapping_sub(2)`

<br>

**Example**:
```rust
fn main() {
    println!("u8::MIN={}, u8::MAX={}", u8::MIN, u8::MAX);
    println!("u8::MIN.wrapping_add(1)={}, u8::MAX.wrapping_add(1)={}", u8::MIN.wrapping_add(1), u8::MAX.wrapping_add(1));
    println!("i8::MIN={}, i8::MAX={}", i8::MIN, i8::MAX);
    println!("i8::MIN.wrapping_add(1)={}, i8::MAX.wrapping_add(1)={}", i8::MIN.wrapping_add(1), i8::MAX.wrapping_add(1));
}
```

<br>

## Floats
**Float** is a numbers with **decimal point**, i.e. `5.5` is a float, `5.0` is also a float, and even `5.` is a float.<br>
Rust **by default** infers `f64` for *floats* if you don’t tell explicit type.<br>

<br>

The basic methods for floats:
- `.floor()` returns the **next lowest** integer;
- `.ceil()` returns the **next highest** integer;
- `.round()`
  - like `.ceil()` if **>= 0.5** (**greater than or equal to 0.5**);
  - like `.floor()` if **< 0.5** (**less than 0.5**);
- `.trunc()` like `.floor()`;

<br>

## Boolean
In Rust, you can turn a `bool` into an `integer` if you want because **it’s safe** to do that. But you **can’t** do it the other way around:
```rust
fn main() {
  let true_false1 = (true as u8, false as i32);
  let true_false2: (u8, i32) = (true.into(), false.into());
  println!("{} {}", true_false1.0 , true_false1.1);
  println!("{} {}", true_false2.0, true_false2.1);
}
```

<br>

There are two methods: `.then()` and `.then_some()`, that turn a `bool` into an `Option`:
- `b.then_some(t: T) -> Option<T>`
  - returns `Some(t)` if the bool `b` is `true`, or `None` otherwise;
- `b.then(f: F) -> Option<T>`
  - a closure `f -> T` is called if the bool `b` is `true`;
  - whatever is returned from the closure gets wrapped in an `Option`;

<br>

Example: `b.then(|| {}).ok_or_else(|| {})`, first `then()` converts `b` **into** `Option` and then `ok_or_else()` converts `Option` to `Result`.<br>

<br>

## Copy types
Rust’s simplest types are known as `Copy` types. They are all on the stack, and the **compiler knows their size**. That means that they are **very easy (cheap) to copy**, so the compiler
always copies their data when you send these types to a function. `Copy` types are **so small** and **easy** that there's no reason not to.<br>

`Copy` types are so **cheap** that you don’t need to worry about ownership.<br>

You also see the word **trivial** to talk about `Copy` types a lot, such as “It’s **trivial to copy them**.” That means: it’s so easy to copy them that there is no reason not to copy them.<br>
`Copy` types include `integers`, `floats`, `booleans` (true and false), `char`, and others.<br>

If it is a `Copy` type, the data would be **copied**, **not moved**.<br>
`Clone` is similar to `Copy` but usually needs more memory.<br>

<br>

# Type inference
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



# Unit type `()`
The **unit type** or just **Unit** is denoted as `()`. The **unit type** has only **one value**, which is also `()`.<br>
It is **ZST** (**zero-sized type**).<br>

<br>

# DST
**DST** or **D**ynamically **S**ized **T**ypes (aka **unsized**). [**More here**](../traits/utility-traits/Sized.md).

<br>

# Structs vs. Enums
Structs vs. Enums:
- use `struct` when you want *one thing* **and** *another thing*, i.e. if you have a *lot of things to group together*;
- use `enum` when you want *one thing* **or** *another thing*, i.e. if you have a *lot of choices and need to select one*;

The name of a `struct` and `enum` should be in **UpperCamelCase** (capital letter for each word with no spaces).<br>

<br>

# Enums
An `enum` in Rust is **tagged union** or **sum type**.<br>
`enum` consists of different `variants`.<br>
Each `variant` in the `enum` reperesents **some** `type`.<br>
A value of an `enum` type matches to **one specific** `variant`.<br>

<br>

## Syntax
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

Variable of `enum` type can **only** be initialized with **specific** value of type `SomeType_i`:
```Rust
let v: <Name> = <Name>::Variant_i(x);
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

## Access to `enum` variant
To access to **specific** `variant` of variable of `enum` type **pattern matching** is used.

**Example**:
```Rust
#[derive(Debug)]
struct MyStruct { x: i32, y: i32 }

#[derive(Debug)]
enum MyEnum {
    Bar,
    Foo(i32, i32, i32),
    Baz (MyStruct),
    FooBar(String),
}

fn main() {
    let s = MyStruct { x: 3, y: 4 };

    let v = MyEnum::Baz(MyStruct { x: 3, y: 4 });

    match &v {
        MyEnum::Bar => println!(""),
        MyEnum::Foo(x, y, z) => println!("MyEnum::Foo"),
        MyEnum::Baz(MyStruct) => println!("MyEnum::Baz"),
        MyEnum::FooBar(val) => println!("MyEnum::FooBar"),
    }

    println!("{:X?}", v);
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

# Structs
`Struct` type is container for values of different types.

<br>

## Syntax
### *Type declaration* syntax
There are **three types** of structs:
- **unit struct**:
  - **unit** means “doesn’t have anything” (like the **unit type**);
  - you **must write a semicolon** after a *unit struct*
- **tuple struct**:
  - in *tuple struct* you **only** declare **types** inside a `()`;
  - *tuple struct* **doesn't** containt fields;
  - you **must write a semicolon** after a *tuple struct*
- **named struct**:
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

### *Initialization* syntax
#### `Struct` constructor 
```Rust
let v: <Name> = <Name> {
    f1: val_1,
    f2: val_2,
    ...
}
```

##### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}

fn main() {
    let v = Foo { bar: 1, baz: 2 };
    println!("Fields of 'v': 'bar' = {}, 'baz' = {}.", v.bar, v.baz);
}
```

<br>

#### Method `new()`
```Rust
let v: <Name> = <Name>::new(a=val_1, b=val_2, ... );
```

<br>

## Range operator in structs
A `struct` **constructor** can include `..` operator to **copy** some values from another variable of **the same** `struct` type. 

#### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}

fn main() {
    let v1 = Foo { bar: 1, baz: 2 };
    let v2 = Foo { bar: 77, .. v1};
    println!("Fields of 'v1': 'bar' = {}, 'baz' = {}.", v1.bar, v1.baz);
    println!("Fields of 'v2': 'bar' = {}, 'baz' = {}.", v2.bar, v2.baz);
}

Output:
Fields of 'v1': 'bar' = 1, 'baz' = 2.
Fields of 'v2': 'bar' = 77, 'baz' = 2.
```

<br>

# Tuple structs
`tuple struct` is a hybrid between a `tuple` and a `struct`.<br>
Unlike the `struct` type, the fields in the `tuple struct` don't have names.

<br>

## Syntax
### *Type declaration* syntax
```Rust
struct <Name> (T1, ... );
```
where `Ti` is of some type.

#### Examples
```Rust
struct Foo(i32, i32, i32);
struct Bar(i32, i32, i32);
```

<br>

### *Initialization* syntax
```Rust
let v = <Name>(val_1, ... );
```
where `val_i` - value of some type `Ti`.

#### Examples
```Rust
let f = Foo(0, 0, 0);
let b = Bar(0, 0, 0);
```

<br>

# Implementing structs and enums
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

# Tuples
A `tuple` is an **ordered** and **fixed size** *collection* of elements of *different types*. 

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* tuples:
**Comma separated** list of **values** *enclosed* in **parentheses**, e.g., `(val1, val2, val3)`.
- **2-length tuple**
```Rust
let tup = (1, "foo");
```
- **5-length tuple**
```Rust
let tup = (1, "foo", 2, "bar", 3);
```

<br>

## *Type declaration* syntax
- **Comma separated** list of **type declarations** *enclosed* in **parentheses**, e.g., `(i32, &str)`.
```Rust
let x: (i32, &str) = (1, "hello");
```
 
<br>

## Example of tuple definition
```rust
fn main() {
    let random_tuple: (&'static str, i32, Vec<char>, char, [i32; 3], f64) = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
}
```

- `("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7)` is a **tuple**;
- the **tupe of tuple** `(&'static str, i32, Vec<char>, char, [i32; 3], f64)` depends on the types of the items inside it;

<br>

## Access to fields of a tuple
There is **dot notation** `.<index_of_field>` to access to field in a tuple with index of `<index_of_field>`.<br>

Example:
```Rust
let tuple = (1, 2, 3);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;

println!("x is {}", x);
```

<br>

# Unit-like structs
`unit-like struct` type is a `struct` with no members.<br>
`unit-like struct` type is a **zero-sized type** (**ZST**).

## Syntax
### *Type declaration* syntax
```Rust
struct <Name>;
```

#### Examples
```Rust
struct Foo;
struct Bar;
```

<br>

### *Initialization* syntax
```Rust
let v = <Name>;
```

#### Examples
```Rust
let f = Foo;
let b = Bar;
```

<br>

# Type aliases
```Rust
fn main () {
    type meters = u64;
    let a: meters = 10;
    let b: u64 = 20;
    assert_eq!(a+b, 30u64);
}
```

In the above example, `meters` is **not** a new type, it is **alias** (**synonym**) for `u64`, so `meters` is treated as `u64`.

<br>

Another example:
```Rust
type MySend = Box<dyn Fn() + Send + 'static>;
```

In the above example type alias `MySend` can be used instead `Box<dyn Fn() + Send + 'static>`.

<br>

If you have **duplicate names** or you **have some reason to change a type name**, you can use `as` to rename type **during import**:
```rust
fn main() {
    use String as W;
    let my_string = W::from("Hi!");
}
```

<br>

Also there is keyword `type` to set alias for type:
```rust
use std::iter::{Take, Skip};
use std::vec::IntoIter;

type SkipTake = Take<Skip<IntoIter<char>>>;

fn skip_4_take_5_chars(input: Vec<char>) -> SkipTake {
    input.into_iter().skip(4).take(5)
}

fn main() {
    use String as W;
    let r = skip_4_take_5_chars("abcdef".to_string().chars().collect());
}
```

<br>

**A type alias doesn’t create a new type**. It’s just a name to use instead of an existing type. So if you write type `File = String;`, the compiler just sees a `String` whenever `File` is used.<br>

<br>

# Newtype pattern
The `newtype pattern` allows to create a **new type** that is **distinct** from its contained value and also has its own semantic.

<br>

## Syntax
```Rust
struct <MyNewTypeName>(T);
```
where `T` is of some type.

#### Example
```Rust
struct Foo(i32);

fn main() {
    let f = Foo(10);
    println!("Value of f: {}.", f.0);
}
```

<br>

# Infallible
The `std::convert::Infallible`:
```rust
pub enum Infallible {}
```

<br>

The type `Infallible` is for **for errors** that **can never happen**. Since `Infallible` is a **enum** that **has no variant**, a value of this type can **never** actually exist, i.e. `Infallible` can be useful in `Result` as **error type** to indicate that the result is **always** `Ok`.<br>

The `Infallible` **enum** has the **same role as** the `!` **never type**, which is still *unstable*.<br>

When `!` *is stabilized*, the `Infallible` *will become* **type alias** to `!` and eventually **deprecate** `Infallible`:
```rust
pub type Infallible = !;
```

<br>

For example, **blanket implementation** `TryFrom for T` uses `Infallible`:
```rust
impl<T, U> TryFrom<U> for T
where
    U: Into<T>
{
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Infallible> {
        Ok(U::into(value))  // Never returns `Err`
    }
}
```

<br>

## `try_`
In Rust, the `try_` prefix is a widely used naming convention in the standard library and community for functions that are **fallible** (**can fail**) and return a `Result` type.<br>
The primary use case of the `try_` prefix is to provide a version of a **function that returns** a `Result<T, E>` (or sometimes `Option<T>`) **instead of panicking on failure**:
```rust
// This function will panic if the id does not exist.
fn get_config(id: u32) -> Config { ... }

// This will not panic and will return a Result instead.
fn try_get_config(id: u32) -> Result<Config, ConfigError> { ... }
```

<br>

**Examples**:
- `Box::try_new()`: a **fallible constructor** for `Box` that returns a `Result<Box<T>, AllocError>` **instead of panicking** on memory allocation failure;
- `Mutex::try_lock()`: tries to lock a mutex, **returning** a `Result` **if it fails** immediately, as opposed to `lock()`, which **blocks** the current thread until it can acquire the lock;

<br>

## Blanket impl
```rust
impl<T, U> TryFrom<U> for T
where
    U: Into<T>,
{
    type Error = Infallible;

    fn try_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::into(value))
    }
}
```

<br>

## never type `!`
The `!` type is called **never type**.<br>

<br>

```Rust
const FOO: bool = true;

fn main () {
    let bar = None;
    while FOO {
        let v = match bar {
            Some(v) => v,
            None => continue,
        };
    };
}
```

In the above example, variable `v` inside `while {}` is of **never type**, because `None => continue` arm of `match` will never return any value.

<br>

```Rust
const FOO: bool = true;

fn main () {
    let bar = Some(10);
    while FOO {
        let v = match bar {
            Some(v) => v,
            None => continue,
        };
    };
}
```

In the above example, variable `v` inside `while {}` is of `i32` type, because `Some(v) => v` arm of `match` returns value `10`.

<br>

So,
- `panic!` macro returns **never type**;
- `loop` returns **never type**, e.g., `let r: ! = loop {};`;
- keyword `continue` inside loop returns **never type**;

<br>