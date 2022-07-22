mod server;

use http::{method::*, request::*};
use server::Server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
