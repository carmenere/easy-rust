mod http;
mod future;
mod request;
mod runtime; 

use request::RequestFuture;
use runtime::Runtime;

pub fn main() {
    let fut1 = RequestFuture::new("/", "HTTP/1.1","ya.ru:80", "close");
    let fut2 = RequestFuture::new("/", "HTTP/1.1","google.com:80", "close");

    let mut rt = Runtime::new();

    rt.spawn(fut1);
    rt.spawn(fut2);

    rt.run();
}
