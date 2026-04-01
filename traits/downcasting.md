# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Downcasting to a concrete type](#downcasting-to-a-concrete-type)
  - [Example: downcast `dyn Animal`](#example-downcast-dyn-animal)
  - [Example: downcast `dyn Error`](#example-downcast-dyn-error)
<!-- TOC -->

<br>

# Downcasting to a concrete type
You can **downcast** a **trait object** back to a **concrete type** as long as you **know** what **concrete type** it might be. 

**Downcasting** is the process of **converting** a *trait object* (like `Box<dyn Trait>` or `&dyn Trait`) back into its **concrete type** at runtime.<br>
**Note**, you **can only try downcasting to one type at a time**.<br>

The primary mechanism for safe downcasting in the standard library is the `std::any::Any` trait.<br>

Restrictions:
- the concrete types must have a `'static` lifetime bound (meaning they **don't contain** *any non-static references*);

<br>

## Example: downcast `dyn Animal`
```rust
use std::{any::Any, fmt::Debug};

trait Animal {
    fn my_name(&self);
    fn as_ref_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
    fn rename(&mut self, name: &str);
}

#[derive(Debug)]
struct Cat {
    name: String,
}

#[derive(Debug)]
struct Dog {
    name: String,
}

impl Animal for Cat {
    fn my_name(&self) {
        println!("{:?}", self);
    }
    
    fn as_ref_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Animal for Dog {
    fn my_name(&self) {
        println!("{:?}", self);
    }
    
    fn as_ref_any(&self) -> &dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
    
    fn rename(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

fn main() {
    let mut speakers: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat {name: "Barsik".to_string()}),
        Box::new(Dog{name: "Beethoven".to_string()}),
        Box::new(Cat{name: "Felix".to_string()}),
    ];

    for speaker in speakers.iter_mut() {
        if let Some(cat) = speaker.as_ref_any().downcast_ref::<Cat>() {
            cat.my_name();
        } else if let Some(dog) = speaker.as_mut_any().downcast_mut::<Dog>() {
            dog.my_name();
            dog.rename("Leo");
            dog.my_name();
        } else {
            println!("  Couldn't downcast to a known type.");
        }
    }
}
```

**Output**:
```rust
Cat { name: "Barsik" }
Dog { name: "Beethoven" }
Dog { name: "Leo" }
Cat { name: "Felix" }
```

<br>

## Example: downcast `dyn Error`
```rust
use std::{error::Error, fmt::Debug};

struct Foo;

#[derive(Debug)]
enum MyErr {
    A,
    B,
}

impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "My Error")
    }
}

impl Error for MyErr {}

fn main() {
    let err: Box<dyn Error> = Box::new(MyErr::A);
    let r = err.downcast_ref::<MyErr>();
    println!("{:?}", r);

    // let r = err.downcast_ref::<Foo>(); // ❌ ERROR: the trait `std::error::Error` is not implemented for `Foo`
    //                                                 required by signature of downcast_ref: `T: Error + 'static`
}
```

**Output**:
```rust
Some(A)
```

<br>