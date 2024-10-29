use std::{thread, time::Duration};

use super::http::{Http, Request};
use super::future::{Future, Poll, join_all} ;

struct ReqCoroutine {
    state: State,
    req: Request,
}

enum State {
    Start,
    Wait(Box<dyn Future<Output = String>>),
    Resolved,
}

impl ReqCoroutine {
    fn new(path: &str, protocol: &str, host: &str, connection: &str) -> Self {
        Self {
            state: State::Start,
            req: Request::new(path, protocol, host, connection),
        }
    }
}

impl Future for ReqCoroutine {
    type Output = ();
    
    fn poll(&mut self) -> Poll<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("coroutine starting");
                    let fut = Box::new(Http::get(self.req.clone()));
                    self.state = State::Wait(fut);
                },
                State::Wait(ref mut fut) => {
                    match fut.poll() {
                        Poll::Ready(resp) => {
                            println!("Response: {}", resp);
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
    let mut futures = vec![];
    futures.push(ReqCoroutine::new("/", "HTTP/1.1","ya.ru:80", "close"));
    futures.push(ReqCoroutine::new("/", "HTTP/1.1","google.com:80", "close"));
    join_all(futures)
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
