use super::BitParseError;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum RecordType {
    A = 1,
    AAAA = 28,
    CNAME = 5,
    MX = 15,
    NS = 2,
}

impl TryFrom<u16> for RecordType {
    type Error = BitParseError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            28 => Ok(Self::AAAA),
            5 => Ok(Self::CNAME),
            15 => Ok(Self::MX),
            2 => Ok(Self::NS),
            _ => Err(BitParseError::BadField("Record Type".into(), value as u64)),
        }
    }
}
