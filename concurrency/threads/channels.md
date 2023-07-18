# Channel
**Message passing** is a way for threads to talk to each other.<br>
**Channel** is like Unix pipe: one end is for **sending** data and the other end is for **receiving** and every of 2 ends is placed in **separated thread**.<br>

There is `std::sync::mpsc::channel` for **channels**.<br>

`mpsc` means **multiproducer**, **single consumer**, i.e., **multisender**, **single reciever**.<br>

`mpsc::channel::<(S, R)>()` returns tuple `(Sender<S>, Receiver<R>)`. Then `Sender<S>` and `Receiver<R>` both can be sent to to the spawned threads.<br>

`Sender<T>` implements `Clone` trait. To get channel with multiple senders create channel and clone the sender as many times as necessary.<br>
`Receiver<T>` **can’t** be cloned.<br>

`Sender<T>` **moves** the **value** it sends into the channel.<br>

`.send()` and `.recv()` methods both return `Result`.<br>

`Receiver<T>` implements the `IntoIterator` trait that allows us to use the `reciever` as an `Iterator` in a `for` loop: `for val in receiver { ... }`.

<br>

#### Example
```Rust
use std::thread::{self, JoinHandle};
use std::sync::mpsc;

fn pipe_1(id: u32) -> (thread::JoinHandle<()>, mpsc::Receiver<(u32, u32)>) {
    let (sender, receiver) = mpsc::channel::<(u32, u32)>();
    let t = thread::spawn(move || {
        println!("My id = {}", id);
        let r = sender.send((id, id + 100));
    });

    (t, receiver)
}

fn pipe_2(rcv: mpsc::Receiver<(u32, u32)>) -> thread::JoinHandle<()> {
    let t = thread::spawn(move || {
        for (id, v) in rcv {
            println!("Got value {} from thred with id = {}.", v, id)
        }
    });
    t
}

fn pipe_chain (ids: &[u32], threads: &mut Vec<thread::JoinHandle<()>>) {
    for id in ids {
        // First pipe
        let (t, rcv) = pipe_1(*id);
        threads.push(t);
        
        // Second pipe
        let t = pipe_2(rcv);
        threads.push(t);
    }
}

fn main() {
    let ids: [u32; 3] = [1, 2, 3];
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(10);

    pipe_chain(&ids, &mut threads);

    println!("Number of threads is {}", threads.len());

    for thread in threads {
        thread.join();
    }
}
```

<br>

# Synchronous Channels
If *receiver* is **slower** than *sender*, values will accumulate in the channel and **consume more memory**.<br>
To **slow down** sender there is **synchronous channel**: `std::sync::mpsc::sync_channel(N)`.<br>
The argument `N` specifies **queue size**.<br>

If the *queue* is **full** a call to `.send()` will **block** until there is a free space in the *queue*.<br>

Example:
```Rust
let (sender, receiver) = mpsc::sync_channel::<(u32, u32)>(10);
```
