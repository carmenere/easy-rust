# Table of contents
<!-- TOC -->
- [Table of contents](#table-of-contents)
- [Interior mutability](#interior-mutability)
  - [Cell](#cell)
  - [RefCell](#refcell)
  - [Mutex](#mutex)
  - [RwLock](#rwlock)
<!-- TOC -->

<br>

# Interior mutability
In Rust, you need to use `mut` to change a variable. Rust provides some ways to safely change values **inside of a struct** that is itself **immutable**, e.g. **without** the word `mut`.<br>

- `use std::cell`
  - `Cell`
  - `RefCell`

`Cell` and `RefCell` **don’t have** any **guards** to make sure that data isn’t being changed at the same time, so Rust **won’t let** you use them in **multiple threads**.<br>

<br>

Inside the standard library, `std::sync` is for types that are **thread-safe**, meaning they **can be used in multiple threads**:
- `use std::sync`
  - `Mutex`
  - `RwLock`

<br>

## Cell
```rust
use std::cell::Cell;

#[derive(Debug)]
struct Foo {
    name: String,
    flag: Cell<bool>,
}

impl Foo {
    fn disable(&self) {
        self.flag.set(false);
    }
}

fn main() {
    let phone = Foo {
        name: "YY Electronics".to_string(),
        flag: Cell::new(true),
    };

    phone.disable(); // changes.flag to Cell(false)
    println!("{phone:#?}");
}
```

<br>

## RefCell
```rust
use std::cell::RefCell;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
    active: RefCell<bool>,
}

fn main() {
    let usr = User {
        id: 1,
        name: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{:?}", usr.active);

    let mut borrow = usr.active.borrow_mut();
    *borrow = false;
    drop(borrow); // if not to do this any futher call usr.active.borrow_mut() panic
    println!("{:?}", usr.active);
    
    *usr.active.borrow_mut() = true;
    println!("{:?}", usr.active);

    *usr.active.borrow_mut() = true; // NO panic
}
```
**Output**:
```python
RefCell { value: true }
RefCell { value: false }
RefCell { value: true }
```

<br>

But you have to be **careful** with a `RefCell` because **it checks borrow rules at run time**, not compilation time. The **borrow rules**:
- **many immutable** borrows is fine;
- **one mutable** borrow is fine;
- **mutable** and **immutable** borrows **together** - `RefCell` **panics**;

<br>

There are two **ways to be sure** that your **code won’t panic** when using a `RefCell`:
- always **immediately change** the value with `.borrow_mut()` **without assigning** this to a variable
  - if **no** variable holds the **ref** returned by `.borrow_mut()` there is **no** way the code will panic:
```rust
*usr.active.borrow_mut() = true;
```
- use the `.try_borrow_mut()` method instead of `borrow_mut()` if there is a chance of a double borrow, this will return an **error** if the `RefCell` is already
**borrowed**;

<br>

## Mutex
**Mutex** means **mutual exclusion** which means **only one at a time**.<br>
In rust `Mutex` is another way to change values without declaring `mut`.<br>
The `Mutex<T>` is safe because it only lets **one thread change it at a time**. To do this, it uses a method `.lock()`, which returns `Result<MutexGuard<T>>`.<br>

```rust
use std::sync::Mutex;

fn main() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut gurad = my_mutex.lock().unwrap();
    println!("{my_mutex:?}");
    println!("{gurad:?}");
    *gurad = 6;
    println!("{gurad:?}");
}
```
**Output**:
```python
Mutex { data: <locked>, poisoned: false, .. }
5
6
```

<br>

A `Mutex` is **unlocked** when the `MutexGuard` **goes out of scope** or when it is **explicitly dropped**.<br>
One way to do this is to put the `MutexGuard` into **its own scope**:
```rust
use std::sync::Mutex;

fn main() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    {
        let mut gurad = my_mutex.lock().unwrap();
        *gurad = 6;
    }
}
```

<br>

The other way to do this is to use `drop()`:
```rust
use std::sync::Mutex;

fn main() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut gurad = my_mutex.lock().unwrap();
    *gurad = 6;
    drop(gurad);
}
```

<br>

**To to be sure** that your **code will never has a deadlock** always **immediately change** the value with `*my_mutex.lock().unwrap() = ... ;`.<br>
When you type `*my_mutex.lock().unwrap() = ... ;`, you **never create a variable that holds the lock**, so you don’t need to call `drop()`:
```rust
use std::sync::Mutex;

fn main() {
    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex);
    for _ in 0..100 {
        *my_mutex.lock().unwrap() += 1;
    }
    println!("{:?}", my_mutex);
}
```
**Output**:
```python
Mutex { data: 5, poisoned: false, .. }
Mutex { data: 105, poisoned: false, .. }
```

<br>

You have to be **careful** with a `Mutex` because if another variable tries to `.lock()` it in the same thread, it will **wait forever**. This is known as a **deadlock**.<br>

<br>

**Example of deadlock**:
```rust
use std::sync::Mutex;

fn main() {
    let my_mutex: Mutex<i32> = Mutex::new(5);
    let mut gurad = my_mutex.lock().unwrap();
    *gurad = 6;
    let mut gurad = my_mutex.lock(); // deadlock here!
}
```

<br>

Instead of `.lock()`, you can use a method called `.try_lock()`. This method will **try once**, and if it *doesn’t get the lock*, it will **give up**. You can use `if let` or `match`
for this:
```rust
use std::sync::Mutex;
fn main() {
    let my_mutex = Mutex::new(5);
    let mut guard1 = my_mutex.lock().unwrap();
    let mut gurad2 = my_mutex.try_lock();
    if let Ok(value) = gurad2 {
        println!("The MutexGuard has: {value}")
    } else {
        println!("Didn't get the lock")
    }
}
```
**Output**:
```python
Didn't get the lock
```

<br>

## RwLock
`RwLock` stands for **read–write lock**. It is *similar to* a `RefCell` in the way it is used: you can get **mutable** or **immutable** references to the value inside:
- you use `.write().unwrap()` instead of `.lock().unwrap()` to **change it**.<br>
- you use `.read().unwrap()` instead of `.lock().unwrap()` to **get read access**.<br>

<br>

But `RwLock` is also *similar to* `Mutex` in that the program will **deadlock** instead of panicking if you try to use `.write()` when you can’t get access:
- **many** variables with `.read()` access is okay;
- **one** variable with `.write()` access is okay;
- **one** variable with `.read()` access and **one** variable with `.write()` access **together** - **deadlock**;

<br>

**Example of deadlock**:
```rust
use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();
    println!("{read1:?}, {read2:?}");
    let write1 = my_rwlock.write().unwrap(); // deadlock!
}
```

<br>

The `RwLock` has the same `.try_` methods to **ensure that you’ll never have a deadlock**: `.try_read()` and `.try_write()`:
```rust
use std::sync::RwLock;

fn main() {
    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };
}
```
**Output**:
```python
Couldn't get write access, sorry!
```

<br>