use bitter::BitReader;
use derive_builder::Builder;

use super::{BitParseError, PacketError};

#[derive(Debug, Clone, PartialEq, Eq, Builder)]
pub struct Flags {
    #[builder(default = "MessageType::Query")]
    message_type: MessageType,

    #[builder(default = "OpCode::StandardQuery")]
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
    StandardQuery = 0b0000,
    InverseQuery = 0b0001,
    ServerStatus = 0b0010,
}

impl TryFrom<u8> for OpCode {
    type Error = BitParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(Self::StandardQuery),
            0b0001 => Ok(Self::InverseQuery),
            0b0010 => Ok(Self::ServerStatus),
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
        let flags = Flags::try_from(bytes).unwrap();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::StandardQuery);
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
        let flags = Flags::try_from(bytes).unwrap();
        assert_eq!(flags.message_type(), &MessageType::Response);
        assert_eq!(flags.op(), &OpCode::StandardQuery);
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
        let flags = Flags::try_from(bytes).unwrap();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::StandardQuery);
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
        let flags = Flags::try_from(bytes).unwrap();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::StandardQuery);
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
        let flags = Flags::try_from(bytes).unwrap();
        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::StandardQuery);
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
        let flags = Flags::try_from(bytes);
        assert!(flags.is_err());
    }

    #[test]
    fn flags_bad_field_op_code() {
        let bytes = vec![0b0111_1000, 0b0000_0000];
        let flags = Flags::try_from(bytes);
        assert!(flags.is_err());
    }

    #[test]
    fn flags_builder_to_binary_success() {
        let flags = FlagsBuilder::default()
            .message_type(MessageType::Query)
            .op(OpCode::StandardQuery)
            .authoritative_answer(false)
            .truncation(false)
            .recursion_desired(false)
            .resursion_available(false)
            .answer_authenticated(false)
            .non_authenticated_data(false)
            .response(ResponseCode::NoError)
            .build()
            .unwrap();

        let bytes = vec![0b0000_0000, 0b0000_0000];
        assert_eq!(Vec::<u8>::from(flags), bytes);
    }

    #[test]
    fn flags_builder_query_to_binary_and_back_success() {
        let flags = FlagsBuilder::default()
            .message_type(MessageType::Query)
            .op(OpCode::InverseQuery)
            .authoritative_answer(false)
            .truncation(false)
            .recursion_desired(true)
            .resursion_available(false)
            .answer_authenticated(true)
            .non_authenticated_data(false)
            .response(ResponseCode::NoError)
            .build()
            .unwrap();

        let bytes = Vec::<u8>::from(flags);
        let flags = Flags::try_from(bytes).unwrap();

        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::InverseQuery);
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
        let flags = Flags::try_from(bytes).unwrap();

        let bytes = Vec::<u8>::from(flags);
        let flags = Flags::try_from(bytes).unwrap();

        assert_eq!(flags.message_type(), &MessageType::Query);
        assert_eq!(flags.op(), &OpCode::InverseQuery);

        assert!(flags.authoritative_answer());
        assert!(flags.truncation());
        assert!(flags.recursion_desired());
        assert!(!flags.recursion_available());
        assert!(!flags.answer_authenticated());
        assert!(!flags.non_authenticated_data());
        assert_eq!(flags.response(), &ResponseCode::NoError);
    }
}
