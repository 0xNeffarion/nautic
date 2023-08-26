use super::BitParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Class {
    IN = 1,
    Any = 255,
    CH = 3,
    HS = 4,
    NONE = 254,
}

impl TryFrom<u16> for Class {
    type Error = BitParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Class::IN),
            255 => Ok(Class::Any),
            3 => Ok(Class::CH),
            4 => Ok(Class::HS),
            254 => Ok(Class::NONE),
            _ => Err(BitParseError::BadField("Class".into(), value as u64)),
        }
    }
}
