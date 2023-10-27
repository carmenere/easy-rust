# Example: join threads in order they are terminated
```Rust
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let counter = 0u64;

    let threads = (0..=10).map(|i| {
        thread::spawn(move || {
            println!("i = {}, tid = {:?} ... ", i, thread::current().id());
            thread::sleep(Duration::from_secs(10 - i));
            println!("i = {}, tid = {:?} finished", i, thread::current().id());
            i
        })
    }).collect::<Vec<_>>();

    println!("main thread");

    // t.join() blocks main thread until thread t will be terminated;
    threads.into_iter().rev().for_each(|t| {
        let tid = t.thread().id();
        let r = t.join();
        println!("Join thread with i {}, tid = {:?}", r.unwrap(), tid);
    });
}
```

<br>

**Output**<br>
```bash
i = 0, tid = ThreadId(2) ... 
i = 1, tid = ThreadId(3) ... 
i = 2, tid = ThreadId(4) ... 
i = 3, tid = ThreadId(5) ... 
i = 4, tid = ThreadId(6) ... 
main thread
i = 10, tid = ThreadId(12) ... 
i = 10, tid = ThreadId(12) finished
i = 5, tid = ThreadId(7) ... 
i = 6, tid = ThreadId(8) ... 
i = 7, tid = ThreadId(9) ... 
i = 8, tid = ThreadId(10) ... 
i = 9, tid = ThreadId(11) ... 
Join thread with i 10, tid = ThreadId(12)
i = 9, tid = ThreadId(11) finished
Join thread with i 9, tid = ThreadId(11)
i = 8, tid = ThreadId(10) finished
Join thread with i 8, tid = ThreadId(10)
i = 7, tid = ThreadId(9) finished
Join thread with i 7, tid = ThreadId(9)
i = 6, tid = ThreadId(8) finished
Join thread with i 6, tid = ThreadId(8)
i = 5, tid = ThreadId(7) finished
Join thread with i 5, tid = ThreadId(7)
i = 4, tid = ThreadId(6) finished
Join thread with i 4, tid = ThreadId(6)
i = 3, tid = ThreadId(5) finished
Join thread with i 3, tid = ThreadId(5)
i = 2, tid = ThreadId(4) finished
Join thread with i 2, tid = ThreadId(4)
i = 1, tid = ThreadId(3) finished
Join thread with i 1, tid = ThreadId(3)
i = 0, tid = ThreadId(2) finished
Join thread with i 0, tid = ThreadId(2)
```