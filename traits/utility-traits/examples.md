# 1. Function that adds two numbers of any integer type
## Using trait ``Into``
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

fn main() {
    let a: u16 = 5;
    let b: u8 = 10;
    
    let s = add(a, b);
    println!("Sum of 'a' and 'b' = {}", s);
}

fn add<Tx, Ty>(x: Tx, y: Ty) -> u32
where
    Tx: Into<u32>,
    Ty: Into<u32>
{
    x.into() + y.into()
}
```

<br>

## Using trait ``From``
```Rust
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]

fn main() {
    let a: u16 = 5;
    let b: u8 = 10;
    
    let s = add(a, b);
    println!("Sum of 'a' and 'b' = {}", s);
}

fn add<Tx, Ty>(x: Tx, y: Ty) -> u32
where
    u32: From<Tx> + From<Ty>
{
    u32::from(x) + u32::from(y)
}
```

<br>

# 2. Function that accepts `&str` and `String`
Consider following example:<br>
```Rust
struct Person {
    name: String,
}

impl Person {
    fn new (name: &str) -> Person {
        Person { name: name.to_string() }
    }
}
```

<br>

This code has following **drawbacks**:
- we must remember to call `.to_string()` inside of the `new()`;
- caller must pass a reference to `String`, i.e., caller must use `&` operator explicitly for `String`;
- `String` will be **clonned** by `.to_string()` inside of the `new()`;
- But in order to convert to String, the called party (callee) needs to control its own memory allocation, and will have a copy.

<br>

## Using trait `Into`
Trait boundary `S: Into<String>` allows passing `String` and `&str` directly to `Person::new()`.<br>
But we also must **explicitly** call `.into()` inside `new`, but it is more general.<br>

```Rust
struct Person {
    name: String,
}

impl Person {
    fn new<S>(name: S) -> Person 
    where S: Into<String> 
    {
        Person { name: name.into() }
    }
}

fn main() {
    // let foo = String::from("Foo");
    let foo = "Foo".to_string();
    let bar = "Bar";
    let p1 = Person::new(bar);
    let p2 = Person::new(foo);
}
```

<br>

## Using trait `AsRef`
Trait boundary `S: Into<String>` allows passing `String` and `&str` directly to `Person::new()`.<br>
But we must **explicitly** call `.as_ref().to_string()` inside `new`, but it is more general.<br>

```Rust
struct Person {
    name: String,
}

impl Person {
    fn new<S>(name: S) -> Person 
    where S: AsRef<str> 
    {
        Person { name: name.as_ref().to_string() }
    }
}

fn main() {
    // let foo = String::from("Foo");
    let foo = "Foo".to_string();
    let bar = "Bar";
    let p1 = Person::new(bar);
    let p2 = Person::new(foo);
}
```

<br>

# 3. Function that returns `&str` and `String`
Consider function:<br>
```Rust
fn remove_spaces(input: &str) -> String {
   let mut buf = String::with_capacity(input.len());

   for c in input.chars() {
      if c != ' ' {
         buf.push(c);
      }
   }

   buf
}
```

<br>

If `input` hasn't spaces then `remove_spaces()` **allocates** new memory **anyway**.<br>
Obvious it needn't to create `buf` when `input` **hasn't** spaces and it's reasonable to return given `input` back to the caller. There are some ways to do that.<br>

<br>

## Using `String` for `input`
```Rust
fn remove_spaces(input: String) -> String { ... }
```

<br>

This solution has 2 drawbacks:
1. It forces the caller to **move** the ownership of `input` into function `remove_spaces()`.
2. It forces the caller to **convert** `&str` into a `String` which causes to allocation of new memory.

<br>

## Using `Cow` type for return value
```Rust
use std::borrow::Cow;

fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());

        for c in input.chars() {
            if c != ' ' {
                buf.push(c);
            }
        }

        return Cow::Owned(buf);
    }

    return Cow::Borrowed(input);
}
```


<br>

## Using `Cow` + `.into()`
```Rust
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        let mut buf = String::with_capacity(input.len());
        let v: Vec<char> = input.chars().collect();

        for c in v {
            if c != ' ' {
                buf.push(c);
            }
        }

        return buf.into();
    }
    return input.into();
}
```

<br>

## Using adapters
```Rust
fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str> {
    if input.contains(' ') {
        input
        .chars()
        .filter(|&x| x != ' ')
        .collect::<std::string::String>()
        .into()
    } else {
        input.into()
    }
}
```
