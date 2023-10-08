use crate::protocol::{ByteScan, ByteScanner, ScanResult};
use bitter::BitReader;
use bytes::Bytes;
use derive_builder::Builder;

use super::{BitParseError, MessageError};

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct Flags {
    #[builder(default = "MessageType::Query")]
    message_type: MessageType,

    #[builder(default = "OpCode::Query")]
    op: OpCode,

    #[builder(default = "false")]
    authoritative_answer: bool,

    #[builder(default = "false")]
    truncation: bool,

    #[builder(default = "false")]
    recursion_desired: bool,

    #[builder(default = "false")]
    recursion_available: bool,

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
        self.recursion_available
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

impl ByteScanner for Flags {
    type Error = MessageError;

    fn try_scan(message: &[u8], cursor: usize) -> ScanResult<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(&message[cursor..]);
        let bits = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("MessageType".into()))?;

        let message_type = MessageType::try_from(bits as u8)?;

        let bits = reader
            .read_bits(4)
            .ok_or_else(|| MessageError::MalformedBits("OPCode".into()))? as u8;

        let op = OpCode::try_from(bits)?;

        let authoritative_answer = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("AuthoritativeAnswer".into()))?;

        let truncation = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("Truncation".into()))?;

        let recursion_desired = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("RecursionDesired".into()))?;

        let recursion_available = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("RecursionAvailable".into()))?;

        let _reserved = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("Reserved".into()))?;

        let answer_authenticated = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("AnswerAuthenticated".into()))?;

        let non_authenticated_data = reader
            .read_bit()
            .ok_or_else(|| MessageError::MalformedBits("NonAuthenticatedData".into()))?;

        let bits = reader
            .read_bits(4)
            .ok_or_else(|| MessageError::MalformedBits("RCode".into()))? as u8;

        let response = ResponseCode::try_from(bits)?;

        let flags = FlagsBuilder::default()
            .message_type(message_type)
            .op(op)
            .authoritative_answer(authoritative_answer)
            .truncation(truncation)
            .recursion_desired(recursion_desired)
            .recursion_available(recursion_available)
            .answer_authenticated(answer_authenticated)
            .non_authenticated_data(non_authenticated_data)
            .response(response)
            .build()?;

        Ok(ByteScan::new(flags, 2))
    }
}

impl From<Flags> for Bytes {
    fn from(flags: Flags) -> Self {
        (&flags).into()
    }
}

impl From<&Flags> for Bytes {
    fn from(flags: &Flags) -> Self {
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

        Bytes::copy_from_slice(&result.to_be_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Query = 0b0,
    Response = 0b1,
}

impl TryFrom<u8> for MessageType {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0 => Ok(Self::Query),
            0b1 => Ok(Self::Response),
            _ => Err(BitParseError::BadField("QR".to_owned(), value as u64)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    Query = 0b0000,
    IQuery = 0b0001,
    Status = 0b0010,
}

impl TryFrom<u8> for OpCode {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::Query),
            0b0001 => Ok(Self::IQuery),
            0b0010 => Ok(Self::Status),
            _ => Err(BitParseError::BadField("OPCODE".to_owned(), value as u64)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ResponseCode {
    NoError = 0b0000,
    FormatError = 0b0001,
    ServerFailure = 0b0010,
    NoDomain = 0b0011,
    NotImplemented = 0b1000,
    Refused = 0b0101,
}

impl TryFrom<u8> for ResponseCode {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::NoError),
            0b0001 => Ok(Self::FormatError),
            0b0010 => Ok(Self::ServerFailure),
            0b0011 => Ok(Self::NoDomain),
            0b1000 => Ok(Self::NotImplemented),
            0b0101 => Ok(Self::Refused),
            _ => Err(BitParseError::BadField("RCODE".to_owned(), value as u64)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn flags_simple_query_success() {
        let bytes = vec![0b0000_0000, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::Query);
        assert!(!flags.authoritative_answer());
        assert!(!flags.truncation());
        assert!(!flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_simple_response_success() {
        let bytes = vec![0b1000_0000, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();
        assert_eq!(flags.message_type(), &MessageType::Response);
        assert_eq!(flags.op(), &OpCode::Query);
        assert!(!flags.authoritative_answer());
        assert!(!flags.truncation());
        assert!(!flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_authoritative_answer_success() {
        let bytes = vec![0b0000_0100, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::Query);
        assert!(flags.authoritative_answer());
        assert!(!flags.truncation());
        assert!(!flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_truncation_success() {
        let bytes = vec![0b0000_0010, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::Query);
        assert!(!flags.authoritative_answer());
        assert!(flags.truncation());
        assert!(!flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_recursion_query_success() {
        let bytes = vec![0b0000_0001, 0b1000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");

        let flags = flags.value();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::Query);
        assert!(!flags.authoritative_answer());
        assert!(!flags.truncation());
        assert!(flags.recursion_desired());
        assert!(flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_bad_field_response_code() {
        let bytes = vec![0b0000_0000, 0b0000_1111];
        let flags = Flags::try_scan(&bytes, 0);
        assert!(flags.is_err());
    }

    #[test]
    fn flags_bad_field_op_code() {
        let bytes = vec![0b0111_1000, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0);
        assert!(flags.is_err());
    }

    #[test]
    fn flags_builder_to_binary_success() {
        let flags = FlagsBuilder::default()
            .message_type(MessageType::Query)
            .op(OpCode::Query)
            .authoritative_answer(false)
            .truncation(false)
            .recursion_desired(false)
            .recursion_available(false)
            .answer_authenticated(false)
            .non_authenticated_data(false)
            .response(ResponseCode::NoError)
            .build()
            .unwrap();

        let bytes = vec![0b0000_0000, 0b0000_0000];
        let to_bytes: Bytes = flags.into();
        assert_eq!(to_bytes, bytes);
    }

    #[test]
    fn flags_builder_query_to_binary_and_back_success() {
        let flags = FlagsBuilder::default()
            .message_type(MessageType::Query)
            .op(OpCode::IQuery)
            .authoritative_answer(false)
            .truncation(false)
            .recursion_desired(true)
            .recursion_available(false)
            .answer_authenticated(true)
            .non_authenticated_data(false)
            .response(ResponseCode::NoError)
            .build()
            .unwrap();

        let bytes: Bytes = flags.into();
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();

        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::IQuery);
        assert!(!flags.authoritative_answer());
        assert!(!flags.truncation());
        assert!(flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }

    #[test]
    fn flags_from_binary_and_back_success() {
        let bytes = vec![0b0000_1111, 0b0000_0000];
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();

        let bytes: Bytes = flags.into();
        let flags = Flags::try_scan(&bytes, 0).expect("Failed to scan flags");
        let flags = flags.value();

        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::IQuery);
        assert!(flags.authoritative_answer());
        assert!(flags.truncation());
        assert!(flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }
}
