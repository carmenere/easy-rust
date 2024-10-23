# TcpStream
**Example**:
```rust
use std::{io::{self, Read, Result, Write}, net::TcpStream};

let mut stream = TcpStream::connect(addr)?;
stream.set_nonblocking(true)?;
```

<br>

The [std::net::TcpStream](https://doc.rust-lang.org/std/net/struct.TcpStream.html) in Rust accept anything that implements [std::net::ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html). This trait is implemented for `&str`.<br>

Before `TcpStream::connect` actually opens a socket
- it will parse the address as an **IP address**;
- if it fails, then it will parse the address as **domain name** and then ask the OS to **resolve** this domain name and this is *potentially* **blocking** operation;

<br>

**By default** `std::net::TcpStream` create **stream** with the `TCP_NODELAY` **flag** set to `false`. It means that **Nagle's** algorithm is **used**, which can cause some issues with latency.<br>
Many other **non-blocking I/O implementations** in other languages, it not most, **disable** this algorithm **by default**.<br>
You can disable it by calling `TcpStream::set_nodelay(true)?;`.<br>

<br>

**By default** `std::net::TcpStream` create **stream** in **blocking** mode.<br>
You can set `TcpStream` to **non-blocking** ba calling `TcpStream::set_nonblocking(true)?;`.<br>