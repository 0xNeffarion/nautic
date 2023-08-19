use std::ops::BitAnd;

use bytes::Bytes;
use derive_builder::Builder;

use crate::{errors::NauticDnsPacketQuestionError, flags::Flags};

#[derive(Debug, Clone, Builder)]
pub struct Header {
    #[builder(setter(into), default = "rand::random::<u16>()")]
    id: u16,
    flags: Flags,

    #[builder(setter(into), field(type = "u16"))]
    questions_size: u16,

    #[builder(setter(into), field(type = "u16"))]
    answers_size: u16,

    #[builder(setter(into), field(type = "u16"))]
    name_servers_size: u16,

    #[builder(setter(into), field(type = "u16"))]
    additional_size: u16,
}

#[derive(Debug, Clone)]
pub struct Packet {
    header: Header,
    questions: Vec<Question>,
}

#[derive(Debug, Clone)]
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

impl TryFrom<u64> for Question {
    type Error = NauticDnsPacketQuestionError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let mut cursor = value.reverse_bits();
        let mut name = String::new();

        loop {
            let length = (cursor & 0xFF) as u8;
            cursor >>= 8;

            if length == 0 {
                break;
            }

            for _ in 0..length {
                let char_bits = (cursor & 0xFF) as u8;
                let character = char::from(char_bits);
                name.push(character);

                cursor >>= 8;
            }
        }

        let qtype = (cursor & 0xFF_FF) as u16;
        let qtype = Record::try_from(qtype)?;

        cursor >>= 16;

        let class = (cursor & 0xFF_FF) as u16;

        Ok(Question { name, qtype, class })
    }
}

impl From<Question> for u64 {
    fn from(value: Question) -> Self {
        let mut cursor: u64 = 0;
        let mut result: u64 = 0;
        let mut result = Bytes::from(0);

        let labels = value.name.trim().split('.').collect::<Vec<&str>>();

        for label in labels {
            let length = label.len() as u64;

            result |= length << cursor;
            cursor += 8;

            for character in label.chars() {
                let char_bits = character as u8;
                result |= (char_bits as u64) << cursor;
                cursor += 8;
            }
        }

        result |= (0u8 << cursor) as u64;
        cursor += 8;

        result += ((value.qtype as u16) << cursor) as u64;

        cursor += 16;
        result += (value.class << cursor) as u64;

        result
    }
}

#[derive(Debug, Clone)]
#[repr(u16)]
pub enum Record {
    A = 1,
    AAAA = 28,
    CNAME = 5,
    MX = 15,
    NS = 2,
}

impl TryFrom<u16> for Record {
    type Error = NauticDnsPacketQuestionError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::A),
            28 => Ok(Self::AAAA),
            5 => Ok(Self::CNAME),
            15 => Ok(Self::MX),
            2 => Ok(Self::NS),
            _ => Err(NauticDnsPacketQuestionError::BadField(
                "QTYPE".to_owned(),
                value.to_string(),
            )),
        }
    }
}
