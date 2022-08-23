use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, req: &Request) -> Response {
        dbg!(req);
        Response::new(StatusCode::Ok, Some(String::from("<h1>Hello World</h1>")))
    }
}