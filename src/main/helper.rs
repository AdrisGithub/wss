use whdp::{Request, Response};

pub type HTTPFunction = fn(Request) -> Response;
pub type RawFunction = fn(String) -> String;
pub type SerializableFunction<I, O> = fn(I) -> O;
