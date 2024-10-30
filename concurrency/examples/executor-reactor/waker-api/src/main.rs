mod http;
mod future;
mod request;
mod runtime; 
mod domains;

use request::RequestFuture;

pub fn main() {
    let mut executor = runtime::init();

    for domain in domains::domains() {
        executor.spawn(RequestFuture::new("/", "HTTP/1.1",&format!("{}:80", domain), "close"));
    }

    executor.block_on();
}
