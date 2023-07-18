# Mutex type
The only way is to get data inside `Mutex` is to call `.lock()` on `Mutex`:
- `.lock()` returns `Result<MutexGuard<T>>`;
- `.unwrap()` returns `MutexGuard<T>`.

<br>

Due to **deref coercions** we can call `T`’s methods on the wrapping `MutexGuard` instance.<br>
When `MutexGuard` instance is **dropped** – the **lock** is **released**.<br>
To **drop** `MutexGuard` explicitly as soon as possible there is `drop()` function: `drop(vsafe);`.

<br>

```Rust
use std::thread::{self, JoinHandle};
use std::sync::{Arc, Mutex};

fn main() {
    let shared_vector = Arc::new(Mutex::new(vec![100, 200, 300]));
    let ids = [1, 2, 3];
    let mut threads = Vec::with_capacity(10);
    for id in ids {
        let a = shared_vector.clone();
        threads.push(
            thread::spawn(move || {
                let a = &a;
                let lock = a.lock();
                if lock.is_ok() {
                    let mut vsafe = lock.unwrap();
                    vsafe.push(id.clone());
                    println!("My thread id is {}. v.len() = {}", id.clone(), vsafe.len())
                }
                else {
                    println!("My thread id is {}. Can not take lock!", id.clone())
                }
            })
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```

<br>

# Atomics
Constructors for all **atomic types** are all `const` functions: `const fn abc() { … }`.

```Rust
use std::thread::{self, JoinHandle};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};

fn main() {
    let counter = Arc::new(AtomicI64::new(10));
    let ids = [1, 2, 3, 4, 5];
    let mut threads = Vec::with_capacity(10);

    for id in ids {
        let c = counter.clone();
        threads.push(
            thread::spawn(move || {
                let c = c;
                c.fetch_add(id, Ordering::SeqCst);
                println!("My thread id is {}. Counter = {}", id, c.load(Ordering::SeqCst))}
            )
        )
    }

    for thread in threads {
        let r = thread.join();
    }
}
```

<br>

# Read Write locks
```Rust 
use std::thread::{self, JoinHandle};
use std::sync::{Arc, RwLock};

fn main() {
    let app_config = Arc::new(RwLock::new(vec![100, 200, 300]));
    
    let ids = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut threads = Vec::with_capacity(10);

    for id in ids {
        let conf = app_config.clone();

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

<br>

# Condvars
Path to type `std::sync::Condvar`.<br>

|Method|Description|
|:-----|:----------|
|`.wait()`|Blocks calling thread until some other thread calls `.notify_all()`.|
|`.notify_all()`|When desired condition becomes true – we must call `.notify_all()` to wake up any waiting thead.|

<br>

# Global variables
It is possible to use **global atomic variable**.<br>
But **atomic globals** are limited to `integers` and `bool`.
