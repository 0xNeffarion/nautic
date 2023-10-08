use super::{types::*, Class};
use crate::protocol::{BitParseError, ByteScanner, LabelSequence, ScanResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub name: LabelSequence,
    pub r#type: RecordType,
    pub class: Class,
    pub ttl: u32,
    pub length: u16,
    pub data: String,
}

impl Record {
    pub fn new(
        name: LabelSequence,
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
        self.name.label()
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

impl ByteScanner for Record {
    type Error = BitParseError;

    fn try_scan(message: &[u8], cursor: usize) -> ScanResult<Self, Self::Error> {
        let scan = LabelSequence::try_scan(message, cursor)?;
        let label = scan.value();
        let scan_length = scan.total_bytes;

        let value = &message[cursor + scan_length..];
        let r#type = RecordType::try_from(u16::from_be_bytes([value[0], value[1]]))?;
        let class = Class::try_from(u16::from_be_bytes([value[2], value[3]]))?;
        let ttl = u32::from_be_bytes([value[4], value[5], value[6], value[7]]);
        let length = u16::from_be_bytes([value[8], value[9]]);
        let data = decode_rdata(&class, &r#type, message, cursor + name_len + 10)?;

        Ok((
            Self {
                name,
                r#type,
                class,
                ttl,
                length,
                data,
            },
            name_len + 10 + length as usize,
        ))
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
