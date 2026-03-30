# Table of contents
- [Table of contents](#table-of-contents)
- [Blocking mode](#blocking-mode)

<br>

# Blocking mode
To enable blocking client run `cargo add  reqwest --features blocking`.<br>

The instance of `Client` has **http methods**:
- `.post()`;
- `.put()`;
- `.get()`;
- `.delete()`;

<br>

The `.get()` method is pretty simple:
```rust
pub fn get<U: IntoUrl>(&self, url: U) -> RequestBuilder
```

This `IntoUrl` trait is one that the reqwest crate made, not the standard library, so you don’t have to remember it. But you can guess from the name that `IntoUrl` means anything that can become a URL, and it’s implemented for both `&str` and `String`.

<br>

**Example**:
```rust
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
    let resp = client.get("https://www.rust-lang.org").send().unwrap();
    println!("{:#?}", resp);
    println!("{}", &resp.text().unwrap()[0..200]);
}
```

**Output**:
```bash
Response {
    url: "https://rust-lang.org/",
    status: 200,
    headers: {
        "server": "GitHub.com",
        "content-type": "text/html; charset=utf-8",
        "x-origin-cache": "HIT",
        "last-modified": "Wed, 11 Mar 2026 01:43:58 GMT",
        "access-control-allow-origin": "*",
        "etag": "\"69b0c8de-48a9\"",
        "expires": "Wed, 11 Mar 2026 13:48:29 GMT",
        "cache-control": "max-age=600",
        "x-proxy-cache": "MISS",
        "x-github-request-id": "ADFE:25B76B:168270:16C29C:69B17055",
        "accept-ranges": "bytes",
        "age": "0",
        "date": "Wed, 11 Mar 2026 14:29:28 GMT",
        "via": "1.1 varnish",
        "x-served-by": "cache-ams2100130-AMS",
        "x-cache": "HIT",
        "x-cache-hits": "0",
        "x-timer": "S1773239369.631582,VS0,VE122",
        "vary": "Accept-Encoding",
        "x-fastly-request-id": "71d736843c4c7171f06a73f48be8d9c7674958cc",
        "content-length": "18601",
    },
}
<!doctype html>
<html lang="en-US">
  <head>
    <meta charset="utf-8">
    <title>
            Rust Programming Language
        </title>
    <meta name="viewport" content="width=device-width,initial
```

<br>

The [`Response` struct](https://docs.rs/reqwest/latest/reqwest/struct.Response.html) has methods
- `.status()`
- `.content_length()`
- `.text()`: it gives a `Result<String>`, where `String` is a **body** of response;

<br>
