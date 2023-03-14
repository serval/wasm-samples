use std::io::BufRead;

use bytes::Bytes;
use http::{self, Request};
use wasi_experimental_http;

fn make_default_req() -> Request<Option<Bytes>> {
    let req = http::request::Builder::new()
        .method(http::Method::POST)
        .uri("https://postman-echo.com/post")
        .header("Content-Type", "text/plain")
        .header("abc", "def");
    let b = Bytes::from("Testing with a request body. Does this actually work?");
    req.body(Some(b)).unwrap()
}

fn make_req(uri: &str) -> Request<Option<Bytes>> {
    let req = http::request::Builder::new()
        .method(http::Method::GET)
        .uri(uri)
        .header("User-Agent", "serval/wasm-samples/http-test");
    req.body(None).unwrap()
}

fn main() {
    let mut uri = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut uri)
        .expect("Failed to read stdin");
    let uri = uri.trim();
    let req = if !uri.is_empty() {
        make_req(uri)
    } else {
        make_default_req()
    };

    println!("Fetching {}", req.uri());
    let mut res = wasi_experimental_http::request(req).expect("cannot make request");
    let str = std::str::from_utf8(&res.body_read_all().unwrap())
        .unwrap()
        .to_string();
    println!(
        "Content-type header: {:#?}",
        res.header_get(String::from("Content-Type"))
    );
    println!("HTTP body: {}", str);
    println!("Status code: {:#?}", res.status_code);
}
