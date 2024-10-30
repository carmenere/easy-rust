use std::io::{ErrorKind, Write, Read};
use mio::{net::{TcpListener, TcpStream}, Interest, Token};
use crate::runtime::{self, reactor};
use super::future::{Future,Poll};
use crate::runtime::{waker::Waker, reactor::reactor};

#[derive(Clone)]
pub struct Request {
    pub path: String,
    pub protocol: String,
    pub host: String,
    pub connection: String,
}

impl Request {
    const CRLF: &str = "\r\n";
    pub fn new(path: &str, protocol: &str, host: &str, connection: &str) -> Self {
        Self {
            path: path.to_owned(),
            protocol: protocol.to_owned(),
            host: host.to_owned(),
            connection: connection.to_owned(),
        }
    }
    fn get(&self) -> String {
        let CRLF = Request::CRLF;
        
        format!("GET {} {}{CRLF}Host: {}{CRLF}Connection: {}{CRLF}{CRLF}", self.path, self.protocol, self.host, self.connection)
    }
}

pub struct Http;

impl Http {
    pub fn get(req: Request) -> impl Future<Output = String> {
        HttpFuture::new(req)
    }
}

struct HttpFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    req: Request,
    id: usize,
}

impl HttpFuture {
    fn new(req: Request) -> Self {
        // get new id for leaf future, then this id is passed to reactor().register() method
        let id = reactor().next_id();
        Self {
            stream: None,
            buffer: vec![],
            req: req,
            id
        }
    }

    fn write_request(&mut self, stream: std::net::TcpStream) {
        println!("Enter write_request().");
        stream.set_nonblocking(true).unwrap();
        let mut stream = mio::net::TcpStream::from_std(stream);
        stream.write_all(self.req.get().as_bytes()).unwrap();
        self.stream = Some(stream);
        println!("Exit write_request().");

    }
}

impl Future for HttpFuture {
    type Output = String;
    
    fn poll(&mut self, waker: &Waker) -> Poll<Self::Output> {
        println!("First poll of HttpFuture.");
        if self.stream.is_none() {
            println!("call connect(\"{}\")", self.req.host);
            let stream: Result<std::net::TcpStream, std::io::Error> = std::net::TcpStream::connect(self.req.host.clone());

            if stream.is_err() {
                let _ = stream.as_ref().map_err(|e| {
                    println!("Error: {:?}", e);
                });
                return Poll::Ready("NO ANSWER".to_owned())
            }

            let _ = stream.map(|stream| {
                self.write_request(stream);
            });

            self.stream.as_ref().map(|s| {
                let _ = s.peer_addr().map(|v| {
                    println!("Sending request to: {:?}", v)
                });
            });

            reactor()
                .register(self.stream.as_mut().unwrap(), Interest::READABLE, self.id);

            reactor().add_waker(waker, self.id);
            return Poll::Pending;
        }

        let mut buff = vec![0u8; 4096];

        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    let _ = reactor().deregister(self.stream.as_mut().unwrap(), self.id);
                    break Poll::Ready(s.to_string())
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // the Waker from the most recent call should be scheduled to wake up
                    // the reason is that the future could have moved to a different executor in between calls, and we need to wake up the correct one
                    reactor().add_waker(waker, self.id);
                    break Poll::Pending;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) if e.kind() == ErrorKind::TimedOut => {
                    break Poll::Ready("ErrorKind::TimedOut".to_owned());
                }
                Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                    break Poll::Ready("ErrorKind::ConnectionReset".to_owned());
                }
                Err(e) => panic!("{e:?}")
            }
        }
    }
    
}