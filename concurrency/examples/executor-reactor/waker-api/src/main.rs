mod http;
mod future;
mod request;
mod runtime; 

use request::RequestFuture;

pub fn main() {
    let fut1 = RequestFuture::new("/", "HTTP/1.1","ya.ru:80", "close");
    let fut2 = RequestFuture::new("/", "HTTP/1.1","google.com:80", "close");

    let mut executor = runtime::init();

    executor.spawn(fut1);
    executor.spawn(fut2);

    executor.block_on();
}
