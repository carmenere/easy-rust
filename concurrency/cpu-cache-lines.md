# Cache coherence
**Cache coherence** ensures that all CPU or/and cores maintain a consistent view of memory.<br>

<br>

Hardware-based solutions for **cache coherence**:
- **snooping-based** coherence implementations;
- **directory-based** coherence implementations;

<br>

Every CPU/core has **cache controller** which tracks the status of each **cache line** in its cache. Each cache controller can send and receives messages from other cache controllers.<br>
In a **snooping system**, all the _cache controllers_ **monitor** (or **snoop**) the memory bus transactions and react accordingly, to maintain memory coherence.<br>

<br>

There are some **snooping protocols**:
1. **MESI protocol**.
2. **MESIF protocol**.
2. **MOESI protocol**.

<br>

Every _cache line_ in cache has **state**. The letters in the acronyms **MESI** represent **states** of **cache lines**:
- Modified (**M**)
- Exclusive (**E**)
- Shared (**S**)
- Invalid (**I**)

<br>

A **write** may only be performed freely if the **cache line** is in the **Modified** or **Exclusive** state.<br>
If it is in the **Shared** state, all other cached copies must be **invalidated** first. This is typically done by a broadcast operation known as **Request For Ownership** (**RFO**).<br>
A cache that holds a line in the **Modified** state must **snoop** (**intercept**) all attempted reads (from all the other caches in the system) of the corresponding main memory location and insert the data that it holds.<br>

<br>

When **different threads** access **the same cache line simultaneously** it causes to **performance penalty** because of _cache line_ **invalidation**.<br>
 and every time it is updated cache controller must enforce cache cogerency using some algorithem: MESI,

<br>

# CPU cache side effects
`CACHE_LINE_SIZE=1` means data in **the same** _cache line_.<br>
`CACHE_LINE_SIZE=64` means data in **different** *cache line*s.<br>

<br>

# Lock free access to array
**Lock free** sequence means every thread **always** accesses sequence **at specific index**.<br>

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