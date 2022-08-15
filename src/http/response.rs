use std::fmt::Display;

pub use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> {
        todo!()
    }
}
