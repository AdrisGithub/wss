use wjp::{map, ParseError, Serialize, SerializeHelper, Values};

#[derive(Debug)]
pub struct POST(pub String);
#[derive(Debug)]
pub struct GET(pub String, pub usize);


impl Serialize for POST {
    fn serialize(&self) -> Values {
        Values::Struct(map!(("message",self.0.serialize())))
    }
}

impl Serialize for GET {
    fn serialize(&self) -> Values {
        Values::Struct(map!(("message",self.0.serialize()),("id",self.1.serialize())))
    }
}

impl TryFrom<Values> for POST {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        Ok(Self(struc.map_val("message", String::try_from)?))
    }
}

impl TryFrom<Values> for GET {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        Ok(Self(
            struc.map_val("message", String::try_from)?,
            struc.map_val("id", usize::try_from)?,
        ))
    }
}