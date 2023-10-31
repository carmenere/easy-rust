# Example: generic version of Point
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