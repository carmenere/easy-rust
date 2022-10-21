# Trait Object
**Trait object** is **DST**.<br>
**Trait object** is a **reference** to ``dyn SomeTrait``.<br>

<br>

``dyn SomeTrait`` **reference** contains **2 pointers**: 
- a pointer to an **instance** **of** a **type** ``T`` that implements ``SomeTrait``; 
- a pointer to an ``T``'s **vtable**. 

<br>

**Trait object** *declaration* examples:
- ``&dyn SomeTrait``
- ``Box<dyn SomeTrait>``

<br>

Calling a method on a **trait object** uses **dynamic dispatch**.<br>
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.<br>

**vtable** (**virtual method table**) **contains** for each method of ``SomeTrait`` **pointer to T's implementation** (i.e. a function pointer).

<br>

## Trait Object Lifetime Bounds
Since a **trait object** can contain references, the **lifetimes** of those references need to be expressed as part of the trait object.<br>
This **lifetime** is written as ``Trait + 'a``.

<br>

# Generics vs. Trait objects
**Generics** have restriction – **monomorphized version** (**specialized versions**) of a polymorphic entity can use **only particular type**.<br>
For example, monomorphized version of ``Vec<T>`` can store elements of only particular type, e.g., ``i32`` or ``String``.<br>

There are 2 approaches to eliminate this restriction:
- use **enum variants** for different types;
- use **trait objects**.

<br>

## Generic approach
```Rust
pub trait Animal {
    fn eat(&self);
}

struct Omnivore;
struct Carnivore;
struct Herbivore;

impl Animal for Omnivore {
    fn eat(&self) {
        println!("I'm omnivore.")
    }
}

impl Animal for Carnivore {
    fn eat(&self) {
        println!("I'm carnivore.")
    }
}

impl Animal for Herbivore {
    fn eat(&self) {
        println!("I'm herbivore.")
    }
}

struct Zoo<T: Animal> {
    animals: Vec<T>
}

impl<T> Zoo<T> 
where 
    T: Animal
{
    pub fn meal(&self) {
        for animal in self.animals.iter() {
            animal.eat()
        }
    }
}

fn main() {
    let mut zoo = Zoo {
        animals: Vec::with_capacity(8)
    };

    zoo.animals.push(Omnivore);
    zoo.animals.push(Carnivore);
    zoo.animals.push(Herbivore);

    zoo.meal();
}
```

**Output**:
```bash
cargo run
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
error[E0308]: mismatched types
    --> src/main.rs:48:22
     |
48   |     zoo.animals.push(Carnivore);
     |                 ---- ^^^^^^^^^ expected struct `Omnivore`, found struct `Carnivore`
     |                 |
     |                 arguments to this function are incorrect
     |
note: associated function defined here
    --> /Users/an.romanov/.rustup/toolchains/1.64-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1763:12
     |
1763 |     pub fn push(&mut self, value: T) {
     |            ^^^^

error[E0308]: mismatched types
    --> src/main.rs:49:22
     |
49   |     zoo.animals.push(Herbivore);
     |                 ---- ^^^^^^^^^ expected struct `Omnivore`, found struct `Herbivore`
     |                 |
     |                 arguments to this function are incorrect
     |
note: associated function defined here
    --> /Users/an.romanov/.rustup/toolchains/1.64-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:1763:12
     |
1763 |     pub fn push(&mut self, value: T) {
     |            ^^^^

For more information about this error, try `rustc --explain E0308`.
error: could not compile `ololo` due to 2 previous errors
```

<br>

## Trait object approach
```Rust
pub trait Animal {
    fn eat(&self);
}

struct Omnivore;
struct Carnivore;
struct Herbivore;

impl Animal for Omnivore {
    fn eat(&self) {
        println!("I'm omnivore.")
    }
}

impl Animal for Carnivore {
    fn eat(&self) {
        println!("I'm carnivore.")
    }
}

impl Animal for Herbivore {
    fn eat(&self) {
        println!("I'm herbivore.")
    }
}

struct PolymorphicZoo {
    animals: Vec<Box<dyn Animal>>
}

impl PolymorphicZoo {
    fn meal(&self) {
        for animal in self.animals.iter() {
            animal.eat()
        }
    }
}

fn main() {
    let mut pzoo = PolymorphicZoo {
        animals: Vec::with_capacity(8)
    };

    pzoo.animals.push(Box::new(Omnivore));
    pzoo.animals.push(Box::new(Carnivore));
    pzoo.animals.push(Box::new(Herbivore));

    pzoo.meal();
}
```

**Output**:
```bash
cargo run
   Compiling ololo v0.1.0 (/Users/an.romanov/Projects/play-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/ololo`
I'm omnivore.
I'm carnivore.
I'm herbivore.
```

