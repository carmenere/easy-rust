# Table of contents
- [Table of contents](#table-of-contents)
- [URLs](#urls)
- [`park` and `unpark`](#park-and-unpark)
  - [Example](#example)

<br>

<br>

# URLs
|Smart pointer|URL|
|:----|:------------|
|`thread::park` function|[**thread::park**](https://doc.rust-lang.org/std/thread/fn.park.html)|
|`thread::Thread::unpark()` method|[**thread::Thread::unpark**](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark)|

<br>

# `park` and `unpark`
While a **mutex** does allow threads to wait until it becomes unlocked, it **doesn't** provide functionality for **waiting for** any other **conditions**.<br>
There are 2 ways to wait for a notification from another thread:
- **thread parking**;
- **condition variables**;

<br>

A thread can **park** *itself*, which puts it to sleep. *Another thread* can then **unpark** the *parked thread*, **waking** it up from its nap.<br>
The `std::thread::park()` function blocks the **current thread**. For unparking there is `unpark()` method on a Thread object representing the thread that you want to unpark.<br>

The `thread::park` function has variant with a time limit: `std::thread::park_timeout(dur: Duration)` function, it **blocks until** the current thread **awakens** or the specified duration has been **reached**.<br>

<br>

## Example
In the following example a spaned thread will consume items from queue, while the main thread will insert a new item into the queue every second.<br>
```Rust
use std::thread;
use std::{sync::Mutex, thread};
use std::collections::VecDeque;
use std::time::Duration;

fn main() {
    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // Consuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(i) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // Producing thread
        for i in 1..=10 {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
```

<br>

The **consuming thread** - let's call it **C** - runs an **infinite loop** in which it
- **locks** the queue;
- **pops** item from the queue;
- **unlocks** the queue;
- if queue returns `None` it **calls** `park()` to go to sleep;

<br>

The **producing thread** - let's call it **P**:
- **locks** the queue;
- **pushes** new item onto the queue;
- **unlocks** the queue;
- **calls** `unpark()` to **notify C** that there are new items;

<br>

An important property of thread parking is that call to `unpark()` **before** the thread parks itself **doesn't get lost**.<br>
The request to **unpark** is still **recorded**, and the next time the thread tries to park itself, it **clears** that request and directly continuous **without** actually going to sleep.<br>
However unpark requests **don't** stack up. Calling `unpark()` **two** times and then calling `park()` **two** times afterwards still results in the **thread going to sleep**. the **first** `park()` **clears** the request and returns directly, but the **second** one **goes to sleep** as usual.<br>
