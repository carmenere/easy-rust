# Example: custom timer

```rust
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::{Duration, Instant};

struct MyTimer {
    id: u32,
    expiration: Instant
}

impl Future for MyTimer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("It is timer {}.", self.id);
        // let now = Instant::now();
        if Instant::now() >= self.expiration {
            println!("Stop timer {}.", self.id);
            Poll::Ready(())
        }
        else {
            println!("Start timer {}.", self.id);
            let waker = cx.waker().clone();
            let e = self.expiration;

            thread::spawn(move || {
                thread::sleep(e - Instant::now());
                waker.wake();
            });
            
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main () {
    let timer1 = MyTimer {
        id: 1,
        expiration: Instant::now() + Duration::from_secs(2)
    };
    let timer2 = MyTimer {
        id: 2,
        expiration: Instant::now() + Duration::from_secs(5)
    };
    timer1.await; // Suspend execution until the result of a timer1 is ready.
    timer2.await;
}
```