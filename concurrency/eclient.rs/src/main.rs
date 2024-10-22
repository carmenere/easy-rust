//! There are many types of events sources we can track. The `mio` solves this by having `Registry::register` accept an object implementing the `Source` trait that `mio` defines.
//! As long as you implemet this trait for the source, you can use the event queue to track events on it.<br>
//! For simplicity we'll implement event queue only for one events sources, for `TcpStream`.<br>
//! There are 2 main abstractions over `epoll`. One is a structure called `Poll` and the other is called `Registry`.

/// This module contains all syscalls
mod ffi;

/// This module contains main abstraction over epoll
mod poll;

fn main() {
    println!("Hello, world!");
}
