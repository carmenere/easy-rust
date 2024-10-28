use std::{thread, time::Duration};
use super::http::{Http, Request};
use super::future::{Future, Poll};

pub struct MainCoroutine {
    state: State,
    req1: Request,
    req2: Request,
}

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl MainCoroutine {
    pub fn new() -> Self {
        Self {
            state: State::Start,
            req1: Request::new("/", "HTTP/1.1","ya.ru:80", "close"),
            req2: Request::new("/", "HTTP/1.1","google.com:80", "close")
        }
    }
}

impl Future for MainCoroutine {
    type Output = ();
    
    fn poll(&mut self) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Coroutine starting");
                    let fut = Box::new(Http::get(self.req1.clone()));
                    self.state = State::Wait1(fut);
                },
                State::Wait1(ref mut fut) => {
                    match fut.poll() {
                        Poll::Ready(resp) => {
                            println!("Response 1: {}", resp);
                            let fut = Box::new(Http::get(self.req2.clone()));
                            self.state = State::Wait2(fut);
                        },
                        Poll::Pending => break Poll::Pending,
                    }
                },
                State::Wait2(ref mut fut) => {
                    match fut.poll() {
                        Poll::Ready(resp) => {
                            println!("Response 2: {}", resp);
                            self.state = State::Resolved;
                            break Poll::Ready(())
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

fn async_main() -> impl Future<Output=()> {
    MainCoroutine::new()
}

pub fn main() {
    let mut future = async_main();

    loop {
        match future.poll() {
            Poll::Ready(_) => break,
            Poll::Pending => {
                println!("Schedule other task");
            },
        }

        println!("Sleep ... ");
        thread::sleep(Duration::from_millis(100));
        println!("Wake up ... ");
    }
}
