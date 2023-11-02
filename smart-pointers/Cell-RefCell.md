# Interior mutability
**Interior mutability** is **property of type** that allows to **wrap** something that’s **mutable** in a structure that is **immutable**.<br>
In other words, **interior mutability** allow us to **bypass Rust's borrow rules**.<br>

There are 2 types that implement **interior mutability**: `Cell<T>` and `RefCell<T>`.<br>
`Cell` and `RefCell` data types allow us to change their values, even if they are **not** stored in a **mutable** variable.

`Cell<T>` is a type that provides **zero-cost interior mutability**, but only for `Copy` types.<br>
`RefCell<T>` also provides **interior mutability**, but **isn’t** restricted to `Copy` types.<br>
`RefCell<T>` type performs **borrow checking at runtime**.<br>

<br>

The following example will **not** compile:<br>
```Rust
#[derive(Debug)]
struct Foo<'a> {
    v: &'a mut i32,
}
fn main() {
    let mut x = 1;
    let foo = Foo { v: &mut x };
    x += 1;
    println!("The value of foo is {:?}.", foo);
    println!("The value of x is {:?}.", x);
}
```

<br>

But we can **bypass** compile errors with **interior mutability** as follows:<br>
```Rust
use std::cell::RefCell;

#[derive(Debug)]
struct Foo<'a> {
    v: &'a RefCell<i32>,
}

fn add_and_print(foo: &Foo) {
    *foo.v.borrow_mut() += 1; // here we modify value inside my_ref_cell
    dbg!(foo);
}

fn main() {
    let my_ref_cell = RefCell::new(1);
    let foo = Foo { v: &my_ref_cell };
    add_and_print(&foo);
    *my_ref_cell.borrow_mut() += 1; // here we modify value inside my_ref_cell
    dbg!(foo);
    dbg!(my_ref_cell);
}
```

<br>

## RefCell vs. Cell
Differences between `RefCell` and `Cell`:
1. Types wrapped in `Cell` **must implement** the `Copy` trait while those in `RefCell` **needn't**.<br>This makes sense. When calling `get` on a `Cell`, you are getting a **copy** of the wrapped data, whereas the methods associated with `RefCell` are `borrow` and `borrow_mut`, which return references to the underlying data.
2. `Cell` **never panics**. But references to `RefCell` internal value are checked **at runtime**, which can cause program to **panic**.

<br>

# `Cell<T>`
`T` must be of **Copy type**.	

Some usefull methods of `Cell<T>`:
- `Cell::new(somevalue)` creates new `Cell`, **moving** value `somevalue` into it;
- `.get()` returns **copy** of the value inside `Cell`;
- `.set(somevalue)` **stores** the given value `somevalue` in the `Cell`; **dropping** the **previous** **value**.

<br>

### Example
```Rust
use std::cell::Cell;

struct Point {
    x: i32,
    y: i32,
    counter: Cell<i32>,
}

impl Point {
    pub fn print(&self) {
        println!("{{x: {}; y:{}}}", self.x, self.y);
    }
    pub fn print_counter(&self) {
        println!("counter:{}", self.counter.get());
    }
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self{
        Point { 
            x: match x {
                Some(v) => v,
                None => Point::X,
            }, 
            y: match y {
                Some(v) => v,
                None => Point::Y,
            },
            counter: Cell::new(0),
        }
    }
    pub fn increment(&self) {
        self.counter.set(self.counter.get() + 1)
    }
}

impl Point {
    const X: i32 = 0;
    const Y: i32 = 0;
}

fn main() {
    let p = Point::new(Some(10), Some(20));
    p.increment();
    p.increment();
    p.print_counter();
}
```

**Output**:
```bash
counter:2                                                                                                                                                                   
```

<br>

# `RefCell<T>`
Some usefull methods of `RefCell<T>` methods:
- `RefCell::new(somevalue)` creates new `RefCell`, moving value `somevalue` into it;
- `.borrow()` returns **shared reference** to value inside `RefCell`; panics if the value is **already mutably** *borrowed*;
- `.borrow_mut()` returns **mutable reference** to value in `RefCell`; panics if the value is **already** *borrowed*;
- `.try_borrow()` returns `Result`, returns `Err` if the value is already **mutably** *borrowed*;
- `.try_borrow_mut()` returns `Result`, returns `Err` if the value is **already** *borrowed*.

<br>

### Example
```Rust
use std::cell::RefCell;

struct Point {
    x: i32,
    y: i32,
    s: RefCell<String>,
}

impl Point {
    pub fn print(&self) {
        println!("{{x: {}; y:{}}}", self.x, self.y);
    }
    pub fn print_s(&self) {
        println!("string: {}", self.s.borrow());
    }
    pub fn new(x: Option<i32>, y: Option<i32>) -> Self{
        Point { 
            x: match x {
                Some(v) => v,
                None => Point::X,
            }, 
            y: match y {
                Some(v) => v,
                None => Point::Y,
            },
            s: RefCell::new("abc".to_string()),
        }
    }
    pub fn update(&self, ch: char) {
        self.s.borrow_mut().push(ch)
    }
}

impl Point {
    const X: i32 = 0;
    const Y: i32 = 0;
}

fn main() {
    let p = Point::new(Some(10), Some(20));
    p.print_s();
    p.update('!');
    p.print_s();
}
```

**Output**:
```bash
string: abc
string: abc!
```

<br>

# Example: wrong code
The following code will **panic**:
```Rust
use std::cell::RefCell;

fn main() {
    let rc = RefCell::new(10);
    let mut prt1 = rc.borrow_mut();
    println!("{:?}", prt1);
    let mut prt2 = rc.borrow_mut();

    *prt1 += 10;
    *prt2 += 10;
}
```

<br>

**Error**:
```bash
thread 'main' panicked at 'already borrowed: BorrowMutError', src/main.rs:7:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<br>

# Rc<RefCell<T>>
For example, `Rc<RefCell<T>>` is a composition of `Rc` and `RefCell`.<br>
`Rc` itself **can’t** be dereferenced **mutably**, so we put `RefCell` inside to get **shared mutability**.<br>
Now we have **dynamically verified (borrow checks are performed at runtime) shared mutable data**.