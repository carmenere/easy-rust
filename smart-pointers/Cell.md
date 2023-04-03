# Interior mutability
**Interior mutability** is **property of type** that allows to **change internal** state even if instance of such type was declared as **immutable**.
There are 2 types that allow implement **interior mutability**: ``Cell<T>`` and ``RefCell<T>``.

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
