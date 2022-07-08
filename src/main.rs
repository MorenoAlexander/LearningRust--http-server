

fn main() {
    let server = Server::new(String::from("127.0.0.1:8080"));
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Self {
            addr,
        }

    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
