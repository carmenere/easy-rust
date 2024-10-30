use super::http::{Http, Request};
use super::future::{Future, Poll} ;
use crate::runtime::waker::Waker;

pub struct RequestFuture {
    state: State,
    req: Request,
}

enum State {
    Start,
    Wait(Box<dyn Future<Output = String>>),
    Resolved,
}

impl RequestFuture {
    pub fn new(path: &str, protocol: &str, host: &str, connection: &str) -> Self {
        Self {
            state: State::Start,
            req: Request::new(path, protocol, host, connection),
        }
    }
}

impl Future for RequestFuture {
    type Output = String;
    
    fn poll(&mut self, waker: &Waker) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("First poll of RequestFuture.");
                    let fut = Box::new(Http::get(self.req.clone()));
                    self.state = State::Wait(fut);
                },
                State::Wait(ref mut fut) => {
                    println!("Second poll of RequestFuture.");
                    match fut.poll(waker) {
                        Poll::Ready(resp) => {
                            self.state = State::Resolved;
                            break Poll::Ready(resp)
                        },
                        Poll::Pending => break Poll::Pending,
                    }
                },
                State::Resolved => {
                    panic!("The resolved future is polled!")
                },
            }
        }
    }
}
