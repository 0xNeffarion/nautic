use bitter::BitReader;
use derive_builder::Builder;

use super::{BitParseError, PacketError};

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct Flags {
    message_type: MessageType,
    op: OpCode,

    #[builder(default = "false")]
    authoritative_answer: bool,

    #[builder(default = "false")]
    truncation: bool,

    #[builder(default = "false")]
    recursion_desired: bool,

    #[builder(default = "false")]
    resursion_available: bool,

    #[builder(default = "false")]
    answer_authenticated: bool,

    #[builder(default = "false")]
    non_authenticated_data: bool,

    #[builder(default = "ResponseCode::NoError")]
    response: ResponseCode,
}

impl Flags {
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

    pub fn op(&self) -> &OpCode {
        &self.op
    }

    pub fn authoritative_answer(&self) -> bool {
        self.authoritative_answer
    }

    pub fn truncation(&self) -> bool {
        self.truncation
    }

    pub fn recursion_desired(&self) -> bool {
        self.recursion_desired
    }

    pub fn recursion_available(&self) -> bool {
        self.resursion_available
    }

    pub fn answer_authenticated(&self) -> bool {
        self.answer_authenticated
    }

    pub fn non_authenticated_data(&self) -> bool {
        self.non_authenticated_data
    }

    pub fn response(&self) -> &ResponseCode {
        &self.response
    }
}

impl TryFrom<Vec<u8>> for Flags {
    type Error = PacketError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes = [bytes[0], bytes[1]];
        let mut reader = bitter::BigEndianReader::new(&bytes);

        let bits = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("MessageType".into()))?;

        let message_type = MessageType::try_from(bits as u8)?;

        let bits = reader
            .read_bits(4)
            .ok_or_else(|| PacketError::MalformedBits("OPCode".into()))? as u8;

        let op = OpCode::try_from(bits)?;

        let authoritative_answer = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("AuthorativeAnswer".into()))?;

        let truncation = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("Truncation".into()))?;

        let recursion_desired = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("RecursionDesired".into()))?;

        let recursion_available = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("ResursionAvailable".into()))?;

        let _reserved = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("Reserved".into()))?;

        let answer_authenticated = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("AnswerAuthenticated".into()))?;

        let non_authenticated_data = reader
            .read_bit()
            .ok_or_else(|| PacketError::MalformedBits("NonAuthenticatedData".into()))?;

        let bits = reader
            .read_bits(4)
            .ok_or_else(|| PacketError::MalformedBits("RCODE".into()))? as u8;

        let response = ResponseCode::try_from(bits)?;

        let flags = FlagsBuilder::default()
            .message_type(message_type)
            .op(op)
            .authoritative_answer(authoritative_answer)
            .truncation(truncation)
            .recursion_desired(recursion_desired)
            .resursion_available(recursion_available)
            .answer_authenticated(answer_authenticated)
            .non_authenticated_data(non_authenticated_data)
            .response(response)
            .build()?;

        Ok(flags)
    }
}

impl From<Flags> for Vec<u8> {
    fn from(flags: Flags) -> Self {
        let mut result = 0u16;

        result |= (flags.message_type().clone() as u16) << 15;
        result |= (flags.op().clone() as u16) << 11;
        result |= (flags.authoritative_answer() as u16) << 10;
        result |= (flags.truncation() as u16) << 9;
        result |= (flags.recursion_desired() as u16) << 8;
        result |= (flags.recursion_available() as u16) << 7;

        // Reserved bit (position 6)
        result |= (flags.answer_authenticated() as u16) << 5;
        result |= (flags.non_authenticated_data() as u16) << 4;
        result |= flags.response().clone() as u16 & 0b1111;

        result.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Query = 0,
    Response = 1,
}

impl TryFrom<u8> for MessageType {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::Response),
            _ => Err(BitParseError::BadField("QR".to_owned(), value as u64)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    StandardQuery = 0,
    InverseQuery = 1,
    ServerStatus = 2,
}

impl TryFrom<u8> for OpCode {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::StandardQuery),
            1 => Ok(Self::InverseQuery),
            2 => Ok(Self::ServerStatus),
            _ => Err(BitParseError::BadField("OPCODE".to_owned(), value as u64)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NoDomain = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl TryFrom<u8> for ResponseCode {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerFailure),
            3 => Ok(Self::NoDomain),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err(BitParseError::BadField("RCODE".to_owned(), value as u64)),
        }
    }
}
