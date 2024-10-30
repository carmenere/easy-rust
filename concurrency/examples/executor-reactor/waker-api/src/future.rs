pub use std::task::Poll;

// Let's for simplicity define our own `Future` type without waker
pub trait Future {
    type Output;

    fn poll(&mut self, fut_id: usize) -> Poll<Self::Output>;
}
