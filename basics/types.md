# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [Data types](#data-types)
* [Scalars](#scalars)
  * [Integer literals](#integer-literals)
  * [Integer overflow](#integer-overflow)
* [Constants](#constants)
  * [Examples](#examples)
* [Static](#static)
  * [Examples](#examples-1)
* [`const` vs. `static`](#const-vs-static)
* [Range operator](#range-operator)
* [Type casting](#type-casting)
* [Unit type `()`](#unit-type-)
* [DST](#dst)
* [Enums](#enums)
  * [Syntax](#syntax)
    * [*Type declaration* syntax](#type-declaration-syntax)
      * [Example](#example)
    * [*Initialization* syntax](#initialization-syntax)
  * [Access to `enum` variant](#access-to-enum-variant)
      * [Example](#example-1)
* [Primitive Type never](#primitive-type-never)
* [Newtype pattern](#newtype-pattern)
  * [Syntax](#syntax-1)
      * [Example](#example-2)
  * [Destructuring let](#destructuring-let)
      * [Example](#example-3)
* [Structs](#structs)
  * [Syntax](#syntax-2)
    * [*Type declaration* syntax](#type-declaration-syntax-1)
      * [Example](#example-4)
    * [*Initialization* syntax](#initialization-syntax-1)
      * [`Struct` constructor](#struct-constructor-)
        * [Example](#example-5)
      * [Method `new()`](#method-new)
  * [Range operator in structs](#range-operator-in-structs)
      * [Example](#example-6)
* [Tuple structs](#tuple-structs)
  * [Syntax](#syntax-3)
    * [*Type declaration* syntax](#type-declaration-syntax-2)
      * [Examples](#examples-2)
    * [*Initialization* syntax](#initialization-syntax-2)
      * [Examples](#examples-3)
* [Tuples](#tuples)
  * [*Initialization* syntax](#initialization-syntax-3)
    * [Syntax options for *pre initialized* tuples:](#syntax-options-for-pre-initialized-tuples)
  * [*Type declaration* syntax](#type-declaration-syntax-3)
  * [Access to fields of a tuple](#access-to-fields-of-a-tuple)
* [Unit-like structs](#unit-like-structs)
  * [Syntax](#syntax-4)
    * [*Type declaration* syntax](#type-declaration-syntax-4)
      * [Examples](#examples-4)
    * [*Initialization* syntax](#initialization-syntax-4)
      * [Examples](#examples-5)
* [Type aliases](#type-aliases)
<!-- TOC -->

<br>

# Data types
Rust has **2 categories** of types:
- **scalar types**: represent a single value;
- **compound types**: represent a collection of values: arrays/tuples/structs/enums/slices/...;

<br>

# Scalars
Rust has **4** scalar type groups:
- **integers**;
- **floats**;
- **booleans**;
- **chars**;

<br>

An **integer** is a number without a fractional component.<br>

<br>

Each **signed** integer of length $`N`$ bits has following allowed values: [-$`2^{N-1}`$, $`2^{N-1}-1`$].<br>
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

## Integer overflow
`rustc` flag `-C overflow-checks=yes|no` controls the behavior of **runtime integer overflow** ([RFC 560](https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md)):
- when this flag is **enabled** `overflow-checks=yes` a **panic** will occur on **overflow** (e.g., `255 + 1` causes to **panic**).<br>
- when this flag is **disabled** `overflow-checks=no` a **two’s complement** (aka **wrap around**) is used (e.g., `255 + 1` becomes `0` for an `u8` integer).<br>

<br>

Rust behaves differently in **debug mode** and **release mode** on **integer overflow**:
- in **debug mode** `overflow-checks=yes` by default;
- in **release mode** `overflow-checks=no` by default;

<br>

**Examples**:
```Rust
RUSTFLAGS="-C overflow-checks=yes|no" cargo run --release

RUSTFLAGS="-C overflow-checks=yes|no" cargo run
```

<br>

# Constants
Constant is **not** variable or place in memory, it is **compile time computation**.<br>

**Properties**:
- **Uppercase** by convention.
- Data type is **mandatory**.
- Values can not be changed.
- **Global** or **Local** scope.

<br>

## Examples
```Rust
const URL: &str = "google.com";
```

<br>

# Static
**Static variables** are **global variables** with following properties:
- *Static variable* **must** have **static lifetime**.
- *Static variables* can be **mutable** or **immutable**.
- *Static variables* have **fixed address** in the memory.
- **Mutable** *static variables* can only be **read** and **modified** inside `unsafe` **block**.

<br>

## Examples
```Rust
static mut COUNTER: u64 = 0;

unsafe fn increment() {
    COUNTER += 1;
}

fn main () {
    // access to modify static variable
    unsafe {
        increment();
    }

    // access to read static variable
    unsafe {
        println!("Counter is {}.", COUNTER);
    }
}
```

<br>

# `const` vs. `static`
`const`:
- **has no fixed address in memory**;
- value is **inlined** to each place where it is used;
- **faster** runtime;
- **bigger** executable file;

<br>

`static`:
- **has fixed address in memory**;
- value is **loaded from memory**;
- **slower** runtime because we need load data from memory;
- **smaller** executable file;

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

# Type casting
```Rust
let v = true;
let flag = v as i32;
```

<br>

# Unit type `()`
The **unit type** or just **Unit** is denoted as `()`. The **unit type** has only **one value**, which is also `()`.<br>
It is **ZST** (**zero-sized type**).<br>

<br>

# DST
**DST** or **D**ynamically **S**ized **T**ypes (aka **unsized**). [**More here**](../traits/utility-traits/Sized.md).

<br>

# Enums
An `enum` in Rust is **tagged union** or **sum type**.<br>
`enum` consists of different `variants`.<br>
Each `variant` in the `enum` reperesents **some** `type`.<br>
A value of an `enum` type matches to **one specific** `variant`.

<br>

## Syntax
### *Type declaration* syntax
```Rust
enum <Name> {
    Variant_1,
    Variant_2(SomeType_2),
    Variant_3(SomeType_3),
}
```
where `Variant_i` wraps type `SomeType_i` or without any type like `Variant_1`.

#### Example
```Rust
enum MyEnum {
    Bar,
    Foo(i32, i32, i32),
    Baz { x: i32, y: i32 },
    FooBar(String),
}
```

<br>

### *Initialization* syntax
Variable of `enum` type can **only** be initialized with **specific** value of type `SomeType_i`:
```Rust
let x: SomeType_i = SomeType_i::new(...);
let v: <Name> = <Name>::Variant_i(x);
```

<br>

## Access to `enum` variant
To access to **specific** `variant` of variable of `enum` type **pattern matching** is used.

#### Example
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

# Primitive Type never
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

## Destructuring let
To **extract** the **inner value** `destructuring let` is used.

#### Example
```Rust
struct Foo(i32);

fn main() {
    let f = Foo(10);
    let Foo(v) = f;  // destructuring let
    println!("Value of 'v': {}.", v);
}

Output:
Value of 'v': 10.
```

<br>

# Structs
`Struct` type is container for values of different types.

<br>

## Syntax
### *Type declaration* syntax
```Rust
struct <Name> {
    f1: T1,
    f2: T2,
    ...
}
```
where: `fi` - name of the `field` in the `struct` of type `Ti`.

#### Example
```Rust
struct Foo {
    bar: i32,
    baz: i32,
}
```

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
