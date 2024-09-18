# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [RwLock](#rwlock)
  - [Example](#example)

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`RwLock`|[**std::sync::RwLock**](https://doc.rust-lang.org/std/sync/struct.RwLock.html)|

<br>

# RwLock
An `RwLock<T>` or **reader-writer lock** is the concurent version of a `RefCell<T>`. An `RwLock<T>` holds a `T` and tracks any outstanding borrows. However, unlike `RefCell<T>`, it **doesn't panic** on conflicting borrows. Instead, it blocks the current thread - putting it to sleep.<br>

**Borrowing** the content of an `RwLock<T>` is called **locking**.<br>

An `RwLock<T>` is slightly more complicated version of `Mutex<T>` that understands the difference between **exclusive** and **shared** access, so it has **3 states**:
- **unlocked**;
- **locked** by **single writer** (for **exclusive access**);
- **locked** by **any number of readers** (for **shared access**);

It is used for data that is **often read** by multiple threads, but only **updated once** in a while.<br>

An `RwLock<T>` has 2 methods **to acquire lock** and 2 guard types **to access data**:
- `read()` for locking as **reader**, it returns `RwLockReadGuard<T>`;
- `write()` for locking as **writer**, it returns `RwLockWriteGuard<T>`;

<br>

The `RwLockReadGuard<T>` implements `Deref` to behave like a **shared reference** to the protected data.
The `RwLockWriteGuard<T>` implements `DerefMut` to behave like an **exclusive reference** to the protected data.

<br>

Both `Mutex<T>` and `RwLock<T>` require `T` to be `Send`, because they can be used to send a `T` to another thread.<br>
An `RwLock<T>` additionally requires `T` to be `Sync`, because it allows multiple threads to hold a shared reference (`&T`) to the protected data.

<br>

The implementation of `RwLock<T>` depends on the OS. Most implementations will **block new readers** if there is a **writer waiting** releasing of read lock. This is to prevent **writer starvation**, a situation when many readers **never** allowing any writer to update the data.

<br>

## Example
```Rust 
use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};

fn main() {
    let conf = Arc::new(RwLock::new(vec![100, 200, 300]));
    
    let ids = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut threads = Vec::with_capacity(10);

    for id in ids {
        let conf = conf.clone();

        threads.push(
            thread::spawn(move || {
                let conf = &conf;
                if id != 2 && id != 6 {
                    let c = conf.read();

                    if c.is_ok() {
                        let safe_conf = c.unwrap();
                        println!("My thread id is {}. conf = {:?}", id.clone(), safe_conf)
                    }
                    else {
                        println!("My thread id is {}. Can not read conf!", id.clone())
                    }
                }

                else {
                    let c = conf.write();

                    if c.is_ok() {
                        let mut safe_conf = c.unwrap();
                        *safe_conf = vec![700 + id, 800 + id, 900 + id];
                        println!("My thread id is {}. conf = {:?}", id.clone(), safe_conf)
                    }
                    else {
                        println!("My thread id is {}. Can not write conf!", id.clone())
                    }
                }
            })
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```
