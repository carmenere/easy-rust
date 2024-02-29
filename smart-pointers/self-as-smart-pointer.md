# Table of contents
- [Table of contents](#table-of-contents)
- [Passing `self` as `Box<Self>`, `Rc<Self>` or `Arc<Self>`](#passing-self-as-boxself-rcself-or-arcself)
    - [Example](#example)

<br>

# Passing `self` as `Box<Self>`, `Rc<Self>` or `Arc<Self>`
If `self` parameter is declared in *associated function* as `Box<Self>`, `Rc<Self>` or `Arc<Self>` then such methods can **only** be called on value of the given type.

<br>

### Example
```Rust
use std::rc::Rc;

struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn print(&self) {
        println!("{{x: {}; y:{}}}", self.x, self.y);
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
            }
        }
    }
}

impl Point {
    const X: i32 = 0;
    const Y: i32 = 0;
}

impl Point {
    pub fn rc_print(self: Rc<Point>) {
        println!("{{x: {}; y:{}}}", self.x, self.y);
    }
}

fn main() {
    let p3 = Rc::new(Point::new(Some(10), Some(20)));
    p3.rc_print();
}
```

**Output**:
```bash
{x: 10; y:20}                                                                                                                                                               
```

If call `.rc_print()` on `Point` there is **error**:
```Rust
let p = Point::new(Some(10), Some(20));
p.rc_print();
```