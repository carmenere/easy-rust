mod http;
mod future;
mod sequentially;
mod concurrently;

// Modelling the commented out code:
// async fn main() {
//     println!("Starting");
//     let req = Request::new("/", "HTTP/1.1","ya.ru:80", "close");
//     let resp = Http::get(req.clone()).await;
//     println!("Response 1: {}", resp);
//     let req = Request::new("/", "HTTP/1.1","google.com:80", "close");
//     let resp = Http::get(req.clone()).await;
//     println!("Response 2: {}", resp);
// }


// fn main() {
//     let mut future = sequentially::main();
// }

fn main() {
    let mut future = concurrently::main();
}
