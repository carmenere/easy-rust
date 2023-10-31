# Operator Overloading
A great example of a situation where this **associated types** are useful is **operator overloading**.<br>

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can **overload** the operations and corresponding **traits** listed **in** `std::ops` by implementing the traits associated with the operator.<br>

<br>

### Example 1: overloading the `+` operator to add two `Point` instances together:
```Rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

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

The new part is `Rhs=Self`: this syntax is called **default type parameters**.<br>
The `Rhs` (*right hand side*) **generic type parameter** defines the type of the `rhs` parameter in the `add()` method.<br>

Notes:
- If we don’t specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will **default** to `Self`, which will be the type we’re implementing `Add` on. 
- When we implement `Add` for `Point`, we use the **default** for `Rhs` because we wante to add two `Point` instances.

<br>

### Example 2: overloading operator when `lhs` and `rhs` are of different type:
```Rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

<br>

### Example 3: generic version of Point
```Rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T, V> 
where 
    T: Add<Output = T> + Into<T>, 
    V: Add<Output = V> + Into<V>
{
    x: T,
    y: V
}

impl<T, V> Add<Point<T, V>> for Point<T, V> 
where 
    T: Add<Output = T> + Into<T>,
    V: Add<Output = V> + Into<V>
{
    type Output = Point<T, V>;

    fn add(self, rhs: Point<T, V>) -> Self::Output {
        Point {x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

fn main() {
    println!("{:?}", Point::<u8, u64> {x: 1, y: 10} + Point::<u8, u64> {x: 1, y: 4});
    println!("{:?}", Point {x: 1.0, y: 10.0} + Point {x: 1.0, y: 4.0});
}
```