use std::io::{ErrorKind, Write, Read};

use super::future::{Future,Poll};

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
}

impl HttpFuture {
    fn new(req: Request) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            req: req,
        }
    }

    fn write_request(&mut self) {
        let stream = std::net::TcpStream::connect(self.req.host.clone()).unwrap();
        stream.set_nonblocking(true);
        let mut stream = mio::net::TcpStream::from_std(stream);
        stream.write_all(self.req.get().as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}


impl Future for HttpFuture {
    type Output = String;
    
    fn poll(&mut self) -> Poll<Self::Output> {
        if self.stream.is_none() {
            self.write_request();
            self.stream.as_ref().map(|s| {
                let _ = s.peer_addr().map(|v| {
                    println!("Sending request to: {:?}", v)
                });
            });

            return Poll::Pending;
        }

        let mut buff = vec![0u8; 4096];

        loop {
            match self.stream.as_mut().unwrap().read(&mut buff) {
                Ok(0) => {
                    let s = String::from_utf8_lossy(&self.buffer);
                    break Poll::Ready(s.to_string())
                }
                Ok(n) => {
                    self.buffer.extend(&buff[0..n]);
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break Poll::Pending;
                }
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }
                Err(e) => panic!("{e:?}")
            }
        }
    }
    
}