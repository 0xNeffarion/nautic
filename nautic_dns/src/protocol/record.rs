use super::{types::*, BitParseError, Class};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    name: String,
    r#type: RecordType,
    class: Class,
    ttl: u32,
    length: u16,
    data: String,
}

impl Record {
    pub fn new(
        name: String,
        r#type: RecordType,
        class: Class,
        ttl: u32,
        length: u16,
        data: String,
    ) -> Self {
        Self {
            name,
            r#type,
            class,
            ttl,
            length,
            data,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> &RecordType {
        &self.r#type
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn ttl(&self) -> u32 {
        self.ttl
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

// impl TryFrom<&Vec<u8>> for Record {
//     type Error = BitParseError;

//     fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
//         let mut reader = bitter::BigEndianReader::new(&value);
//         let mut name = String::new();

//         let r#type: RecordType = reader
//             .read_u16()
//             .ok_or_else(|| BitParseError::MalformedBits("Record Type".into()))?
//             .try_into()?;

//         let class = reader
//             .read_u16()
//             .ok_or_else(|| BitParseError::MalformedBits("Class Type".into()))?;

//         let class: Class = class.try_into()?;

//         let ttl = reader
//             .read_u32()
//             .ok_or_else(|| BitParseError::MalformedBits("TTL".into()))?;

//         let length = reader
//             .read_u16()
//             .ok_or_else(|| BitParseError::MalformedBits("Length".into()))?;

//         let data = todo!();

//         Ok(Record::new(
//             name,
//             r#type,
//             class,
//             ttl,
//             length,
//             data,
//         ))
//     }
// }

// impl From<Record> for Vec<u8> {
//     fn from(value: Record) -> Self {
//         let mut buffer = vec![];

//         let labels = value.name.trim().split('.').collect::<Vec<&str>>();

//         for label in labels {
//             let length = label.len() as u8;
//             buffer.put_u8(length);

//             let chars = label.chars().collect::<Vec<char>>();
//             for i in 0..length {
//                 buffer.put_u8(chars[i as usize] as u8);
//             }
//         }

//         buffer.put_u8(0);
//         buffer.put_u16(value.r#type as u16);
//         buffer.put_u16(value.class);

//         buffer
//     }
// }
