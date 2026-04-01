# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Lifetimes](#lifetimes)
  - [Returning borrows](#returning-borrows)
  - [The anonymous lifetime](#the-anonymous-lifetime)
  - [Example: implement `Display`](#example-implement-display)
<!-- TOC -->

<br>

# Lifetimes
## Returning borrows
If a function returns a reference it was likely derived from one of its arguments. This means that a reference returned from a function **extends the borrow** for one or more arguments:
```rust
fn id(r: &i32) -> &i32 {
    r
}

fn main() {
    let mut number = 10;
    let number_ref = id(&number);
    number += 1; // ❌ ERROR: because 'number' is still borrowed here!
    println!("{}", number_ref);
}
```

<br>

**Consider another example**:
```rust
fn choose<'a>(r1: &'a i32, r2: &'a i32, flag: bool) -> &'a i32 {
    if flag {r1} else {r2}
}

fn main() {
    let mut number1 = 10;
    let mut number2 = 20;
    let number_ref = choose(&number1, &number2, false);

    // number1 += 1; // ❌ ERROR: because 'number1' is still borrowed here!
    // number2 += 1; // ❌ ERROR: because 'number2' is still borrowed here!

    println!("{}", number_ref);
}
```

The `choose()` function returns either `r1` or `r2` depending on the value of `flag`, which means the Rust **can't know at compile time** which one will be return.<br>
To express this to the compiler we provide the same lifetime **for all** `r1`, `r2` and for *returned value*, this means that *returned reference* **borrows both input references**: `r1` and `r2`.<br>
In other words, the **borrow** of both `number1` and `number2` **lasts until last usage of** `number_ref`.<br>

<br>

**Example**:
```rust
fn foo() -> &'static str {
    // &"foo".to_string() // ❌ ERROR:
    "foo" // Ok
}
```

The `&'static str` tells Rust we will only return a **string literals** which live for the whole program.<br>

<br>

## The anonymous lifetime
Consider example:
```rust
struct Foo<'a> {
    name: &'a str,
}

impl Foo {
    fn new() -> Self {
        Self {
            name: "foo",
        }
    }
}
```

This code **will not** compile, because you must specify struct with all its lifetimes `Foo<'a>` and also declare lifetimes for whole `impl` block: `impl<'a>`:
```rust
impl<'a> Foo<'a> {

}
```

The **anonymous lifetime** was made so that you don’t always have to write things like `impl<'a> Foo<'a>`. The *anonymous lifetime* is an indicator that references are being used.<br>

**Example**:
```rust
struct Foo<'a> {
    name: &'a str,
}

impl Foo<'_> {
    fn new() -> Self {
        Self {
            name: "foo",
        }
    }
}
```

<br>

But why Rust requires to declare lifetimes for whole `impl` block?<br>

Consider **trait** `Bar` that that needs to deal with lifetime:
```rust
trait Bar<'a> {
    fn bar(r1: &'a i32, r2: &'a i32, flag: bool) -> &'a i32 {
        if flag { r1 } else { r2 }
    }
    fn bar2<'z>(r1: &'z i32, r2: &'z i32, flag: bool) -> &'z i32 {
        if flag { r1 } else { r2 }
    }
}
```

And you want to implement `Bar` on struct `Foo`: the **trait** has its **own lifetimes** to deal with, and the **struct** has its **own lifetimes** to deal with. Both the
struct and the trait choose to call them `'a`.<br>

So, when you use `impl`, you declare lifetimes and their relationships.<br>

You might implement trait `Bar` like this:
```rust
impl<'a> Bar<'a> for Foo<'a> {
    fn bar(r1: &'a i32, r2: &'a i32, flag: bool) -> &'a i32 {
        if flag { r1 } else { r2 }
    }

    fn bar2<'z>(r1: &'z i32, r2: &'z i32, flag: bool) -> &'z i32 {
        if flag { r1 } else { r2 }
    }
}
```

**This means** the `'a` for the *trait* and the *struct* is the **same lifetime**.<br>

<br>

But you might implement trait `Bar` like this:
```rust
impl<'a, 'b> Bar<'a> for Foo<'b> {
    fn bar(r1: &'a i32, r2: &'a i32, flag: bool) -> &'a i32 {
        if flag { r1 } else { r2 }
    }

    fn bar2<'z>(r1: &'z i32, r2: &'z i32, flag: bool) -> &'z i32 {
        if flag { r1 } else { r2 }
    }
}
```

**This means** there are **2 different lifetimes** here, the *trait* has its own while the *struct* has its own two.<br>

<br>

But you also can use **anonymous lifetime**:
```rust
impl Bar<'_> for Foo<'_> {
    fn bar<'x>(r1: &'x i32, r2: &'x i32, flag: bool) -> &'x i32 {
        if flag { r1 } else { r2 }
    }

    fn bar2<'z>(r1: &'z i32, r2: &'z i32, flag: bool) -> &'z i32 {
        if flag { r1 } else { r2 }
    }
}
```

**Note**, that when you use **anonymous lifetime** you must add lifetime parameter for methods: in the example above it was method `bar`.<br>

<br>

## Example: implement `Display`
```rust
struct Foo<'a> {
    name: &'a str,
}

impl Foo<'_> {
    fn new() -> Self {
        Self {
            name: "foo",
        }
    }
}

impl std::fmt::Display for Foo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "I am {}", self.name)
    }
}

fn main() {
    println!("{}", Foo::new())
}
```

<br>