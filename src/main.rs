#![allow(dead_code)]
use server::Server;
use crate::website_handler::WebsiteHandler;

mod http;

mod server;
mod website_handler;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run(WebsiteHandler);
}
