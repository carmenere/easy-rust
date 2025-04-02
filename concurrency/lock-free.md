# Lock free approach
**Lock free** access to sequence means every thread gets its own index and **always** accesses sequence by its **index**.<br>

<br>

In examples below:
- `CACHE_LINE_SIZE=1` means that data of all threads are in **the same** _cache line_;
- `CACHE_LINE_SIZE=64` means that data of all threads are **separated** in **different** _cache lines_;

<br>

## Cache lines: using static array
```rust
use std::time::Instant;
use envparse::parse_env;

const N_CORES: usize = 4;
const N_ITERATIONS: u32 = 10_000_000;
const CACHE_LINE_SIZE: usize = parse_env!("CACHE_LINE_SIZE" as usize);
static mut ARRAY: [u32; N_CORES*CACHE_LINE_SIZE] = [0; N_CORES*CACHE_LINE_SIZE];

fn main() {
    let now = Instant::now();
    static mut arr: &mut [u32; N_CORES*CACHE_LINE_SIZE] = unsafe {&mut ARRAY};
    let mut threads = Vec::with_capacity(N_CORES);

    for index in 0..N_CORES {
        threads.push(std::thread::spawn(move || {
            unsafe {
                let mut ptr = arr[index * CACHE_LINE_SIZE..(index + 1) * CACHE_LINE_SIZE].as_mut_ptr();
                println!("index = {}, ptr = {:p}", index, ptr);
                for i in 0..N_ITERATIONS {
                    if i % 2 > 0 {
                         *ptr += i % 2;
                    } else {
                         *ptr += 1;
                    }
                     *ptr -= 1;
                }
            }
        }))
    }

    threads.into_iter().for_each(|t| t.join().unwrap());

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

## Cache lines: using scoped threads
```rust
use std::time::Instant;
use envparse::parse_env;

const N_CORES: usize = 4;
const N_ITERATIONS: u32 = 10_000_000;
const CACHE_LINE_SIZE: usize = parse_env!("CACHE_LINE_SIZE" as usize);

fn main() {
    let now = Instant::now();

    let mut vec: Vec<u32>  = vec![0; N_CORES*CACHE_LINE_SIZE];

    std::thread::scope(|s| {
        for (idx, chunk) in vec.chunks_mut(CACHE_LINE_SIZE).enumerate() {
            s.spawn(move || {
                let mut ptr = chunk.iter_mut().next().unwrap();
                println!("index = {}, ptr = {:p}", idx, ptr);
                for i in 0..N_ITERATIONS {
                    if i % 2 > 0 {
                        *ptr += i % 2;
                    } else {
                        *ptr += 1;
                    }
                    *ptr -= 1;
                }
            });
        }
    });

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```

<br>

# Compare Lock free with Mutex
```rust
use std::sync::{Arc, Mutex};
use std::time::Instant;
use envparse::parse_env;

const N_CORES: usize = 8;
const N_ITERATIONS: u32 = 10_000_000;

fn main() {
    let now = Instant::now();

    let mut vec: Arc<Mutex<Vec<u32>>>  = Arc::new(Mutex::new(vec![0; 100]));

    for i in 0..N_CORES {
        let mut vec = vec.clone();
        std::thread::spawn(move || {

            for i in 0..N_ITERATIONS {
                {
                    let mut vec = vec.lock().unwrap();
                    if i % 2 > 0 {
                        vec[0] += i % 2;
                    } else {
                        vec[0] += 1;
                    }
                    vec[0] -= 1;
                }
            }
        });
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
```