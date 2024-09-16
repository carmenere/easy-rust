# Table of contents
- [Table of contents](#table-of-contents)
- [`park` and `unpark`](#park-and-unpark)
- [Examples](#examples)
  - [Just one thread](#just-one-thread)
  - [Multiple threads](#multiple-threads)

<br>

# `park` and `unpark`
`std::thread::park` blocks the **current thread**, which can then be *resumed* **from another thread** by calling the `unpark` method on the blocked **thread’s handle**.

<br>

```Rust
use std::thread;
use std::time::Duration;

fn main () {
    let parked_thread: thread::JoinHandle<()> = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    // Let some time pass for the thread to be spawned.
    thread::sleep(Duration::from_millis(10));

    println!("Unpark the thread");
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}
```

<br>

# Examples
## Just one thread
```Rust
use std::{thread, thread::JoinHandle, time::Duration};
use rand::{Rng};

fn main() {
    let mut v = (1..=100).collect::<Vec<u32>>();

    let handle: JoinHandle<u64> = thread::spawn(|| {
        let mut r = rand::thread_rng();
        let id = thread::current().id();
        let delay = r.gen_range(1..=5);
        println!("Thread id: {:?}", id);
        thread::sleep(Duration::from_secs(delay));
        println!("Thread id: {:?}", id);
        delay
    });
    
    let r = handle.join();
    if let Ok(r) = r {
        println!("Result: {}", r);
    }
}
```

<br>

## Multiple threads
```Rust
use std::{thread, thread::JoinHandle, time::Duration};
use rand::{Rng};

fn main() {
    let mut v = (1..=10).collect::<Vec<u32>>();

    let handles: Vec<JoinHandle<u64>> = v.iter().map(|i| {
        thread::spawn(|| {
            let mut r = rand::thread_rng();
            let id = thread::current().id();
            let delay = r.gen_range(1..=5);
            println!("Thread id: {:?}, will sleep {} sec. ", id, delay);
            thread::sleep(Duration::from_secs(delay));
            println!("Thread id: {:?}, waked up, continue execution.", id);
            delay * delay
        })
    }).collect();

    for h in handles {
        let id = h.thread().id();
        if let Ok(r) = h.join() {
            println!("Thread id: {:?}, result: {}", id, r);
        }
    }
}
```

<br>