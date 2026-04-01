# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declaration](#declaration)
- [In a nutshell](#in-a-nutshell)
- [Example](#example)
<!-- TOC -->

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`Cow`|[**std::borrow::Cow**](https://doc.rust-lang.org/stable/std/borrow/enum.Cow.html)|

<br>

# Declaration
Let’s look at the **simplified version** of `Cow`:
```rust
enum Cow<B> {
    Borrowed(B),
    Owned(B),
}
```

<br>

Then let’s look at the **real version** of `Cow`:
```rust
enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

<br>

- the `'a` means that `Cow` **can** hold a reference;
- the `ToOwned` means that `B` **must** be a type that can be turned into an owned type;
- the `?Sized` means that `B` **might** be dynamically sized type;

<br>

# In a nutshell
The type `Cow` is a smart pointer providing **clone-on-write** functionality: 
- it provides **immutable** access to **borrowed** data;
- it performs **clone** the data lazily when **mutation** or **ownership** is required.

<br>

`Cow<T>` implements the `Deref` trait which means it can directly call the immutable methods of `T`.<br>
`.to_mut()` returns **mutable reference** to owned data. It **clones** the data if it is **not** already owned. Multiple calls to `.to_mut()` will produce only **one** `.clone()`.

<br>

If we need to **mutate** `T`, then we can convert it into an **owned** variable using the `into_owned()`:<br>
 - if the variant of `Cow` was already `Owned` then we **move ownership**;
 - if the variant of `Cow` is `Borrowed`, then we **allocate** new memory;

<br>

Imagine that you have a function that returns `Cow<'static, str>`:
- if you tell the function to return `"My message".into()`, it will look at the type: `"My message"` is a `str`, this is a `Borrowed` type, so it selects `Borrowed(&'a B)` and returns `Cow::Borrowed(&'static str)`;
- if you tell the function to return `format!("My message").into()`, it will look at the type: `format!("My message")` is a `String`, this is a `Owned` type, so it selects `Owned(String)` and returns `Cow::Owned(String)`;

<br>

The `Cow` has some other methods, like `into_owned()` or `into_borrowed()`, so you can change it if you need to.<br>

<br>

# Example
**Code**:
```rust
use std::borrow::Cow;
struct User<'a> {
    name: Cow<'a, str>,
}

fn main() {
    let user_1 = "User1";
    let user_2 = &"User2".to_string();
    let user_3 = "User3".to_string();
    
    let user1 = User {
        name: user_1.into(),
    };

    let user2 = User {
        name: user_2.into(),
    };

    let user3 = User {
        name: user_3.into(),
    };

    for name in [user1.name, user2.name, user3.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n {n}")
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n {n}")
            }
        }
    }
}
```
**Output**:
```rust
Borrowed name, didn't need an allocation:
 User1
Borrowed name, didn't need an allocation:
 User2
Owned name because we needed an allocation:
 User3
```
