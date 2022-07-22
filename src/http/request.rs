
use super::method::Method;

pub struct Request {
    path: String,
    method: Option<String>,
    query_string: Method,
}
