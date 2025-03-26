# Table of contents
<!-- TOC -->
* [Table of contents](#table-of-contents)
* [URLs](#urls)
* [Declarations](#declarations)
  * [`Deref`](#deref)
  * [`DerefMut`](#derefmut)
* [In a nutshell](#in-a-nutshell)
  * [Example](#example)
* [Deref coercion](#deref-coercion)
  * [Deref coercion rules](#deref-coercion-rules)
  * [Example](#example-1)
* [Dot `.` operator](#dot--operator)
  * [Example](#example-2)
* [Implementations in `std`](#implementations-in-std)
* [Blanket implementations](#blanket-implementations)
  * [`impl Deref<Target=T> for &T`](#impl-dereftargett-for-t)
  * [`impl Deref<Target=T> for &mut T`](#impl-dereftargett-for-mut-t)
<!-- TOC -->

<br>

# URLs
|Trait|URL|
|:----|:------------|
|`Deref`|[std::ops::Deref](https://doc.rust-lang.org/std/ops/trait.Deref.html)|
|`DerefMut`|[std::ops::DerefMut](https://doc.rust-lang.org/std/ops/trait.DerefMut.html)|

<br>

# Declarations
## `Deref`
```rust
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
```

<br>

## `DerefMut`
```rust
pub trait DerefMut: Deref {
    fn deref_mut(&mut self) -> &mut Self::Target;
}
```

<br>

# In a nutshell
There are **2 types** of reference:
1. **Regular reference** is an ordinary `&` reference.
2. **Smart pointer** is any type that implements `Deref` and `DerefMut` traits.

<br>

Without `Deref` trait the compiler can only dereference **regular references**.<br>
Methods `deref()` and `deref_mut()` return a reference to the value we want to access with the `*` operator.<br>

<br>

In expression `*v`:
  - if `v` is a value of type `T` and `T` implements `Deref<Target=U>`:
    - Rust **substitutes** the `v` with a call to the `deref()`;
    - then Rust applies `*` to the value returned from `deref()`: `*(v.deref())`;
  - if `v` is a value of type `T` and `T` implements `DerefMut<Target=U>`:
    - Rust **substitutes** the `v` with a call to the `deref_mut()`;
    - then Rust applies `*` to the value returned from `deref_mut()`: `*(v.deref_mut())`;
  - if `v` is a value of type `T` and `T` doesn't implemet `Deref` and `DerefMut`:
    - Rust applies `*` to the `v` **directly**;

<br>

## Example
```Rust
use std::ops::Deref;

struct MyBox<T> {
    value: T
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let x = MyBox { value: 'a' };
    assert_eq!('a', *x);
}
```

<br>

# Deref coercion
**Deref coercion** is **implicit** type conversion.<br>
Rust performs **deref coercion** every time we pass a reference to a function or method that **doesn't** match the parameter type.<br>
But, Rust **doesn't** *coerce* **immutable** *reference* to **mutable** *reference*.<br>

<br>

## Deref coercion rules
1. Compiler **doesn't** *coerce* **immutable** *reference* to **mutable** *reference*.
2. If `T: Deref<Target=U>`, then
- values of type `&T` are coerced to values of type `&U`;
- values of type `&mut T` are coerced to values of type `&U`;
3. If `T: DerefMut<Target=U>`, then
- values of type `&mut T` are coerced to values of type `&mut U`;

<br>

## Example
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
- `&MyBox<String>` coerced to `&String` because `MyBox<T>` implements `Deref<Target=T>` such that it returns `&T`;
- `&String` coerced to `&str` because `String` implements `Deref<Target=str>`;

<br>

# Dot `.` operator
When you use **dot operator** `.`, the compiler inserts **at compile time** as many `*` as necessary to find the appropriate method. So, there is **no runtime overhead** of finding the method.

In other words,
- `T: Deref<Target=U>` means that when we can call on the value of `T` type methods of `U` type which take `&self`;
- `T: DerefMut<Target=U>` means that when we can call on the value of `T` type methods of `U` type which take `&mut self`;

<br>

## Example
If `x` has type `&i32`, then writing `x.count_ones()` is shorthand for `(*x).count_ones()`, because the `count_ones` method requires an `i32`.

<br>

# Implementations in `std`
- `Vec<T>`:
  - `Vec<T>` implements `Deref<Target=[T]>`;
- `String`:
  - `String` implements `Deref<Target=str>`;
- `Rc<T>`:
  - `Rc<T>` implements `Deref<Target=T>`
- `Arc<T>`:
  - `Arc<T>` implements `Deref<Target=T>`
- `Box<T>`:
  - `Box<T>` implements `Deref<Target=T>`

<br>

# Blanket implementations
## `impl Deref<Target=T> for &T`
```rust
impl<T: ?Sized> Deref for &T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}
```

<br>

## `impl Deref<Target=T> for &mut T`
```rust
impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}
```
