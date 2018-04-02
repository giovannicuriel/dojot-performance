extern crate hyper;
extern crate futures;

mod http_listener;

fn main() {
    http_listener::start_server();
}