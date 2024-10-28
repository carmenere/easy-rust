pub use std::task::Poll;

// Let's for simplicity define our own `Future` type without waker
pub trait Future {
    type Output;

    fn poll(&mut self) -> Poll<Self::Output>;
}

pub struct JoinAll<F: Future> {
    futures: Vec<(bool, F)>,
    finished_count: usize,
}

pub fn join_all<F: Future>(futures: Vec<F>) -> JoinAll<F> {
    let futures = futures.into_iter().map(|f| (false, f)).collect();
    JoinAll{
        futures,
        finished_count: 0
    }
}

impl<F> Future for JoinAll<F>
where F: Future {
    type Output = ();

    fn poll(&mut self) -> Poll<Self::Output> {
        for (finished, fut) in self.futures.iter_mut() {
            if *finished {
                continue;
            }

            match fut.poll() {
                Poll::Ready(_) => {
                    *finished = true;
                    self.finished_count += 1;
                },
                Poll::Pending => continue,
            }
        }

        if self.finished_count == self.futures.len() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}