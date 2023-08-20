use bitter::BitReader;
use bytes::BufMut;

use super::{records::*, BitParseError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    name: String,
    qtype: Record,
    class: u16,
}

impl Question {
    pub fn new(name: String, qtype: Record, class: u16) -> Self {
        Self { name, qtype, class }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn qtype(&self) -> &Record {
        &self.qtype
    }

    pub fn class(&self) -> u16 {
        self.class
    }
}

impl TryFrom<Vec<u8>> for Question {
    type Error = BitParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(&value);
        let mut name = String::new();

        for i in 0.. {
            // Read the the lenght of the next label
            let length = reader
                .read_u8()
                .ok_or_else(|| BitParseError::MalformedBits("Label Lenght".into()))?;

            // If the length is 0, we've reached the end of the domain name. Othewise add a '.' to the name if it isnt the first iteration
            if length == 0 {
                break;
            } else if i != 0 {
                name.push('.');
            }

            for _ in 0..length {
                let char_bits = reader
                    .read_u8()
                    .ok_or_else(|| BitParseError::MalformedBits("Label Character".into()))?;

                name.push(char_bits as char);
            }
        }

        let qtype: Record = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Record Type".into()))?
            .try_into()?;

        let class = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Class Type".into()))?;

        Ok(Question { name, qtype, class })
    }
}

impl From<Question> for Vec<u8> {
    fn from(value: Question) -> Self {
        let mut buffer = vec![];

        let labels = value.name.trim().split('.').collect::<Vec<&str>>();

        for label in labels {
            let length = label.len() as u8;
            buffer.put_u8(length);

            let chars = label.chars().collect::<Vec<char>>();
            for i in 0..length {
                buffer.put_u8(chars[i as usize] as u8);
            }
        }

        buffer.put_u8(0);
        buffer.put_u16(value.qtype as u16);
        buffer.put_u16(value.class);

        buffer
    }
}
