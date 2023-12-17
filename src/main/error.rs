use std::fmt::{Debug, Display, Formatter};

pub struct WBSLError;

impl Debug for WBSLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WBSL ERROR")
    }
}

impl Display for WBSLError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
