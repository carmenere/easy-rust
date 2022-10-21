# Traits
**Trait** is **functionality** that you can call **on** particular **structure**.<br>

**Trait** is *similar* to **interface** or **abstract class**.<br>

Inside trait there can be *both* **signatures** (like interface or abstract class) and **default implementations**.<br>

**Static method** – method that **doesn't** takes ``&self``.<br>
**Bound method** – method that takes ``&self`` *as first parameter*.<br>

**Constructors** are **static methods** of trait that have name ``new`` and return ``Self``.<br>

<br>

## ``impl ... for ...``
To implement trait ``SomeTrait`` for particular type `SomeStruct` there is following syntax:
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
Rust **cannot** return trait, i.e. return type cannot have an unboxed trait object. Reason for that is the Rust memory guaranties. Rust needs to know the size of the returned value at compile time.

Solution is to return ``Box<dyn SomeTrait>``.

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
When using *generics* **control over type var** is **on calling side** and **compiler decides** which concrete type will be used instead ``T``.<br>

**Associated type** is **type** that is **controlled** inside **implemetation** of **trait**.<br>

If trait has **associated type** it is called **trait with an associated type**.<br>

Version of trait ``Iterator`` with an **associated type**:
```Rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The **associated type** here is named ``Item``.<br>
To **refer** to **associated type** there is syntax: ``Self::Item``.

<br>

To set concrete type for ``Item`` **assignment** is used, example:
```Rust
impl Iterator for MyType {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip—
    }
}
```

<br>

# Generics vs. Associated types
What is the difference between using **generics** and **associated types**?<br>

Version of trait ``Iterator`` with **type var**:
```Rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Generics vs. Associated types:
- When using **generics** we must annotate the types in each implementation.<br>
- We can have **multiple implementations** of ``Iterator`` for ``MyType``, in other words, when a trait has a **generic parameter**, it can be implemented for a type **multiple times**, changing the concrete types of the **generic type parameters** each time.<br>
- With **associated types**, we don’t need to annotate types because we **can’t** implement a trait on a type **multiple times**.<br>

<br>

# Blanket implementations
Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in the Rust standard library. 

For example, the standard library implements the ``ToString`` trait on **any type** **that implements** the ``Display`` trait:
```Rust
impl<T> ToString for T 
where
    T: Display + ?Sized
```

Because the standard library has this **blanket implementations**, we can call the ``to_string()`` of ``ToString`` trait on **any type** that **implements** the ``Display`` trait.


