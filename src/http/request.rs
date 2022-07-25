use super::method::Method;

pub struct Request {
    path: String,
    method: Option<String>,
    query_string: Method,
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Self {
        todo!()
    }
}
