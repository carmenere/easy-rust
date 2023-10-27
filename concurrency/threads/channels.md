# Channel
**Message passing** is a way for threads to talk to each other.<br>
**Channel** is like Unix pipe: one end is for **sending** data and the other end is for **receiving** and every of 2 ends is placed in **separated thread**.<br>

There is `std::sync::mpsc::channel` for **channels**.<br>
`mpsc` means **multiproducer**, **single consumer**, i.e., **multisender**, **single reciever**.<br>
`mpsc::channel::<(S, R)>()` returns tuple `(Sender<S>, Receiver<R>)`. Then `Sender<S>` and `Receiver<R>` both can be sent to to the spawned threads.<br>
`Sender<T>` implements `Clone` trait. To get channel with **multiple senders** create channel and **clone** the sender as many times as necessary.<br>
`Receiver<T>` **can’t** be cloned.<br>
`Sender<T>` **moves** the **value** it sends into the channel.<br>
`.send()` and `.recv()` methods both return `Result`.<br>
`Receiver<T>` implements the `IntoIterator` trait that allows us to use the `reciever` as an `Iterator` in a `for` loop: `for val in receiver { ... }`.

<br>

#### Example
```Rust
use std::sync::mpsc::{self, Sender, Receiver, RecvError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    let (tx, rx): (Sender<u64>, Receiver<u64>) = mpsc::channel();
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(2);

    threads.push(thread::spawn(move || {
        let r: Result<(), mpsc::SendError<u64>> = tx.send(10);
        match r {
            Ok(_) => println!("The value is successfully sent."),
            Err(_) => println!("An error occured while value was sending."),
        }
    }));

    threads.push(thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let value: Result<u64, RecvError> = rx.recv();
        thread::sleep(Duration::from_secs(1));
        match value {
            Ok(v) => println!("Received value '{}'.", v),
            Err(_) => println!("Got an error."),
        }
    }));

    threads.into_iter().for_each(|t| t.join().unwrap());
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
