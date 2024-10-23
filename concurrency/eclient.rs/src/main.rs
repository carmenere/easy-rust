//! There are many types of sources of events. In general source of events is specific to ech platform.<br>
//! The `mio` solves this by having `Registry::register` accept an object implementing the `Source` trait that `mio` defines.<br>
//! As long as you implemet this trait for the source, you can use the event queue to track events on it.<br>
//! For simplicity we'll implement event queue only for one events sources, for `TcpStream`.<br>
//! There are 2 main abstractions over `epoll`. One is a structure called `Poll` and the other is called `Registry`.

use std::{io::{self, Read, Result, Write}, net::TcpStream};

use ffi::Event;
use poll::Poll;

/// This module contains all syscalls
mod ffi;
/// This module contains main abstraction over epoll
mod poll;

struct HttpRequest {
    protocol: String,
    host: String,
    connection: String,
}

impl HttpRequest {
    const CRLF: &str = "\r\n";
    fn new(protocol: &str, host: &str, connection: &str) -> Self {
        Self {
            protocol: protocol.to_owned(),
            host: host.to_owned(),
            connection: connection.to_owned(),
        }
    }
    fn get(&self, path: &str) -> String {
        let CRLF = HttpRequest::CRLF;
        
        format!("GET {} {}{CRLF}Host: {}{CRLF}Connection: {}{CRLF}{CRLF}", path, self.protocol, self.host, self.connection)
    }
}

fn handle_events(events: &[Event], streams: &mut [TcpStream]) -> Result<usize> {
    let mut handled_events = 0;
    for event in events {
        let idx = event.token();
        let mut data = vec![0u8; 4096];

        loop {
            match streams[idx].read(&mut data) {
                Ok(n) if n == 0 => {
                    handled_events += 1;
                    break;
                }

                Ok(n) => {
                    let txt = String::from_utf8_lossy(&data[..n]);

                    println!("RECEIVED: {:?}", event);
                    println!("{txt}\n-----\n");
                }

                // `WouldBlock` means not ready to read in a non-blocking manner.
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                e @ Err(_) => return e,
            }
        }
    }

    Ok(handled_events)
}

fn main() -> Result<()>{
    let mut poll = Poll::new()?;
    let n_events = 5;

    let mut streams = vec![];
    let addr = "localhost:8080";

    for i in 0..n_events {
        let http = HttpRequest::new("HTTP/1.1","localhost", "close");
        let delay = (n_events - i) * 1000;
        let url = format!("/{delay}/request-{i}");
        let request = http.get(&url);

        let mut stream = TcpStream::connect(addr)?;
        stream.set_nonblocking(true)?;

        stream.write_all(request.as_bytes())?;

        poll.registry().register(&stream, i, ffi::EPOLLIN|ffi::EPOLLET)?;

        streams.push(stream);

        let mut handled_events = 0;

        while handled_events < n_events {
            let mut events = Vec::with_capacity(16);

            poll.poll(&mut events, None);

            if events.is_empty() {
                print!("Timeout or spurious event notification.");
                continue;
            }

            handled_events += handle_events(&events, &mut streams)?;
        }
    }

    println!("Finished.");
    Ok(())
}