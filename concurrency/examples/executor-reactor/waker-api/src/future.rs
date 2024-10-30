pub use std::task::Poll;
use crate::runtime::waker::Waker;

// Let's for simplicity define our own `Future` type without waker
pub trait Future {
    type Output;

    fn poll(&mut self, waker: &Waker) -> Poll<Self::Output>;
}
