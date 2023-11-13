# Example: generic method `.sum()` and generic function `sum()`
```rust
trait Sum<T> {
    fn sum(&self) -> T;
}

impl<T> Sum<T> for Vec<T> 
where 
    T: Default + std::ops::Add<T> + std::ops::AddAssign + Copy
{
    fn sum(&self) -> T {
        let mut sum: T = T::default();
        for i in self {
            sum += *i;
        }
        sum
    }
}

fn sum<T>(v: Vec<T>) -> T
where
    T: std::ops::Add + Copy + std::iter::Sum
{
    v.into_iter().sum() 
}

fn main() {
    let v = vec![1,2,3];
    println!("{}", v.sum());
    println!("{}", sum(v));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_method() {
        let v = vec![1,2,3];
        assert_eq!(6, v.sum());
    }

    #[test]
    fn sum_function() {
        let v = vec![1,2,3];
        assert_eq!(6, sum(v));
    }
}
```
