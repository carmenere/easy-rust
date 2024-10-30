use std::{sync::OnceLock, time::Duration};
// use super::future::Future;
use mio::{Poll, Registry, Events};

pub mod executor;
pub mod reactor;
pub mod waker;

use executor::Executor;

pub fn init() -> Executor {
    reactor::start();
    Executor::new()
}

// static REGISTRY: OnceLock<Registry> = OnceLock::new();

// pub fn registry() -> &'static Registry {
//     REGISTRY.get().expect("Runtime has not initialized yet.")
// }

// pub struct Runtime {
//     poll: Poll,
//     futures: Vec<Box<dyn Future<Output = String>>>
// }

// impl Runtime {
//     pub fn new() -> Self {
//         let poll = Poll::new().unwrap();
//         let registry = poll.registry().try_clone().unwrap();
//         let _ = REGISTRY.set(registry);
//         Self { 
//             poll,
//             futures: vec![]
//         }
//     }

//     pub fn spawn<F>(&mut self, fut: F) 
//     where F: Future<Output = String> + 'static {
//         self.futures.push(Box::new(fut));
//     }

//     pub fn run(&mut self)
//     {
//         for (fut_id, fut) in (&mut self.futures).into_iter().enumerate() {
//             match fut.poll(fut_id) {
//                 std::task::Poll::Ready(s) => {
//                     println!("Ready: response = {}", s)
//                 },
//                 std::task::Poll::Pending => {
//                     println!("Pending")
//                 },
//             }
//         };

//         loop {
//             let mut events: Events = Events::with_capacity(8);
//             let _ = self.poll.poll(&mut events, Some(Duration::from_millis(1)));

//             for e in events.into_iter() {
//                 let fut_id: usize = e.token().into();

//                 let fut = &mut self.futures[fut_id];
                
//                 match fut.poll(fut_id) {
//                     std::task::Poll::Ready(s) => {
//                         println!("Ready: response = {}", s)
//                     },
//                     std::task::Poll::Pending => {
//                         println!("Pending")
//                     },
//                 }
//             }
//         }
//     }
// }