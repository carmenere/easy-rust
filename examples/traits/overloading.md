# Example1: overloading the `+` operator for `struct Point`
```Rust
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

<br>

# Example2: use references
```Rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point{x: 10, y: 20};
    let p2 = Point {x: 20, y: 20};
    let result = &p1.add(&p2);
    dbg!(result);
    assert_eq!(&p1 + &p2, Point{x:30, y:40});
}
```

<br>

### Example3: overloading operator when `lhs` and `rhs` are of different type:
```Rust
use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let p1: Millimeters = Millimeters(10);
    let p2: Meters = Meters(20);
    let result: Millimeters = p1.add(p2);
    dbg!(result);
}
```