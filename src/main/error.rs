use std::fmt::{Debug, Formatter};
use std::io::Error;
use whdp::HttpParseError;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash, Default)]
pub struct WSSError(String);

impl WSSError {
    pub const fn new(str: String) -> Self {
        WSSError(str)
    }
}

impl From<Error> for WSSError {
    fn from(value: Error) -> Self {
        Self::new(value.to_string())
    }
}
impl From<HttpParseError> for WSSError{
    fn from(value: HttpParseError) -> Self {
        Self::new(value.to_string())
    }
}

impl Debug for WSSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}