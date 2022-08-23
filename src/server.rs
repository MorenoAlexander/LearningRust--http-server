use crate::http::{response, Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub trait Handler {
    fn handle_request(&mut self, req: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Error parsing request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler : impl Handler) -> ! {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handle_request(&request)

                                }
                                Err(e) => {
                                    dbg!(&e);
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Error sending response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
