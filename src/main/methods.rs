use whdp::HttpMethod;
use whdp::resp_presets::not_found;

use crate::helper::HTTPFunction;
#[derive(Copy, Clone,PartialOrd, PartialEq,Ord, Eq,Debug,Hash)]
pub enum Methods {
    Get(HTTPFunction),
    Post(HTTPFunction),
    Options(HTTPFunction),
    Connect(HTTPFunction),
    Delete(HTTPFunction),
    Put(HTTPFunction),
    Patch(HTTPFunction),
    Head(HTTPFunction),
    Trace(HTTPFunction),
}

impl Default for Methods{
    fn default() -> Self {
        Self::Get(|_|not_found(""))
    }
}

impl Methods {
    pub const fn get_inner(self) -> HTTPFunction {
        match self {
            Methods::Get(s) => s,
            Methods::Post(s) => s,
            Methods::Connect(s) => s,
            Methods::Delete(s) => s,
            Methods::Head(s) => s,
            Methods::Patch(s) => s,
            Methods::Trace(s) => s,
            Methods::Options(s) => s,
            Methods::Put(s) => s,
        }
    }
    pub const fn get_type(&self) -> HttpMethod {
        match self {
            Methods::Get(_) => HttpMethod::Get,
            Methods::Post(_) => HttpMethod::Post,
            Methods::Connect(_) => HttpMethod::Connect,
            Methods::Delete(_) => HttpMethod::Delete,
            Methods::Head(_) => HttpMethod::Head,
            Methods::Patch(_) => HttpMethod::Patch,
            Methods::Trace(_) => HttpMethod::Trace,
            Methods::Options(_) => HttpMethod::Options,
            Methods::Put(_) => HttpMethod::Put,
        }
    }
}
