use whdp::HttpMethod;
use wjp::{Deserialize, Serialize};

use crate::helper::{HTTPFunction, RawFunction};
use crate::ser_servlet::SerializeServlet;

pub enum Methods<I: Deserialize, O: Serialize> {
    Get(HTTPFunction),
    GetRaw(RawFunction),
    GetSer(SerializeServlet<I, O>),
    Post(HTTPFunction),
    PostRaw(RawFunction),
    PostSer(SerializeServlet<I,O>),
}

impl<I: Deserialize, O: Serialize> Methods<I, O> {
    pub fn get_inner(self) -> HTTPFunction {
        match self {
            Methods::Get(s) => s,
        }
    }
    pub fn get_type(&self) -> HttpMethod {
        match self {
            Methods::Get(_) => HttpMethod::Get,
        }
    }
}
