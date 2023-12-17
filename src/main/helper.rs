use whdp::{Request, Response};

pub type HTTPFunction = fn(Request) -> Response;
