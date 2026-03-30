# `rayon`
Rayon is a crate that lets you **automatically** spawn multiple threads when working with iterators and related types. Instead of using `thread::spawn()` to spawn threads, you can just add `par_` to the **iterator methods** you already know (``).

The speedup that Rayon gives will depend a lot on your code and the number of threads on your computer.<br>

First, we will use a method `std::thread::available_parallelism()` to see how many threads are available.<br>

<br>

**Example**:
```rust
fn main() {
    let n = std::thread::available_parallelism();

    println!("Available threads: {:?}, the number of threads will be spawned: {:?}", n, rayon::current_num_threads());

    let my_vec = vec![0; 4_000_000_000];
    let start1 = std::time::Instant::now();
    let r= my_vec
        .iter()
        .enumerate()
        .fold(0, |acc, item| acc + item.0);
    println!("Without rayon result = {}, time = {:?}", r, start1.elapsed());

    let my_vec = vec![0; 4_000_000_000];
    let start2 = std::time::Instant::now();
    let r= my_vec
        .par_iter()
        .enumerate()
        .fold(|| 0, |acc, item| acc + item.0).sum::<usize>();
    println!("With rayon result = {}, time = {:?}", r, start2.elapsed());
}
```

**Output**:
```bash
cargo run --release
Available threads: Ok(10), the number of threads will be spawned: 10
Without rayon result = 7999999998000000000, time = 42ns
With rayon result = 7999999998000000000, time = 488.417µs
```

```bash
cargo run
Available threads: Ok(10), the number of threads will be spawned: 10
Without rayon result = 7999999998000000000, time = 1.282937166s
With rayon result = 7999999998000000000, time = 225.613292ms
```

<br>