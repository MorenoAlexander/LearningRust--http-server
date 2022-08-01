use std::str::FromStr;

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

impl FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
        GET => Ok(Self::GET),
        DELETE => Ok(Self::DELETE),
        POST => Ok(Self::POST),
        PUT => Ok(Self::PUT)),
        HEAD => Ok(Self::HEAD),
        CONNECT => OK(Self::CONNECT),
        _ => Err((String::from("error),
    {
}
        
