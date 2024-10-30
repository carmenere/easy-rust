use std::thread;

mod http;
mod future;
mod request;
mod runtime; 
mod domains;

use request::RequestFuture;
use runtime::{executor::Executor, reactor};

pub fn main() {
    let _ = reactor::start();

    // We can run multiple executors each in separate kernel thread, then we get concurrency + parallelism
    let domains = domains::domains();
    let n_cores = 6;
    let chunk = domains.len()/n_cores;

    let mut handlers = vec![];

    println!("Number of domains = {}, N = {}, chunk = {}", domains.len(), n_cores, chunk);

    for i in 0..n_cores {
        let h = thread::spawn(move || {
            let tid = thread::current().id();
            println!("Thread with id {tid:?} has started.");
            let executor = Executor::new();
            let min = i*chunk;
            let max = min + chunk;
            for domain in domains::domains()[min..max].iter() {
                executor.spawn(RequestFuture::new("/", "HTTP/1.1",&format!("{}:80", domain), "close"));
                executor.block_on();
            }
        });

        handlers.push(h);
    }

    handlers.into_iter().for_each(|h| {
        h.join().unwrap();
    });
}
