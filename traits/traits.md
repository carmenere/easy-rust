# Table of contents
- [Table of contents](#table-of-contents)
- [Traits](#traits)
  - [`impl ... for ...`](#impl--for-)
      - [Example](#example)
  - [Returning traits](#returning-traits)
    - [Example](#example-1)
      - [Incorrect code](#incorrect-code)
      - [Working version](#working-version)
- [Associated Types](#associated-types)
- [Default type parameters](#default-type-parameters)
  - [Operator Overloading](#operator-overloading)
- [Generics vs. Associated types](#generics-vs-associated-types)
- [Blanket implementations](#blanket-implementations)
  - [`T`, `&T` and `&mut T`](#t-t-and-mut-t)
- [Trait casting](#trait-casting)
      - [Example](#example-2)
- [Supertraits](#supertraits)

<br>

# Traits
**Trait** is **functionality** that you can call **on** particular **structure**.<br>

**Trait** is *similar* to **interface** or **abstract class**.<br>

Inside trait there can be *both* **signatures** (like interface or abstract class) and **default implementations**.<br>

**Static method** – method that **doesn't** takes `&self`.<br>
**Bound method** – method that takes `&self` *as first parameter*.<br>

**Constructors** are **static methods** of trait that have name `new` and return `Self`.<br>

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

# Associated Types
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

# Default type parameters
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

## Operator Overloading
A great example of a situation where this **associated types** are useful is **operator overloading**.<br>

Rust doesn’t allow you to create your own operators or overload arbitrary operators.<br>
But you can **overload** the operations and corresponding **traits** listed **in** `std::ops` by implementing the traits associated with the operator.<br>

More xamples [here](https://github.com/carmenere/easy-rust/blob/main/examples/traits/overloading.md) and [here](https://github.com/carmenere/easy-rust/blob/main/examples/traits/overloading-generic-version-of-Point.md).
<br>

# Generics vs. Associated types
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
Types `&T` and `&mut T` are **disjoint sets**. Type `T` is a **superset** of both `&T` and `&mut T`.<br>
So, the compiler **doesn't allow** us to define an implementation of `MyTrait` for `&T` and `&mut T` since it would conflict with the implementation of `MyTrait` for `T` which **already includes** all of `&T` and `&mut T`:
```rust
trait MyTrait {}

impl<T> MyTrait for T {}
impl<T> MyTrait for &T {} // ❌
impl<T> MyTrait for &mut T {} // ❌
```

<br>

**But** when **trait has generic** there are will be different implementations for different types.<br>

Consider example:
```rust
impl<T> Borrow<T> for T
impl<T> Borrow<T> for &T
```

From compiler's point of view both implementations `Borrow<T> for &T` and `Borrow<T> for T` are **not** overlapped.<br>
Indeed, `Borrow<T> for &T` and `Borrow<T> for T` give 4 **disjoint** implementations for `T=u32` and `T=&u32`:
1. `Borrow<u32> for u32` (`impl<T> Borrow<T> for T` for `T=u32`);
2. `Borrow<&u32> for &u32` (by `impl<T> Borrow<T> for T` for `T=&u32`);
3. `Borrow<u32> for &u32` (by `impl<T> Borrow<T> for &T` for `T=u32`);
4. `Borrow<&u32> for &&u32` (by `impl<T> Borrow<T> for &T` for `T=&u32`);

<br>

# Trait casting
Rust allows to implement **multiple traits** with the **same** methods **names** on **one type**.<br>
To resolve such methods with the same names there is **fully qualified syntax**: `<SomeType as SomeTrait>::some_method(arg1, arg2, ... )`.<br>
These means we want to call `some_method` of trait `SomeTrait` as implemented for `SomeType`.<br>
Also it is possible to call `SomeTrait::some_method(&insatance)`.
By default, the compiler use method of struct itself.<br>

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
