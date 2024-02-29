# Table of contents
- [Table of contents](#table-of-contents)
- [`std`](#std)
- [Coercions](#coercions)
- [Reference type and Dereferenceable type](#reference-type-and-dereferenceable-type)
- [`Deref` trait](#deref-trait)
    - [Example](#example)
- [`DerefMut` trait](#derefmut-trait)
- [Deref coercion](#deref-coercion)
- [Deref coercion cases](#deref-coercion-cases)
- [Dot `.` operator](#dot--operator)
- [Examples](#examples)

<br>

# `std`
|Trait|Path in `std`|
|:----|:------------|
|`Deref`|[std::ops::Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html)|
|`DerefMut`|[std::ops::DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)|

<br>

# Coercions
**Type coercions** are **implicit** type conversions.

<br>

More about type here [type coercions in Rust](https://doc.rust-lang.org/reference/type-coercions.html)

<br>

# Reference type and Dereferenceable type
What types can be **dereferenced**?<br>
A type can be **dereferenced** if it **dereferenceable type**.<br>
**Dereferenceable type** is a type that implements the `Deref` and/or `DerefMut` traits.<br>
**Reference type** is a type of **reference** that was created by `&` **reference operator**.<br>
Compiler automatically adds `Deref` and/or `DerefMut` traits for **reference types**.<br>
So, **reference type** is **dereferenceable type**.<br>

Non-pointer types like `bool` or `char` or `(u8, u8)` **cannot** be **dereferenced**: they **don't** implement the `Deref` trait and **don't** act like pointers to some other type.<br>

```Rust
fn foo(b: &bool) -> bool { *b }
```

<br>

# `Deref` trait
`Deref` trait is used for **immutable** dereferencing operations, like `let v = *s;`.<br>

**Defenition** of `Deref` trait:
```Rust
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
```

### Example
```Rust
use std::ops::Deref;

struct DerefExample<T> {
    value: T
}

impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

let x = DerefExample { value: 'a' };
assert_eq!('a', *x);
```

<br>

`deref` method returns a **reference** to the value we want to access with the `*` **dereference operator**.<br>
Without the `Deref` trait, the compiler can only **dereference** **reference types**.<br>
The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a **reference type** that it knows how to dereference.

When we type `*y` in our code, behind the scenes Rust actually converts it to: `*(y.deref())`.<br>

<br>

# `DerefMut` trait
`DerefMut` trait is used for **mutable** dereferencing operations, like `*v = 5;`.<br>

**Defenition** of `Deref` trait:
```Rust
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

When we type `*y` in our code, behind the scenes Rust actually converts it to: `*(y.deref_mut())`.<br>

<br>

# Deref coercion
**Deref coercion** converts a *reference* to a **dereferenceable type** into a *reference* to **another type**.

To see **deref coercion** in action, consider function `hello` that has the parameter `name` of type `&str`:
```Rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}
```

This `hello` function receives a **string slice** as an **argument**, such as `hello("Rust");`.<br>
**Deref coercion** makes it possible to call `hello` with a **reference** to a value of type `MyBox<String>`:

```Rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

Here we’re calling the `hello` function with the argument `&m`, which is a **reference** to a `MyBox<String>` value.<br>
Rust performs following **deref chain**: 
- `&MyBox<String>` -> `&String` (because `MyBox<T>` implements `Deref` trait such that it returns `&String`);
- `&String` -> `&str` (because `String` implements `Deref` trait such that it returns `&str`).

<br>

# Deref coercion cases
|From => To|Trait boudary|Description|
|:---------|:-------------|:----------|
|`&T` => `&U`|`T: Deref<Target=U>`|If you have a `&T`, and `T` implements `Deref` to some type `U`, compiler will **coerce** `&T` into `&U` **transparently**.|
|`&mut T` => `&mut U`|`T: DerefMut<Target=U>`|If you have a `&mut T`, and `T` implements `DerefMut` to some type `U`, compiler will **coerce** `&mut T` into `&mut U` **transparently**.|
|`&mut T` => `&U`|`T: Deref<Target=U>`|If you have a `&mut T`, and `T` implements `Deref` to some type `U`, compiler will **coerce** `&mut T` into `&U` **transparently**.|

<br>

> Note<br>
> Compiler **will not** *coerce* **immutable** *reference* to **mutable** *reference*.

<br>

# Dot `.` operator
When you use **dot operator** `.`, the compiler will insert as many `*` (dereferencing operations) as necessary to find the appropriate method. As **this happens** **at compile tim**e, there is **no** **runtime cost** of finding the method.

For example, if `x` has type `&i32`, then writing `x.count_ones()` is shorthand for `(*x).count_ones()`, because the `count_ones` method requires an `i32`.

<br>

# Examples
```Rust
fn foo(a: &[i32]) {
    // code
}

fn bar(s: &str) {
    // code
}

let v = vec![1, 2, 3];
// &Vec<i32> coerces into &[i32] because Vec<T> impls Deref<Target=[T]>
foo(&v); 

let s = "Hello world".to_string();
let rc = Rc::new(s);
// Rc<T> impls Deref<Target=T> and &Rc<String> coerces into &String 
// which coerces into &str. This happens as much as needed at compile time.
bar(&rc);
```
