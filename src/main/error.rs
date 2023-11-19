use std::io::Error;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug, Hash, Default)]
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
