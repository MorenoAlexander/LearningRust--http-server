use http::{method::*, request::*};
use server::Server;

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

mod server {

    pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }

        pub fn run(self) {
            println!("Listening on {}", self.addr);
        }
    }
}

mod http {

    pub mod method {

        pub enum Method {
            GET,
            POST,
            PUT,
            DELETE,
            HEAD,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH,
        }
    }

    pub mod request {
        use super::method::Method;

        pub struct Request {
            path: String,
            method: Option<String>,
            query_string: Method,
        }
    }
}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
