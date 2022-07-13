

fn main() {

    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Self {
            addr,
        }

    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH 
}

struct Request {
    path: String,
    method: Option<String>,
    query_string: Method,

}

/*
GET /user?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
 */
