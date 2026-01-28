# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Vocab](#vocab)
- [Patterns](#patterns)
  - [`match` expressions](#match-expressions)
    - [Examples](#examples)
      - [Assigning result of `match`](#assigning-result-of-match)
      - [Match value against separate integer values](#match-value-against-separate-integer-values)
      - [Match value against several values and range](#match-value-against-several-values-and-range)
      - [Match value against boolean values](#match-value-against-boolean-values)
      - [Match guards](#match-guards)
  - [Literal patterns](#literal-patterns)
  - [Identifier patterns](#identifier-patterns)
  - [Wildcard patterns](#wildcard-patterns)
  - [Rest patterns](#rest-patterns)
  - [Range patterns](#range-patterns)
  - [Reference patterns](#reference-patterns)
  - [Struct patterns](#struct-patterns)
  - [Tuple struct patterns](#tuple-struct-patterns)
  - [Tuple patterns](#tuple-patterns)
  - [Slice patterns](#slice-patterns)
- [Destructuring](#destructuring)
  - [Destructuring structs](#destructuring-structs)
    - [Examples](#examples-1)
      - [Destructuring structs](#destructuring-structs-1)
      - [Destructuring nested structs](#destructuring-nested-structs)
  - [Destructuring tuples](#destructuring-tuples)
  - [Destructuring arrays/slices](#destructuring-arraysslices)
  - [Destructuring pointers/ref](#destructuring-pointersref)
    - [Examples](#examples-2)
      - [Borrow values in `for` loop](#borrow-values-in-for-loop)
      - [Borrow reference](#borrow-reference)
      - [Borrow value inside `Option`](#borrow-value-inside-option)
      - [Nested variant](#nested-variant)
- [if let](#if-let)
- [while let](#while-let)
<!-- TOC -->

<br>

# Vocab
- **scrutinize** /ˈskruː.tɪ.naɪz/ to examine something very carefully;
- **scrutiny** /ˈskruːtɪnɪ/ careful examination of something;
- **scrutineer** /ˌskruː.tɪˈnɪər/ a person who scrutinises;

<br>

# Patterns
**Patterns** are used in:
- signatures of *functions* and *closures*;
- [`let` declarations](https://doc.rust-lang.org/reference/statements.html#let-statements);
```rust
let Pattern: Type = Expression;
```
- [`match` expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html);
```rust
match Scrutinee {
    Match_Arm_1 => Expression (without block or with block),
    ...
    Match_Arm_N => Expression (without block or with block),
}
```
- [`if let` expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html);
```rust
if let Pattern = Scrutinee { }
if let Pattern = Scrutinee { } else { }
if let Pattern = Scrutinee { } else if condition {} else { }
```
- [`while let` loops](https://doc.rust-lang.org/reference/expressions/loop-expr.html#while-let-patterns);
```rust
while let Pattern = Scrutinee { }
```
- [`for` loops](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops);
```rust
for Pattern in Expression { }
```

<br>

**Scrutinee** is the **value** (it can have form of **expression**) that is matched against **patterns**. The *scrutinee* and the *patterns* must have the **same type**. For example, in `match x {A => (), B => ()}` the expression `x` is the *scrutinee*.<br>

A `match`, `if let` and `while let` expressions all have a **scrutinee**, which is the **value** to compare to the patterns.<br>

**Multiple** *patterns* may be **joined** with the `|` operator in `match`, `if let` and `while let` expressions. *Each pattern* will be **tested** in **left-to-right** sequence **until** a *successful match* is found.<br>

<br>

*Pattern* can accept **pattern guard** (aka **match guard** in the `match` *expression*) to further refine the criteria for matching a case.<br>
**Pattern guard** must appear **after** the *pattern* and consist of a **bool-typed expression** following the `if` keyword.<br>
When the *pattern* matches successfully, the *pattern guard* expression is executed. If the *pattern guard* expression evaluates to `true`, the **whole** *pattern* is successfully matched against. Otherwise, the **next** *pattern*, including other matches with the `|` operator in the same arm, is tested.<br>
A *pattern guard* may refer to the **variables bound** within the *pattern* they follow.<br>

<br>

**Refutable pattern** **NOT** *always* matches the *expression*/*value* it is being matched against.<br>
**Irrefutable pattern** *always* matches the *expression*/*value* it is being matched against.<br>

**Examples**:
```rust
let (x, y) = (1, 2); // here (x, y) is an irrefutable attern
if let (a, 3) = (1, 2) { ... }; // here (a, 3) is a refutable attern, but will not match against (1,2)
```
<br>

There are available many patterns:
- **literal** patterns;
- **identifier** patterns;
- **wildcard** patterns;
- **rest** patterns;
- **range** patterns;
- **reference** patterns;
- **struct** patterns;
- **tuple struct** patterns;
- **tuple** patterns;
- **slice** patterns;

<br>

## `match` expressions
In the `match` *expression*:
- *each line* is called an **arm**;
- *each* **arm** consist of **match arm** (*pattern* or *joined patterns*) and **expression**;
- *each* **arm** has to **return** the **same type**;
- *each* **arm** must be separated through a **comma** (**not** a *semicolon*);
- `match` always **stops** when it finds the first *pattern* that matches successfully* and **doesn’t** *check the rest*;

<br>

### Examples
#### Assigning result of `match`
```rust
fn main() {
    let my_number = 5;
    let second_number = match my_number {
        0 => 0,
        5 => 10,
        _ => 2,
    };
}
```

<br>

#### Match value against separate integer values
```rust
fn main() {
    let my_number: u8 = 5;
    match my_number {
        0 => println!("it's zero"),
        1 => println!("it's one"),
        2 => println!("it's two"),
        _ => println!("It's some other number"),
    }
}
```

<br>

#### Match value against several values and range
```rust
fn main() {
    let number = 7;

    match number {
        1 => println!("arm 1: {}", number),
        2 | 7 | 11 => println!("arm 2 | 7 | 11: {}", number),
        11..=20 => println!("arm 11..=20: {}", number),
        _ => println!("Others: {}", number),
    }
}
```

<br>

#### Match value against boolean values
```rust
fn main() {
    let boolean = true;
    let binary = match boolean {
        false => println!("false"),
        true => println!("true"),
    };
}
```

<br>

#### Match guards
```rust
fn match_colors(rgb: (i32, i32, i32)) {
    match rgb {
        (r, _, _) if r < 10 => println!("Not much red"),
        (_, g, _) if g < 10 => println!("Not much green"),
        (_, _, b) if b < 10 => println!("Not much blue"),
        _ => println!("Each color has at least 10"),
    }
}

fn main() {
    let first = (200, 0, 0);
    let second = (50, 50, 50);
    let third = (200, 50, 0);
    
    match_colors(first);
    match_colors(second);
    match_colors(third);
}
```

<br>

## Literal patterns
**Literal pattern matches** against **literal value**.<br>
*Literal pattern* is always **refutable**.<br>

**Example**:
```rust
match a {
    1 => (),
    2 | 3 => (),
    _ => (),
}
```

<br>

## Identifier patterns



Pattern binding is useful when we **don't** have identifier and match agains function result, in such case we need bound values to variable inside arms:
```rust
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13..= 19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}
```

<br>

In the above example when `age()` return value in `[1; 12]` this value is **matched** by `n @ 1..= 12` arm and this value is **assigned** to `n` variable.<br>

<br>

**Another example**:
```rust
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        Some(n @ 42) => println!("arm 'Some(n @ 42)', value: {}", n),
        Some(n) => println!("arm 'Some(n)', value: {}", n),
        _ => (),
    }
}
```

<br>

**Identifier pattern binds** the value it matches to a variable and this variable shadows any variable with the same name in a scope.<br>
The scope of new binding depends on the context of where the pattern is used: **let binding** or **match expression**. For example, variables bound within the *pattern* of `match` *arms* are scoped to the **match guard** and the **arm's block**.<br>

The *identifier pattern* that consist of **only** an *identifier* **matches any value** and **binds it** to *that identifier*. This is the most commonly used pattern in *variable declarations* and *functions/closures parameters*.<br>

It is possible to narrow the scope of *identifier pattern*. There is syntax: `identifier @ subpattern`. So, *identifier pattern* is **irrefutable** if the `subpattern` is **not** specified or the `subpattern` is **irrefutable**.<br>

<br>

## Wildcard patterns
The **wildcard pattern** is an **underscore symbol** `_`.<br>
It is used to **ignore values** when they **don’t matter** and it is **always irrefutable**.<br>
The *wildcard pattern* **matches any value**, but unlike *identifier pattern* it **doesn't** *copy*, *move* or *borrow* the *matched value*.<br>
Inside `struct`, `tuple` or `tuple struct` it **skips** *concrete field*.<br>
Inside `enum` expression it **skips** the *remainig variants*.<br>

```rust
let (a, _) = (10, x); // the x is always matched by _
let a = |a: i32, _: i32| { a }; // here _ is used to gnote closure parameter
```

<br>

## Rest patterns
The **rest pattern** is an **et cetera token** `..`.<br>
Inside `struct`, `tuple` or `tuple struct` it **skips** the *remainig fields*.<br>
The *rest pattern* is **always irrefutable**.<br>

<br>

## Range patterns
The **range pattern** matches **scalar values** within the range defined by their bounds:
- `low..high` - **exclusive** *range pattern* **doesn't** include *upper bound*;
- `low..=high` - **inclusive** *range pattern*;

```rust
match x {
    c @ 0..=5 => (), // the variable c will be bound to value from [0; 5]
    7..10 => (),
    _ => (),
}
```

<br>

## Reference patterns
```rust
let int_reference = &3;

let a = match *int_reference { 0 => "zero", _ => "some" };
let b = match int_reference { &0 => "zero", _ => "some" };

assert_eq!(a, b);
```

<br>

## Struct patterns
**Struct pattern** **matches** *struct values* that match all criteria defined by its subpatterns.<br>
It is also used to **destructure** a *struct value*.<br>

```rust
match s {
    Point {x: 10, y: 20} => (),
    Point {y: 10, x: 20} => (),    // order doesn't matter
    Point {x: 10, ..} => (),
    Point {..} => (),
}
```

<br>

## Tuple struct patterns
**Tuple struct pattern matches** *tuple struct values*.<br>
It is also used to **destructure** a *tuple struct values*.<br>

```rust
match ts {
    PointTuple {0: 10, 1: 20} => (),
    PointTuple {1: 10, 0: 20} => (),   // order doesn't matter
    PointTuple {0: 10, ..} => (),
    PointTuple {..} => (),
}
```

<br>

## Tuple patterns
**Tuple pattern matches** *tuple values*.<br>
It is also used to **destructure** a *tuple values*.<br>

```rust
let pair = (10, "ten");
let (a, b) = pair;

assert_eq!(a, 10);
assert_eq!(b, "ten");
```

<br>

## Slice patterns
**Slice pattern matches** both *arrays of fixed size* and *slices of dynamic size*.<br>
It is also used to **destructure** a *arrays* and *slices*.<br>

```rust
let arr = [1, 2, 3];

match arr {
    [1, _, _] => "starts with one",
    [a, b, c] => "starts with something else",
};

let v = vec![1, 2, 3];
match v[..] {
    [a, b] => { /* this arm will not apply because the length doesn't match */ }
    [a, b, c] => { /* this arm will apply */ }
    _ => { /* this wildcard is required, since the length is not known statically */ }
};
```

<br>

# Destructuring
*Patterns* can be used to **destructure** *complex types*: *structs*, *enums*, *tuples*, *unions*.<br>
**Destructuring** breaks up a *expression*/*value* of complex type into its **constituents**.<br>
The syntax used in a *pattern* that **destructures** an object is analogous to one used by the expression which **created** it.<br>
In other words, *destructuring* **unpacks** the value is being matched against *into its constituent values*. Such *unpacked values* can be assigned to variables, this is called **pattern binding**.<br>

Example, **destructuring** allows to **bind** *structs's fields* to separate variable at once:
```rust
#[derive(Debug)]
struct Person {
    car: Option<String>,
    age: u32,
    name: String,
    surname: String,
    date_of_birth:u32,
}

fn main() {
    let p = Person{
        car: None,
        age: 40,
        name: "Anton".to_string(),
        surname: "a".to_string(),
        date_of_birth: 10,
    };

    match p {
        Person{age: person_age@ 0..=11, ..} => println!("age={}", person_age),
        Person{car: Some(_), ..} => println!("person={:?}", p),
        Person{name: ref person_name, ..} if person_name == "Anton" => println!("person_name={:?}", person_name),
        Person{ref name, ..} => println!("name={:?}", name),
        _ => (),
    }
}
```

<br>

Example, **destructuring** allows to **assign** *tuple's elements* to separate variable at once:
```rust
fn main() {
    let strings = ("one".to_string(), "two".to_string(), "three".to_string());
    let (a, b, c) = strings;
    println!("{b}");
    // println!("{strings:?}");  This wouldn’t compile, 
}
```

<br>

**By default**, all *unpacked values* are **copied** or **moved** into the scope, depending on whether the *unpacked value* implements `Copy`.<br>
In the previous above, a `String` is **not** a `Copy` type, so the values are moved into `a`, `b`, and `c`, and variable `strings` **can’t** be accessed anymore.<br>

<br>

There are 2 binding modes:
- **move** or **copy** (by default);
- **borrow**;

<br>

To **borrow** *unpacked value* there is keyword `ref/ref mut`. The `ref` keyword is needed because the `&` operator within a pattern is a **part of pattern**. For example, if `&` was used to **create** the value, it needs to be used within *pattern*. In the *pattern matching* `ref` or `ref mut` is **not** *part of pattern*, it annotates *pattern bindings* to **borrow** unpacked value, **not** *move*.<br>

So,
- the variable `n` in the pattern `Some(n)` is of type `String`: the **actual** *unpacked value*;
- the variable `n` in the pattern `Some(ref n)` is of type `&String`: a *reference* to the *unpacked value*;
- the variable `n` in the pattern `Some(ref mut n)` is of type `&mut String`: a **mutable** *reference* to the *unpacked value*;

<br>

So,
- `&` denotes that your **pattern expects a reference to an object**. Hence `&` is a part of pattern: `&Foo` matches **different** objects than `Foo` does;
- `ref` indicates that you want a **reference** to an *unpacked value*. It is not matched against: `Foo(ref foo)` matches the **same** objects as `Foo(foo)`;

<br>

Here the *unpacked value* from the value `a` will be **copied** (or **moved**):
```rust
match a {
    Some(value) => (),
    None => (),
}
```

<br>

Here the *unpacked value* from the value `a` will be **borrowed**:
```rust
match a {
    Some(ref value) => (),
    None => (),
}
```

<br>

## Destructuring structs
- to **create struct**: `let p = Foo { a: u32, b: String }`;
- to **destructure struct**: `let Foo { a, b } = p;` - field names becomes **separate variables** `a` and `b`;
  - also you can **rename variables** while destructuring: `let Foo { a: new_name_for_a, b: new_name_for_b } = p;`
  - also you can explicitly choose some fields and **skip others**, just use `..`: `let Foo { a: new_name_for_a, .. } = p;`
  - also you can destructure **inside the signature** of a function: `fn bar (Foo { a, b, .. }: &Foo)`
    - then you can use variables `a` and `b` inside function;

<br>

Consider example:
```rust
struct Foo {a: u32, b: u32}
```

Possible arms:
- `Foo {a, b} => {},`
  - **matches** value `foo` of type `Foo` with **any values**;
  - **binds** `foo.a` to *variable* `a`;
  - **binds** `foo.b` to *variable* `a`;
- `Foo {a: new_name_for_a, b: new_name_for_b} => {},`
  - **matches** value `foo` of type `Foo` with **any values**;
  - **binds** `foo.a` to *variable* `new_name_for_a`;
  - **binds** `foo.b` to *variable* `new_name_for_b`;
- `Foo {a: 2, b: 5} => {},`
  - **matches** value `foo` of type `Foo` **only** if with `foo.a == 2` and `foo.b == 5`;
  - **binds** `foo.a` to *variable* `a`;
  - **binds** `foo.b` to *variable* `a`;
- `Foo {a, b} if a == 2 and b == 5 => {},`
  - **matches** value `foo` of type `Foo` **only** if with `foo.a == 2` and `foo.b == 5`;
  - **binds** `foo.a` to *variable* `a`;
  - **binds** `foo.b` to *variable* `a`;
- `Foo { a: new_name_for_a @ 0..=10, .. }`
  - **matches** value `foo` of type `Foo` **only** if with `foo.a in range [0; 10]`;
  - **binds** `foo.a` to *variable* `a`;
  - **ignores rest fields**;

<br>

### Examples
#### Destructuring structs
```rust
struct Person {
    age: u32,
    name: String,
}

fn main() {
    let p = Person {age:30, name: "Anton".to_string()};
    let Person {
        age,
        name,
    } = p;
    println!("Age is {age}; name is {name}.");
    // println!("Age is {}; name is {}.", p.age, p.name); // p.name is not accessible, because it's String and was moved

    let p2 = Person {age:40, name: "Petr".to_string()};
    let Person {
        age: x,
        name: y,
    } = p2;
    println!("Age is {x}; name is {y}.");
}
```

<br>

#### Destructuring nested structs
```rust
fn main() {
    #[derive(Debug)]
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    
    struct Bar {
        foo: Foo,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let bar = Bar { foo: Foo { x: (55, 77), y: 33 } };

    let Bar { foo: Foo { x: nested_x, y: nested_y } } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");

    let Bar { foo: nested_foo } = bar;
    println!("Nested: nested_foo = {nested_foo:?}");
}
```

<br>

## Destructuring tuples
Consider example:
```rust
let triple: (u32, i32, u8) = (0_u32, -2, 3_u8);
```

Possible arms:
- `(a, .., b) => {},`
  - **binds** the **first** value to *variable* `a`;
  - **binds** the **last** value to *variable* `b`;
  - **ignores** the *rest of the tuple*;
- `(0, y, z) => {},`
  - **matches** only **tuples** where the **first** value is `0`;
  - **binds** the **second** value to *variable* `y`;
  - **binds** the **third** value to *variable* `z`;
- `(x @ 0, y, z) => {},`
  - **matches** only **tuples** where the **first** value is `0`;
  - **binds** the **first** value to *variable* `x`;
  - **binds** the **second** value to *variable* `y`;
  - **binds** the **third** value to *variable* `z`;
- `(x, y, z) if x == 0 => {},`
  - **matches** only **tuples** where the **first** value is `0`;
  - **binds** the **first** value to *variable* `x`;
  - **binds** the **second** value to *variable* `y`;
  - **binds** the **third** value to *variable* `z`;
- `(1, ..) => {},`
  - **matches** only **tuples** where the **first** value is `1`;
  - the **rest values doesn't matter**;
- `(_, a, _) => {},`
  - **binds** the **second** value to *variable* `a`;
  - **single values** can be ignored with `_`;
- `(.., 2) => {},`
  - **matches** only **tuples** where the **last** value is `2`;
  - the **rest values doesn't matter**;
- `(3, .., 4) => {},`
  - **matches** only **tuples** where the **first** value is `3`;
  - the **last** value is `4`;
  - **ignores** the *rest of the tuple*;

<br>

## Destructuring arrays/slices
Consider example:
```rust
let array = [1, -2, 6, 8];
```

Possible arms:
- `[a, .., b] => {},`
  - **binds** the **first** value to *variable* `a`;
  - **binds** the **last** value to *variable* `b`;
  - **ignores** the *rest of the tuple*;
- `[0, x, y, z] => {},`
  - **matches** only **arrays/slices** where the **first** value is `0`;
  - **binds** the **second** to *variable* `x`;
  - **binds** the **third** to `y`;
  - **binds** the **fourth** to `z`;
- `[1, ..] => {},`
  - **matches** only **arrays/slices** where the **first** value is `1`;
  - the **rest doesn't matter**;
- `[_, a, _, _] => {},`
  - **binds** the **second** value to *variable* `a`
  - **single values** are **ignored** with `_`;
- `[.., 2] => {},`
  - **matches** only **arrays/slices** where the **last** value is `2`;
  - the **rest doesn't matter**;
- `[3, .., 4] => {},`
  - **matches** only **arrays/slices** where the **first** is `3` and the **last** value is `4`;
  - the **rest doesn't matter**;
- with `@` bindings:
  - `[first, middle @ .., last] => {},`
    - **binds** the **first** value to *variable* `first`;
    - **binds** the **last** value to *variable* `last`;
    - **binds** the **rest** values to *variable* `middle`;
  - `[_, middle @ .., _] => {},`
    - **ignores** the **first** and the **last** values;
    - **binds** the **rest** values to *variable* `middle`;
  - `[_, tail @ ..] => {},`
    - **ignores** the **first**
    - **binds** the **rest** values to *variable* `tail`;

<br>

## Destructuring pointers/ref
Destructuring `pointers`/`ref` uses `&`, `ref`, and `ref mut`.<br>

**Example 1** (**doesn't work**):
```rust
fn main() {
    let s = "foo".to_string();
    let reference = &s;

    // Doesn't work!
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // Doesn't work!
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
}
```

Both variants returns **error**:
```bash
error[E0507]: cannot move out of `*reference` which is behind a shared reference
```

<br>

**Example 2** (**doesn't work**):
```rust
fn do_something_with(input: Option<String>) {}

fn main() {
    let some_value = Some(String::from("Foo"));
    match some_value {
        Some(n) => println!("Hello, {}", n),
        _ => {},
    }
    do_something_with(some_value)
}
```
**Output**:
```rust
error[E0382]: use of partially moved value: `some_value`
```

<br>

### Examples
#### Borrow values in `for` loop
```rust
use hyper::Url;

// print query string params of some URL
let url = Url::parse(some_url).unwrap();
let query_params: Vec<(String, String)> = url.query_pairs().unwrap_or(vec![]);
for &(ref name, ref value) in &query_params {
    println!("{}={}", name, value);
}
```

**Without** the `ref`s we will attempt to **move** those items into the `loop` scope.<br>

<br>

#### Borrow reference
```rust
fn main() {
    let s = "foo".to_string();
    let reference = &s;

    // Works!
    match reference {
        ref val => println!("Got a value via destructuring: {:?}", val),
    }
}
```

Here `val` is **reference to reference**: `&&s`.<br>

<br>

#### Borrow value inside `Option`
```rust
fn do_something_with(input: Option<String>) {}

fn main() {
    let some_value = Some(String::from("Foo"));
    match some_value {
        Some(ref n) => println!("Hello, {}", n),
        _ => {},
    }
    do_something_with(some_value)
}
```

<br>

#### Nested variant
Here `field @ (x, ref y)` allows borrowing `y` **without moving** field out of value, which would otherwise cause a partial move error:
```rust
fn main() {
    struct S { field: (i32, i32) }
    let value = S { field: (10, 20) };
    match value {
        S { field: nested @ (x, ref y), } => {
            println!("Nested: {:?}, x: {}, y: {}", nested, x, y);
        }
    }
}
```

<br>

# if let
The `if let Some(i) = number` means: **if** `let` **destructures** `number` **into** `Some(i)`, **evaluate the block** `{ }`:
```rust
fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Another branch");
    }
}
```

<br>

Another benefit is that `if let` allows us to match **unit variants**. This is true even in cases where the enum **doesn't** implement `PartialEq`.<br>
If enum **doesn't** implement `PartialEq`, then `if Foo::Bar == x` would fail to compile, because instances of the enum **cannot** *be equated*, however `if let` will **continue** to work.<br>

Example:
```rust
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    if Foo::Bar == a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("YES");
    }

    if let Foo::Bar = a {
        println!("This works!");
    }
}
```

<br>

# while let
The `while let Some(i) = optional` means: **while** `let` **destructures** `optional` **into**  `Some(i)`, **evaluate the block** `{}`, **else** `break`:
```rust
fn main() {
    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into  `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}
```
