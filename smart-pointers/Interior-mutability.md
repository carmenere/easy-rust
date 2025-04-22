# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [Declarations](#declarations)
  - [`UnsafeCell<T>`](#unsafecellt)
  - [`OnceCell<T>`](#oncecellt)
  - [`Cell<T>`](#cellt)
  - [`RefCell<T>`](#refcellt)
- [Interior mutability](#interior-mutability)
  - [`UnsafeCell<T>`](#unsafecellt-1)
  - [`Cell<T>`](#cellt-1)
  - [`RefCell<T>`](#refcellt-1)
  - [RefCell vs. Cell](#refcell-vs-cell)
- [Rc\<RefCell\>](#rcrefcell)
- [Examples](#examples)
  - [Bypass borrow checker with interior mutability](#bypass-borrow-checker-with-interior-mutability)
  - [`Cell<T>`](#cellt-2)
  - [`RefCell<T>`](#refcellt-2)
  - [Second call `borrow_mut()`](#second-call-borrow_mut)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`UnsafeCell<T>`|[**std::cell::UnsafeCell**](https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html)|
|`OnceCell<T>`|[**std::cell::OnceCell**](https://doc.rust-lang.org/stable/std/cell/struct.OnceCell.html)|
|`Cell<T>`|[**std::cell::Cell**](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html)|
|`RefCell<T>`|[**std::cell::RefCell**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html)|

<br>

# Declarations
## `UnsafeCell<T>`
```rust
pub struct UnsafeCell<T: ?Sized> {
    value: T,
}
```

<br>

## `OnceCell<T>`
```rust
pub struct OnceCell<T> {
    inner: UnsafeCell<Option<T>>,
}
```

<br>

## `Cell<T>`
```rust
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}
```

<br>

## `RefCell<T>`
```rust
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    borrowed_at: Cell<Option<&'static crate::panic::Location<'static>>>,
    value: UnsafeCell<T>,
}
```

<br>

# Interior mutability
**Interior mutability** enables **mutation** through a **shared** (**immutable**) **reference**.<br>
In other words, **interior mutability bypasses** Rust's borrowing rules and **mutate** value through **shared** reference (`&T`).<br>
**Types** that **allow** *interior mutability* are called **cell types**.<br>
All **cell types** internally use `UnsafeCell` to wrap their data.<br>
**Cell types** come in 3 flavors:
|Cell type|Description|
|:--------|:----------|
|`UnsafeCell<T>`|Core primitive for **interior mutability**.|
|`Cell<T>`|Provides **zero-cost interior mutability**, but only for `Copy` types.|
|`RefCell<T>`|Provides **interior mutability** performing **borrow checking at runtime**. **Isn’t** restricted to `Copy` types.|
|`OnceCell<T>`|A cell which can be written to only **once**.|

<br>

**All cell types** are **not** *thread safe*. So, **cell types are not** `Sync`.<br>
If `Cell<T>` (or any other **cell type**) was `Sync` then `Cell<T>` would be `Send` and could be shared between threads.<br>
To make *cell types* **thread safe** use `Mutex`.<br>

<br>

So, **all cell types** implement `!Sync`:
```rust
impl<T>         !Sync for OnceCell<T> {}
impl<T: ?Sized> !Sync for UnsafeCell<T> {}
impl<T: ?Sized> !Sync for Cell<T> {}
impl<T: ?Sized> !Sync for RefCell<T> {}
```

<br>

## `UnsafeCell<T>`
This type is **building block** for **interior mutability**.<br>
An `UnsafeCell` wraps `T`, but doesn't come with any conditions or restrictions to avoid UB. Instead, its `get()` method just returns a **raw pointer** `*mut T` to the value it wraps:
```rust
impl<T: ?Sized> UnsafeCell<T> {
    pub const fn get(&self) -> *mut T {
        self as *const UnsafeCell<T> as *const T as *mut T
    }
}
```

It leaves it up to the developper to use it in a way that does not cause any UB.<br>

<br>

The `UnsafeCell` type implements `Send` if `T` does:
```rust
impl<T: ?Sized> Send for UnsafeCell<T>
where
    T: Send,
```

<br>

## `Cell<T>`
The `Cell<T>` type provides **zero-cost interior mutability**, but only for `Copy` types.<br>
To avoid UB, it only allows you to **copy** the value out or **replace** it with another value as a whole. That's why it requires `T` to be of **Copy type**.<br>
The `Cell<T>` can be used within a **single** thread.<br>

The `Cell<T>` type implements `Send` if `T` does:
```rust
unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}
```

<br>

Some usefull methods of `Cell<T>`:
- [**Cell::new(val)**](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.new):
  - **creates** new `Cell`, **moving** value `val` into it;
- [**get()**](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.get):
  - **returns** **copy** of the contained value;
- [**set(val)**](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.set):
  - **stores** the given value `val` in the `Cell` and **drops** the **previous** value;
- [**replace(val)**](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.replace):
  - **replaces** the contained value with `val`, and **returns** the **old** contained value.

<br>

## `RefCell<T>`
Unlike a `Cell<T>`, a `RefCell<T>` type allows to **borrow** its contents at a small runtime cost.<br>
The `RefCell<T>` performs **borrow checking at runtime**: if you try to borrow it while it is already mutably borrowed (or vice-versa), it will panic, which avoids UB.<br>
The `RefCell<T>` can be used within a **single** thread.<br>

The `RefCell<T>` type implements `Send` if `T` does:
```rust
unsafe impl<T: ?Sized> Send for RefCell<T> where T: Send {}
```

<br>

Some usefull methods of `RefCell<T>` methods:
- [**Cell::new(val)**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.new):
  - **creates** new `RefCell`, moving value `val` into it;
- [**get_mut()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.get_mut):
  - returns a **mutable** reference to the underlying data.
- [**into_inner()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.into_inner):
  - **consumes** the `RefCell`, **returning** the wrapped value.
- [**borrow()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.borrow):
  - returns **shared** reference to value inside `RefCell`;
  - panics if the value is **already mutably** *borrowed*;
- [**borrow_mut()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.borrow_mut):
  - returns **mutable** reference to value in `RefCell`;
  - panics if the value is **already** *borrowed*;
- [**try_borrow()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.try_borrow):
  - returns `Result`:
    - returns `Err` if the value is already **mutably** *borrowed*;
- [**try_borrow_mut()**](https://doc.rust-lang.org/stable/std/cell/struct.RefCell.html#method.try_borrow_mut):
  - returns `Result`:
    - returns `Err` if the value is **already** *borrowed*.

<br>

## RefCell vs. Cell
Differences between `RefCell` and `Cell`:
1. Types wrapped in `Cell` **must implement** the `Copy` trait while those in `RefCell` **needn't**.<br>This makes sense. When calling `get` on a `Cell`, you are getting a **copy** of the wrapped data, whereas the methods associated with `RefCell` are `borrow` and `borrow_mut`, which return references to the underlying data.
1. `Cell` **never panics**. But references to `RefCell` internal value are checked **at runtime**, which can cause program to **panic**.

<br>


# Rc<RefCell<T>>
For example, `Rc<RefCell<T>>` is a composition of `Rc` and `RefCell`.<br>
`Rc` itself **can’t** be dereferenced **mutably**, so we put `RefCell` inside to get **shared mutability**.<br>
Now we have **dynamically verified (borrow checks are performed at runtime) shared mutable data**.

<br>

# Examples
## Bypass borrow checker with interior mutability
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

## `Cell<T>`
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

## `RefCell<T>`
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

## Second call `borrow_mut()`
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
