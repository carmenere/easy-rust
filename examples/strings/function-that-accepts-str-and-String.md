# Example: function that accepts `&str` and `String`
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

## Variant 1: using trait `Into`
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

## Variant 2: using trait `AsRef`
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
        println!("{}", name.as_ref());
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
