# Interior mutability
**Interior mutability** is **property of type** that allows to **change internal** state even if instance of such type was declared as **immutable**.
There are 2 types that allow implement **interior mutability**: ``Cell<T>`` and ``RefCell<T>``.

<br>

# `RefCell<T>`
`T` must be of **Move type**.	

Some usefull methods of `RefCell<T>` methods:
- `RefCell::new(somevalue)` creates new `RefCell`, moving value `somevalue` into it;
- `.borrow()` returns **shared reference** to value inside `RefCell`; panics if the value is **already** **mutably** *borrowed*;
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
