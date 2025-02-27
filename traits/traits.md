# Table of contents
- [Table of contents](#table-of-contents)
- [Traits](#traits)
  - [`impl ... for ...`](#impl--for-)
      - [Example](#example)
  - [Returning traits](#returning-traits)
    - [Example](#example-1)
      - [Incorrect code](#incorrect-code)
      - [Working version](#working-version)
- [Associated items](#associated-items)
  - [Associated functions and methods](#associated-functions-and-methods)
  - [Associated types](#associated-types)
  - [GATs](#gats)
    - [Default type parameters](#default-type-parameters)
    - [Operator Overloading](#operator-overloading)
    - [Generics vs. Associated types](#generics-vs-associated-types)
  - [Associated constants](#associated-constants)
- [Blanket implementations](#blanket-implementations)
  - [`T`, `&T` and `&mut T`](#t-t-and-mut-t)
- [Specializations](#specializations)
- [Fully qualified path](#fully-qualified-path)
  - [Example](#example-2)
      - [Example](#example-3)
- [Supertraits](#supertraits)

<br>

# Traits
**Trait** is **functionality** that you can call **on** particular **structure**.<br>

**Trait** is *similar* to **interface** or **abstract class**.<br>

Inside trait there can be *both* **signatures** (**declarations**) and **default implementations**.<br>

<br>

## `impl ... for ...`
To implement trait `SomeTrait` for particular type `SomeStruct` there is following syntax:
```Rust
impl SomeTrait for SomeStruct {
    ...
}
```
<br>

#### Example
```Rust
struct RustDev {
    awesome: bool
}

struct JavaDev {
    awesome: bool
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { println!("Hello world!") }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome: awesome }
    }

    fn language(&self) -> &str {
        // unimplemented!()
         "Rust"
    }

    fn say_hello(&self) {
        // todo!()
        println!("println!(\"Hello world!\");");
    }
}

fn main() {
    // Explicit instantiation
    // let r = RustDev { awesome: true};

    // Instantiation through type constructor
    let r = RustDev::new(true);

    println!("{}", r.language());
    r.say_hello();
}
```

<br>

## Returning traits
Rust **cannot** return trait, i.e. return type cannot have an unboxed trait object. Reason for that is the Rust memory guaranties. Rust needs to know the size of the returned value at compile time.<br>

**Ways to return trait**:
- `Box<dyn SomeTrait>` or `&Trait` for **dynamic dispatch**;
- `impl SomeTrait` or `T: SomeTrait` for **static dispatch**; `impl SomeTrait` is more generic than `T: SomeTrait`, because it means **any type** that implement `SomeTrait`, i.e., the compiler replaces every `impl Trait` with a **concrete type** *at compile time*.

<br>

### Example
#### Incorrect code
```Rust
struct Dog {}
struct Cat {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

fn get_animal(rand_number: f64) -> Animal {
    if rand_number < 1.0 {
        Dog {}
    }
    else {
        Cat {}
    }
}
```

<br>

#### Working version
```Rust
fn get_animal(rand_number: f64) -> Box<dyn Animal> {
    if rand_number < 1.0 {
        Box::new(Dog {})
    }
    else {
        Box::new(Cat {})
    }
}
```

<br>

Vec::<u8>::with_capacity(1024);

# Associated items
**Associated items** are the **items declared** in **traits** or defined in **implementations**.<br>
Kinds **associated items**:
- **associated functions**;
- **methods**;
- **associated types**;
- **associated constants**;

<br>

## Associated functions and methods
**Associated functions** are functions associated with a type. An example of a common **associated function** is a `new` function that returns a value of the type the associated function is associated with. Also `new` is called **constructor**.<br>
**Methods** are *associated functions* whose first parameter is named `self`. *Methods* are called on a particular instance of a type.<br>
Note, then `&self` is a short form for `self: &Self`. For example in `impl X for &T`, `Self` represents `&T` and `&self` means `self: &&T`.<br>

<br>

## Associated types
When using *generics* **control over type var** is **on calling side** and **compiler decides** which concrete type will be used instead `T`.<br>

**Associated type** is **type** that is **controlled** inside **implemetation** of **trait**.<br>

If trait has **associated type** it is called **trait with an associated type**.<br>

Version of trait `Iterator` with an **associated type**:
```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The **associated type** here is named `Item`.<br>
To **refer** to **associated type** there is syntax: `Self::Item`.

<br>

To set concrete type for `Item` **assignment** is used, example:
```Rust
impl Iterator for MyType {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip—
    }
}
```

<br>

## GATs
**Generic associated types** (**GATs**) are *associated types* that includes **generic parameters**.<br>

<br>

### Default type parameters
Consider following syntax:
```Rust
use std::ops::Add;

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}


impl Add<Merers> for Millimeters {

}
```

<br>

The syntax `Rhs=Self` is called **default type parameter**.<br>
The `Rhs` (*right hand side*) **generic type parameter** defines the type of the `rhs` parameter in the `add()` method.<br>

Notes:
- If we don’t specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will **default** to `Self`, which will be the type we’re implementing `Add` on. 
- When we implement `Add` for `Point`, we use the **default** for `Rhs` because we wante to add two `Point` instances.

<br>

### Operator Overloading
A great example of a situation where this **associated types** are useful is **operator overloading**.<br>

But you can **overload** the operations and corresponding **traits** listed **in** `std::ops` by implementing the traits associated with the operator.<br>

More examples [here](../examples/traits/overloading.md) and [here](../examples/traits/overloading-generic-version-of-Point.md).<br>

<br>

### Generics vs. Associated types
What is the difference between using **generics** and **associated types**?<br>

Example of trait `Iterator` with **generic**:
```Rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

<br>

**Generics vs. Associated types**:
- When a trait has a **generic parameter**, it can be implemented for a type **multiple times**, changing the **concrete types** of the **generic type parameters** each time. In other words, we can have **multiple implementations** of *some trait* for *some type*.<br>
- Also, with **generics** we must annotate the **type parameters** in **each implementation**.<br>
- With **associated types** we **can’t** implement a trait on a type **multiple times**, so we **don’t** need to annotate types.<br>

<br>

## Associated constants

<br>

# Blanket implementations
Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in the Rust standard library. 

For example, the standard library implements the `ToString` trait on **any type that implements** the `Display` trait:
```Rust
impl<T> ToString for T 
where
    T: Display + ?Sized
```

Because the standard library has this **blanket implementations**, we can call the `to_string()` of `ToString` trait on **any type** that **implements** the `Display` trait.

<br>

## `T`, `&T` and `&mut T`
Type `T` is **type variable** that runs values `U`, `&U` and `&mut U`, where `U` runs through all **owned types**.<br>
From compiler point of view `impl<T> MyTrait for T {}` means following implementations (`U` runs through all **owned types**):
```rust
T=U: impl<U> MyTrait for U {}
T=&U: impl<U> MyTrait for &U {}
T=&mut U: impl<U> MyTrait for &mut U {}
```

<br>

From compiler point of view both implementations `impl<T> MyTrait for T {}` and `impl<T> MyTrait for &T {}` are **overlapped**, because first implementation **already includes** `&U`.<br>

<br>

**Consider example**:
```rust
trait MyTrait {}

impl<T> MyTrait for T {}
impl<T> MyTrait for &T {} // ❌
impl<T> MyTrait for &mut T {} // ❌

fn main (){}
```

<br>

**Output**:
```bash
error[E0119]: conflicting implementations of trait `MyTrait` for type `&_`
 --> src/main.rs:4:1
  |
3 | impl<T> MyTrait for T {}
  | --------------------- first implementation here
4 | impl<T> MyTrait for &T {}
  | ^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&_`

error[E0119]: conflicting implementations of trait `MyTrait` for type `&mut _`
 --> src/main.rs:5:1
  |
3 | impl<T> MyTrait for T {}
  | --------------------- first implementation here
4 | impl<T> MyTrait for &T {}
5 | impl<T> MyTrait for &mut T {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `&mut _`

For more information about this error, try `rustc --explain E0119`.
error: could not compile `foo` (bin "foo") due to 2 previous errors
```

<br>

**But** when **trait has generic** there are will be different implementations for different types.<br>

Consider example:
```rust
impl<T> Borrow<T> for T
impl<T> Borrow<T> for &T
```

From compiler's point of view both implementations `Borrow<T> for T` and `Borrow<T> for &T` are **not** overlapped.<br>

From compiler point of view `Borrow<T> for T` means following implementations (`U` runs through all **owned types**):
```rust
T=U: impl<U> Borrow<U> for U {}
T=&U: impl<U> Borrow<&U> for &U {}
T=&mut U: impl<U> Borrow<&mut U> for &mut U {}
```

<br>

From compiler point of view `Borrow<T> for &T` means only **one** following implementation (`U` runs through all **owned types**):
```rust
T=U: impl<U> Borrow<U> for &U {}
```

<br>

# Specializations
[**RFC**](https://rust-lang.github.io/rfcs/1210-impl-specialization.html).<br>

**Specialization** will **permit overlapping** in case of the one implemetation is **clearly more specific** than the other. The **more specific** `impl` block is used in a case of overlap.<br>

<br>

# Fully qualified path
Rust allows to implement **multiple traits** with the **same** methods **names**. **Fully qualified path** is used to resolve such methods.<br>

<br>

**Fully qualified path** has following syntax:
```rust
<Type as Trait>::function(arg1, arg2, ... )
```
These means we want to call `function` of trait `Trait` as implemented for `Type`.<br>

<br>

For **method** (when function has `self` argument) it is possible to call:
```rust
Trait::method(&insatance_of_Type)
```

<br>

Consider expression `x.f()` where `x` is **instance** of `X`. There are possible 2 cases:
1. Type `X` **doesn't** implement method `f(&self)` in its own `impl` block. Also type `X` implements several traits that share the same name `f` for method. Then `x.f()` causes to error: `error[E0034]: multiple applicable items in scope`.<br>
2. Type `X` implements method `f(&self)` in its own `impl` block. Also type `X` implements several traits that share the same name `f` for method. Then in `x.f()` compiler by default will use **own implementation** of method `f(&self)`, i.e. `f(&self)` from `impl` block of `X`.<br>

<br>

## Example
Consider types `X` and `Y`. Both types have own `impl` block and both implement **method** `f(&self)`.<br>
Consider traits `T1`, `T2`. All have **associated function** `f()`.<br>
Consider traits `T3`. It has **method** `f(&self)`.<br>
```rust
struct X;
struct Y;

impl X {
    fn f(&self) { println!("X"); }
}

impl Y {
    fn f(&self) { println!("Y"); }
}

trait T1 {
    fn f() { println!("T1 f"); }
}

trait T2 {
    fn f() { println!("T2 f"); }
}

trait T3 {
    fn f(&self);
}

impl T1 for X {}
impl T2 for X {}
impl T3 for X {
    fn f(&self) {
        println!("X, T3, f()");
    }
}

impl T1 for Y {}
impl T2 for Y {}
impl T3 for Y {
    fn f(&self) {
        println!("Y, T3, f()");
    }
}

fn main() {
    let x = X;
    let y = Y;
    x.f();
    y.f();
    <X as T1>::f();
    <Y as T2>::f();
    //T2::f(); // -> error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
    T3::f(&x); // compiler infers type X from variable x and calls f() from X's implementation of T3
    T3::f(&y); // compiler infers type Y from variable y and calls f() from Y's implementation of T3
}
```



<br>

#### Example
```Rust
trait Foo {
    fn abc(&self);
}

trait Bar {
    fn abc(&self);
}

struct FooBar;

impl FooBar {
    fn abc(&self) {
        println!("FooBar, method abc()");
    }
}

impl Foo for FooBar {
    fn abc(&self) {
        println!("FooBar, impl Foo, method abc()");
    }
}

impl Bar for FooBar {
    fn abc(&self) {
        println!("FooBar, impl Bar, method abc()");
    }
}

fn main () {
    let fb: FooBar = FooBar;
    fb.abc();
    <FooBar as Foo>::abc(&fb);
    <FooBar as Bar>::abc(&fb);
}
```

<br>

**Output**:
```bash
FooBar, method abc()
FooBar, impl Foo, method abc()
FooBar, impl Bar, method abc()
```

<br>

# Supertraits
```Rust
trait MyTrait: ToString {

}
```

<br>

The syntax above means `MyTrait` depends on `ToString`, in other words, `ToString` must be implemented on the type on which trait `MyTrait` is implemented.<br>
So, we can use methods of trait `ToString` inside `MyTrait`.<br>

Here, `ToString` is a **supertrait** for `MyTrait`.
