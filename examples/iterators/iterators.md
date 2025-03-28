# Function that returns an iterator that yield back the sequence of slices
Function must _receive_ **collection**, for example: `[1, 2, 3, 4]`.<br>
And _return_ **sequence of slices**:
```rust
[1, 2, 3, 4]
[2, 3, 4]
[3, 4]
[4]
[]
```

<br>

**Solution**:
```rust
fn tails<T>(slice: &[T]) -> impl Iterator<Item = &[T]> {
    (0..=slice.len()).map(|i| &slice[i..])
}

fn main() {
    let my_array = [1, 2, 3, 4];
    let result = tails(&my_array);
    dbg!(result.collect::<Vec<_>>());
}
```