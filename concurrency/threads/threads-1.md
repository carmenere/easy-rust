# Table of contents
- [Table of contents](#table-of-contents)
- [Threads](#threads)
  - [Using `JoinHandle`s to wait for threads to finish](#using-joinhandles-to-wait-for-threads-to-finish)
- [Scoped threads](#scoped-threads)
- [Channels](#channels)

<br>

# Threads
You create threads with `std::thread::spawn` and a closure to tell it what to do. Creating threads is also called **spawning threads**.<br>

Example:
```rust
fn main() {
std::thread::spawn(|| {
println!("I am printing something");
});
}
```

In fact, the **output will be different** every time. Sometimes it will print, and sometimes it won’t. That is because sometimes `main()` finishes **before** the thread finishes, and when `main()` **finishes**, the **whole program is over**.<br>
Also, sometimes the threads will panic with error `cannot access stdout during`. This error occurs when the *thread tries to do something* just when the *program is
shutting down*.<br>

The better way to **avoid** termination of *main thread* is to **stop** *main thread* **until** the all spawned are over. The `spawn()` function actually returns something called a `JoinHandle` that lets us do exactly this:
```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T,
    F: Send + 'static,
    T: Send + 'static,
```

<br>

- `'static` means that the closure and its return value must have a lifetime of the whole program;
  - that’s because **threads can outlive the scope they have been created in**;
  - since we can’t know when it will return, we need to have them as long as possible, so until the end of the program;
- `f` is the closure;

<br>

## Using `JoinHandle`s to wait for threads to finish
Consider example:
```rust
fn main() {
    for _ in 0..10 {
        let handle = std::thread::spawn(|| {
            println!("I am printing something");
        });
        handle.join();
    }
}
```

In the above example we start a thread, do something, and then call `.join()` to wait and only then we start a new thread.<br>

But we want to **start all the threads at the same time** and **only then call** `.join()` on the threads. To solve this, we can create a `Vec` that will hold all of the `JoinHandles`. Then we can call `.join()` on them:
```rust
use std::{thread::{spawn, JoinHandle}, time::Duration};

fn main() {
    let threads = (1..=10).into_iter().map(|id| {
        spawn(move || {
            println!("thread {}", id);
            id
        })
    }).collect::<Vec<_>>();

    threads.into_iter().for_each(|j| {
        let result = j.join().unwrap();
        println!("thread with id {} is over", result);
    });
}
```
**Output**:
```rust
thread 2
thread 1
thread 5
thread 6
thread 4
thread 8
thread with id 1 is over
thread with id 2 is over
thread 7
thread 10
thread 9
thread 3
thread with id 3 is over
thread with id 4 is over
thread with id 5 is over
thread with id 6 is over
thread with id 7 is over
thread with id 8 is over
thread with id 9 is over
thread with id 10 is over
```

<br>

# Scoped threads
**Regular threads** (**non-scoped threads**) need a `'static` guarantee.<br>
Unlike *non-scoped threads*, **scoped threads** *can borrow* **non**-`'static` data, as the **scope guarantees all threads will be joined at the end of the scope**.<br>

With **scoped threads**, you start with a **scope**, using `std::thread::scope()`:
```rust
fn main() {
    std::thread::scope(|s| { // The "s" is a name of scope here.
        s.spawn(|| {
            sleep(Duration::new(1, 0));
            println!("1 hello!");
        });

        s.spawn(|| {
            println!("2 hello!");
        });
    }); // The threads automatically are joined here, so there’s no need to think about JoinHandles.
}
```
**Output**:
```bash
2 hello!
1 hello!
```

<br>

You still need a `Mutex` because more than one thread is changing some value, but you **don’t need** an `Arc` **anymore**. You **don’t need to use** `move` because the threads **just borrow the values** because the **threads are guaranteed to not exist after the scope is over**.<br>

<br>

# Channels
You can create a **channel** in Rust with the `channel()` function in `std::sync::mpsc`. The `mpsc` stand for **multiple producer, single consumer**.<br>
The `channel()` function creates a `Sender` and a `Receiver`. They are tied together, and both hold the same **generic type**.<br>

The `channel()` function signature:
```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
```

The output of the `channel()` function is a `tuple`.<br>

You could specify the type if you want:
```rust
fn main() {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();
    sender.send(5);
    receiver.recv();
}
```

<br>

Also you can omitt type declaration: once you send value `sender.send(5);` Rust will be able to infer the type:
```rust
fn main() {
    let (sender, receiver) = channel();
    sender.send(5);
    receiver.recv();
}
```

<br>

Each of these methods might fail, so they each return a `Result`:
- the `.send()` method for **sender** returns `Result<(), SendError<i32>>`;
  - `.send()` will **return** an `Err` if the `Receiver` **has been dropped**;
- the `.recv()` method for **receiver** returns `Result<i32, RecvError>`;
  - `.recv()` will **return** an `Err` if the `Sender` **has been dropped AND** there is **no data** to receive;
    - it means that the **all data has been received** and the **channel has been closed**;
  - `.recv()` will **return** an `Ok` if the `Sender` **has been dropped AND** there is **still data** to receive;
  - `.recv()` will **keep blocking** if the `Sender` is **alive AND** there is **no data** to receive;

<br>

A `channel` is like an `Arc` because you **can clone it** and **send the clones into other threads**:
```rust
use std::{sync::mpsc::{channel, Receiver, Sender}, thread::sleep, time::Duration};

fn main() {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::spawn(move || {
        sender.send("A").unwrap();
        sleep(Duration::new(1, 0));
        sender.send("B").unwrap();
    });

    std::thread::spawn(move || {
        sender_clone.send("C").unwrap();
        sender_clone.send("D").unwrap();
    });
    
    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }
}
```
**Output**:
```bash
A
C
D
B
```

<br>

**Example** - **third** `.recv()` will **keep blocking**  and the **program will never end**:
```rust
use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    sender.send(5).unwrap();
    sender.send(5).unwrap();

    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
}
```
**Output**:
```rust
Ok(5)
Ok(5)

```

<br>

**Example** - **third** `.recv()` will return `Err`:
```rust
use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    sender.send(5).unwrap();
    sender.send(5).unwrap();
    drop(sender);

    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
    println!("{:?}", receiver.recv());
}
```
**Output**:
```rust
Ok(5)
Ok(5)
Err(RecvError)
```

<br>