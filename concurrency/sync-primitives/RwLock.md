# Table of contents
- [Table of contents](#table-of-contents)
- [RwLock](#rwlock)
- [Examples](#examples)

<br>

# RwLock
`RwLock` stand for **read write locks**.

<br>

# Examples
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
