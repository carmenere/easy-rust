# Example: tokio tasks

```rust
use std::time::Duration;
use tokio::time::sleep;

// To use the single-threaded runtime known as the current_thread runtime use #[tokio::main(flavor = "current_thread")]
// To use the multi-threaded runtime use #[tokio::main(flavor = "current_thread")]

// #[tokio::main(flavor = "current_thread")]
#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let mut h = vec![];
    for i in 0..=3 {
        let t = tokio::spawn(async move {
            task(i).await;
        });
        h.push(t)
    }

    for h in h {
        let _ = h.await.unwrap();
    }
}

async fn task(i: u32) {
    println!("[{i}] Strat task ... ");
    sleep(Duration::from_secs(i.into())).await;
    println!("[{i}] Task is completed.");
}
```