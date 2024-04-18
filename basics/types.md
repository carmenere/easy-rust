# Table of contents
- [Table of contents](#table-of-contents)
- [Type casting](#type-casting)
- [Arrays](#arrays)
  - [*Initialization* syntax](#initialization-syntax)
    - [Syntax options for *pre initialized* arrays:](#syntax-options-for-pre-initialized-arrays)
    - [Syntax options for *empty* arrays:](#syntax-options-for-empty-arrays)
  - [*Type declaration* syntax](#type-declaration-syntax)
- [Constants](#constants)
  - [Notes](#notes)
  - [Examples](#examples)
- [Static](#static)
  - [Examples](#examples-1)
- [DST (Dynamically Sized Types)](#dst-dynamically-sized-types)
- [Enums](#enums)
  - [Syntax](#syntax)
    - [*Type declaration* syntax](#type-declaration-syntax-1)
      - [Example](#example)
    - [*Initialization* syntax](#initialization-syntax-1)
  - [Access to `enum` variant](#access-to-enum-variant)
      - [Example](#example-1)
- [Primitive Type never](#primitive-type-never)
- [Newtype pattern](#newtype-pattern)
  - [Syntax](#syntax-1)
      - [Example](#example-2)
  - [Destructuring let](#destructuring-let)
      - [Example](#example-3)
- [Scalars](#scalars)
- [Number separator](#number-separator)
  - [Example](#example-4)
- [Overflow-checks](#overflow-checks)
  - [Examples](#examples-2)
- [Strings](#strings)
  - [`&str`](#str)
    - [Examples](#examples-3)
  - [`String`](#string)
    - [Examples](#examples-4)
  - [Methods](#methods)
  - [Bytes. Chars. Vec](#bytes-chars-vec)
  - [Conversions between string types](#conversions-between-string-types)
- [Unit type `()`](#unit-type-)
- [Structs](#structs)
  - [Syntax](#syntax-2)
    - [*Type declaration* syntax](#type-declaration-syntax-2)
      - [Example](#example-5)
    - [*Initialization* syntax](#initialization-syntax-2)
      - [`Struct` constructor](#struct-constructor)
        - [Example](#example-6)
      - [Method `new()`](#method-new)
  - [`..` operator](#-operator)
      - [Example](#example-7)
- [Tuple structs](#tuple-structs)
  - [Syntax](#syntax-3)
    - [*Type declaration* syntax](#type-declaration-syntax-3)
      - [Examples](#examples-5)
    - [*Initialization* syntax](#initialization-syntax-3)
      - [Examples](#examples-6)
- [Tuples](#tuples)
  - [*Initialization* syntax](#initialization-syntax-4)
    - [Syntax options for *pre initialized* tuples:](#syntax-options-for-pre-initialized-tuples)
  - [*Type declaration* syntax](#type-declaration-syntax-4)
  - [Access to fields of a tuple](#access-to-fields-of-a-tuple)
- [Unit-like structs](#unit-like-structs)
  - [Syntax](#syntax-4)
    - [*Type declaration* syntax](#type-declaration-syntax-5)
      - [Examples](#examples-7)
    - [*Initialization* syntax](#initialization-syntax-5)
      - [Examples](#examples-8)
- [Type aliases](#type-aliases)
- [Vectors](#vectors)
  - [*Initialization* syntax](#initialization-syntax-6)
    - [Syntax options for *pre initialized* vectors:](#syntax-options-for-pre-initialized-vectors)
    - [Syntax options for *empty* vectors:](#syntax-options-for-empty-vectors)
  - [*Type declaration* syntax](#type-declaration-syntax-6)

<br>

# Type casting
```Rust
let v = true;
let flag = v as i32;
```

<br>

# Arrays
An **array** is **fixed-size** *collection* of elements of **the same type**.<br>
Arrays are **allocated** on the **stack**.

<br>

## *Initialization* syntax
### Syntax options for *pre initialized* arrays:
- **Comma-delimited**: explicit enumeration of values within square brackets \[\]:
```Rust
let arr = [0, 1, 2];
```

- **Repeat expression**: \[`V`; `N`\], where the **value** `V` is **repeated** `N`times:
```Rust
let arr = [100; 5];
```

### Syntax options for *empty* arrays:
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

<br>

# Constants
## Notes
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
**Static variables** are **global variables**.<br>
*Static variable* **must** have **static lifetime**.<br>
*Static variables* can be **mutable** or **immutable**.<br>
**Mutable** *static variables* can only be **read** and **modified** inside `unsafe` **block**.<br>
*Static variables* have **fixed address** in the memory.

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

# DST (Dynamically Sized Types)
[More here](https://github.com/carmenere/easy-rust/blob/main/traits/utility-traits/Sized.md)

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

# Scalars
|Type group|Types|
|:---------|:----|
|**Integer**|**Signed**: `u8`, `u16`, `u32`, `u64`, `u128`<br>**Unsigned**: `i8`, `i16`, `i32`, `i64`, `i128`.<br>**Sizes of pointers**: `isize`, `usize` and they depend on **arch**.|
|**Float**|`f32`, `f64`|
|**Boolean**|`false`<br>`true`|
|**Character**|**One letter** in **single quotes**.<br>Example: `let ch = 'A';`|

<br>

# Number separator
The `_` symbol is called **number separator** and is used in **literals**.

## Example
```Rust
let a = 1_000_000;
let b = 1u64;
let c = 1_u64;
```

<br>

# Overflow-checks
`rustc` flag `-C overflow-checks=yes|no` controls the behavior of **runtime integer overflow** ([RFC 560](https://github.com/rust-lang/rfcs/blob/master/text/0560-integer-overflow.md)):
- when this flag is **enabled** `overflow-checks=yes` a **panic** will occur on **overflow** (e.g., `255 + 1` causes to **panic**).<br>
- when this flag is **disabled** `overflow-checks=no` a **two’s complement** is used (e.g., `255 + 1` becomes `0` for an `u8` integer).<br>

<br>

Rust behaves differently in **debug mode** and **release mode** on **integer overflow**:
- in **debug mode** `overflow-checks=yes` by default;
- in **release mode** `overflow-checks=no` by default;

<br>

## Examples
```Rust
RUSTFLAGS="-C overflow-checks=yes|no" cargo run --release

RUSTFLAGS="-C overflow-checks=yes|no" cargo run
```

<br>

# Strings
A **string** is a sequence of `Unicode` scalar values encoded as a stream of `UTF-8` bytes.<br>

Rust has two main types of strings: `&str` and `String`.

<br>

## `&str`
The `&str` type is called **string literal** or **string slice**.<br>
*Strings* of `&str` type are **statically allocated**, i.e., they are hardcoded into binary and exists while programme is running.<br>
*Strings* of `&str` type have a **fixed-size** and **cannot be mutated**, i.e., they are **immutable**.<br>

<br>

### Examples
```Rust
let s: &str = "ABC";
```

<br>

## `String`
The `String` is a sequence that is allowed to **grow** or **shrink** *in size* **at runtime** and is provided by Rust's standard library.

### Examples
- Instantiating `String` variables by `String` **constructor** (`new()`):
```Rust
let s: String = String::new();
```
- Instantiating `String` variables from `&str` values:
```Rust
let s1: String = String::from("ABC");
```
```Rust
let s2: String = "ABC".to_string();
```

<br>

## Methods
|Method|Description|
|:-----|:----------|
|`.len()`|Returns **length** of string.|
|`.push('c')`|**Append** *one character* to string.|
|`.push_str("abc")`|**Append** *substring* to string.|
|`.replace(from, to)`|**Replace** *substring* `from` to substring `to`.|
|`.split(sep)`|Splits string by separator|

<br>

## Bytes. Chars. Vec<u8>
```rust
#![allow(unused_variables)]

fn main() {
    // String
    let s = String::from("你好");
    println!("Len of s: {}", s.len());
    let s = String::from_utf8("你好".as_bytes().to_vec()).unwrap();
    println!("Len of s: {}", s.len());

    // Vec<u8>
    let v: Vec<u8> = s.as_bytes().to_owned();
    println!("Len of Vec<u8>: {}", v.len());
    println!("Vec<u8>:");
    for item in v {
        println!("  {}", item);
    }

    // std::str::Bytes<'_>
    let bytes: std::str::Bytes<'_> = s.bytes();
    println!("Len of bytes: {}", bytes.len());
    println!("Bytes:");
    for b in bytes {
        println!("  {}", b);
    }

    // std::str::Chars<'_>
    let chars: std::str::Chars<'_> = s.chars();
    println!("Chars:");
    for ch in chars {
        println!("  {}", ch);
    }
}
```

<br>

## Conversions between string types
Types `OsStr` and `OsString` must be imported explicitly:
```rust
#![allow(unused_variables)]

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStringExt, OsStrExt};

fn main() {
    let s: String = String::from("abc");
    let s1: String = String::from("abc");
    let s2: String = String::from("abc");
    let st: &str = "abc";
    let u: &[u8] = "abc".as_bytes();
    let b: &[u8; 6] = b"foobar";
    let bb: [u8; 6] = b"foobar".to_owned();
    let v: Vec<u8> = String::from("abc").into_bytes();
    let v1: Vec<u8> = String::from("abc").into_bytes();
    let v2: Vec<u8> = String::from("abc").into_bytes();
    let ost: &OsStr = OsStr::new("abc");
    let os: OsString = OsString::from("abc");

    //////////////////////////////////////////////////
    
    // &str -> String
    let r: String = String::from(st);
    let r: String = st.to_string();
    let r: String = st.to_owned();

    // &str -> &[u8]
    let r: &[u8] = st.as_bytes();
    
    // &str -> Vec<u8>
    let r: Vec<u8> = st.as_bytes().to_owned();
    
    // &str -> &OsStr
    let r: &OsStr = OsStr::new(st);

    //////////////////////////////////////////////////
    
    // String -> &str
    let r: &str = s.as_str();

    // String -> &[u8]
    let r: &[u8] = s.as_bytes();

    // String -> Vec<u8>
    let r: Vec<u8> = s1.into_bytes();

    // String -> OsString
    let r: OsString = OsString::from(s);

    //////////////////////////////////////////////////

    // &[u8] -> String
    let r: String = String::from_utf8(v1).unwrap();
    let r: String = String::from_utf8(u.to_vec()).unwrap();

    // &[u8] -> &str
    let r: &str = std::str::from_utf8(u).unwrap();
    let r: &str = std::str::from_utf8(&v).unwrap();

    // &[u8] -> Vec<u8>
    let r: Vec<u8> = u.to_owned();
    let r: Vec<u8> = u.to_vec();

    // &[u8] -> &OsStr
    let r: &OsStr = OsStr::from_bytes(u); // this requires os::unix::ffi::OsStrExt
    
    //////////////////////////////////////////////////
    
    // &[u8; 6] -> &[u8]
    let r: &[u8] = st.as_bytes();
    let r: &[u8] = &b[..];
    let r: &[u8] = &b"abc"[..];

    // &[u8; 6] -> [u8; 6]
    let r: [u8; 6] = b.to_owned();

    //////////////////////////////////////////////////
    
    // [u8; 6] -> Vec<u8>
    let r: Vec<u8> = bb.to_vec();
    
    // [u8; 6] -> &[u8; 6]
    let r: &[u8; 6] = &bb;

    // [u8; 6] -> &[u8]
    let r: &[u8] = bb.as_ref();
    let r: &[u8] = bb.as_slice();

    //////////////////////////////////////////////////
    
    // Vec<u8> -> &str
    let r: Vec<u8> = v.as_slice().to_vec();
    let r: &str = std::str::from_utf8(&v).unwrap();
    
    // Vec<u8> -> String
    let r: String = String::from_utf8(v2).unwrap();

    // Vec<u8> -> &[u8]
    let r: &[u8] = v.as_slice();

    // Vec<u8> -> OsString
    let r: OsString = OsString::from_vec(v); // this requires os::unix::ffi::OsStringExt
}
```

<br>

# Unit type `()`
**Unit type** or just **Unit** (denoted as `()`) is an **empty tuple**.<br>
It is **ZST** (**zero-sized type**).

<br>

**Unit** has **exactly one** value – `()`.
<br>

**Unit** and **its value** are the **same**.

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

## `..` operator
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

<br>

# Vectors
A **vector** is *collection* of elements of **the same type** that is allowed to **grow** or **shrink** *in size* **at runtime**.<br>
Vectors are **allocated** on the **heap**.<br>
`Vec` is a type for **vector** provided by the **standard library**.<br>

`capacity` is the number of elements the `Vec` can hold without reallocating.

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
- **Vector type constructor**:
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```
- **Repeat expression** where `N` = 0:
```Rust
let v = vec![100; 0];
```


<br>

## *Type declaration* syntax
- `Vec<T>`
```Rust
let v3: Vec<i64> = Vec::with_capacity(10);
```
