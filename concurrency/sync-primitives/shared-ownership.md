# Table of contents
- [Table of contents](#table-of-contents)
- [Shared ownership](#shared-ownership)

<br>

# Shared ownership
To share data between threads there is `Arc` type.<br>

```Rust
use std::thread;

use std::sync::Arc;

fn main() {
    let shared_vector = Arc::new(vec![100, 200, 300]);
    let ids = [1, 2, 3];
    let mut threads = Vec::with_capacity(10);
    for id in ids {
        let shared_vector_per_thread = shared_vector.clone();
        threads.push(
            thread::spawn(move || {
                let v = &shared_vector_per_thread;
                println!("My thread id is {}. v.len() = {}", id, v.len())}
            )
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```