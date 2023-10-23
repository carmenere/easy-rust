# Lifetime
A **lifetime** is the **scope** within which a **reference** is **valid**.<br>

The notation `'a` is **lifetime** `a`.<br>

Technically, **every reference** **has** some **lifetime** associated with it, but the compiler lets you **elide** them in common cases.<br>
It is called **lifetime elision** or **implicit lifetime annotation**. It is because the Rust compiler is smart enough to infer lifetimes in many cases.<br>
But sometimes it is needed to specify lifetimes **explicitly**. That’s because of how the lifetime elision works.<br>
When a function accepts **multiple references**, they’re **each** given **their own lifetime**.<br>

From Rust point of view, signature:`fn f (s1: &str, s2: &str) → &str ` is **equal** to signature:`fn f<'a, 'b> (s1: &'a str, s2: &'b str) → &'??? str`<br>

So, `rustc` sets to `s1` and `s2` **different** lifetimes and `rustc` **doesn't** know what lifetime to assign to **returning value**.<br>
That is why compiler return error. So we must **explicitly** set lifetimes for input and output parameters.<br>
Example: `fn f<'a> (s1: &'a str, s2: &'a str) → &'a str`.<br>

The **lifetimes** help Rust find **dangling pointers**. Example:<br>
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

fn main() {
    let s1 = String::from("ABC");
    let result: &str;
    {
        let s2 = String::from("XYZ");
        result = longest_string(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}", result);

}


fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    }
    else {
        s2
    }
}
```

```bash
error[E0597]: `s2` does not live long enough
 --> src/main.rs:6:46
  |
5 |         let s2 = String::from("XYZ");
  |             -- binding `s2` declared here
6 |         result = longest_string(s1.as_str(), s2.as_str());
  |                                              ^^^^^^^^^^^ borrowed value does not live long enough
7 |     }
  |     - `s2` dropped here while still borrowed
8 |     println!("The longest string is {}", result);
  |                                          ------ borrow later used here

For more information about this error, try `rustc --explain E0597`.
```

<br>

We get error because compiler assigns the **lifetime** of the `result` variable to the **smallest** lifetime of passed arguments' lifetimes.<br>
In general `s1` and `s2` both can have **different** lifetimes. So compiler assigns the **lifetime** of the **returned value** to the **smallest lifetime** of passed parameters.<br>
It means that the **lifetime** of varibale `result` can last until the end of scope where argument with **smallest lifetime** goes **out of scope**.

<br>

## Lifetimes in functions
For function there are 2 kind of **lifetime parameters**:
- **Input lifetime parameter** is a lifetime associated with a **parameter** of a function. 
- **Output lifetime parameter** is a lifetime associated with the **return value** of a function.

<br>

## Lifetimes rules
**Lifetimes rules**:
1. Each function’s parameter that is **reference** gets its own **lifetime parameter** (aka **elided lifetime**).
2. If there is exactly **one input** *lifetime parameter*, it is assigned to **all output** *lifetime parameters*.
3. If there are **multiple input** *lifetime parameters*, but one of them is `&self` or `&mut self`, the **lifetime** of `self` is assigned to **all output** *lifetime parameters*.

<br>

Declaration `fn foo<'a>(a: &'a i32, b: &'a i32) -> () { ... }` means that **lenders** of `a` and `b` can't go out of their scopes until function execution ends, i.e., **lenders** of `a` and `b` must live as long as function call.

<br>

Declaration `fn foo<'a>(a: &'a i32, b: &'a i32) -> Foo<'a> { ... }` means that **return value** of function should live as long as **input argument** of function, i.e., **lenders** of `a` and `b` must live as long as **return value** of function.

<br>

## Lifetimes in Structs
```Rust
struct Foo<'a> {
    x: &'a i64,
}

fn main() {
    let nref = &1;
    let f = Foo { x: nref };
    println!("{}", f.x);
}
```

<br>

## Lifetimes in impl blocks
```Rust
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let nref = &1;
    let f = Foo { x: nref };
    println!("x is: {}", f.x());
}
```

<br>

## 'static
The lifetime named `'static` is a special lifetime. It signals that something has the **lifetime of the entire program**.<br>
**String literal** has the type `&str`, but under the hood, `&str` is `&'static str` because the **reference** is **always alive**: it's **hardcoded into the data segment of the final binary**.

`'static` *lifetime* **allows return reference to value from function**:
```Rust
fn create_string() -> &'static str {
    let s = String::from("abc");
    &s
}
```

<br>

## Lifetime subtyping
Consider following code:
```Rust
fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if *x > *y {
        x
    } else {
        y
    }
}
```

```bash
cargo run
   Compiling playrs v0.1.0 (/Users/an.romanov/Projects/play.rust-lang.org)
error[E0623]: lifetime mismatch
  --> src/main.rs:10:9
   |
6  | fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
   |                               -------     -------
   |                               |
   |                               this parameter and the return type are declared with different lifetimes...
...
10 |         y
   |         ^ ...but data from `y` is returned here

For more information about this error, try `rustc --explain E0623`.
error: could not compile `playrs` due to previous error
```

This program **doesn't compile**, because the lifetimes `'a` and `'b` are **independent**.<br>

Rust allows you to declare that lifetime `a` contains another lifetime. It is called **lifetime subtyping**.<br>
Notations of **lifetime subtyping**:<br>
- `fn max<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32`;
- `fn max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 where 'b: 'a`;

<br>

Notation `'left: 'right` means `'left` **outlives** `'right`, and `'left` is a **subtype** of `'right`, i.e., `'right` <= `'left`.
