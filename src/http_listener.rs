extern crate hyper;
extern crate futures;

use futures::future::Future;


pub struct Notification;

impl hyper::server::Service for Notification {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: hyper::Request) -> Self::Future {
        Box::new(futures::future::ok(
            hyper::Response::new()
        ))
    }
}

pub fn start_server() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = hyper::server::Http::new().bind(&addr, || Ok(Notification)).unwrap();
    server.run().unwrap();
}
