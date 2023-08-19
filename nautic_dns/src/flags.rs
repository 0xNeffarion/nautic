use derive_builder::Builder;

use crate::errors::NauticDnsPacketFlagError;

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

impl TryFrom<u16> for Flags {
    type Error = NauticDnsPacketFlagError;

    fn try_from(bits: u16) -> Result<Self, Self::Error> {
        let mut cursor = bits.reverse_bits();

        let message_type = MessageType::try_from(cursor & 0b1)?;
        cursor >>= 1;

        let op = OpCode::try_from(cursor & 0b1111)?;
        cursor >>= 4;

        let authoritative_answer = cursor & 0b1 != 0;
        cursor >>= 1;

        let truncation = cursor & 0b1 != 0;
        cursor >>= 1;

        let recursion_desired = cursor & 0b1 != 0;
        cursor >>= 1;

        let recursion_available = cursor & 0b1 != 0;
        cursor >>= 1;

        let _reserved = cursor & 0b1 != 0;
        cursor >>= 1;

        let answer_authenticated = cursor & 0b1 != 0;
        cursor >>= 1;

        let non_authenticated_data = cursor & 0b1 != 0;
        cursor >>= 1;

        let response = ResponseCode::try_from(cursor & 0b1111)?;

        FlagsBuilder::default()
            .message_type(message_type)
            .op(op)
            .authoritative_answer(authoritative_answer)
            .truncation(truncation)
            .recursion_desired(recursion_desired)
            .resursion_available(recursion_available)
            .answer_authenticated(answer_authenticated)
            .non_authenticated_data(non_authenticated_data)
            .response(response)
            .build()
            .map_err(NauticDnsPacketFlagError::FlagBuilderFailure)
    }
}

impl From<Flags> for u16 {
    fn from(flags: Flags) -> Self {
        let mut cursor = 0;
        let mut result = 0;

        result |= (flags.message_type.clone() as u16) << cursor;
        cursor += 1;

        result |= (flags.op.clone() as u16) << cursor;
        cursor += 4;

        result |= (flags.authoritative_answer as u16) << cursor;
        cursor += 1;

        result |= (flags.truncation as u16) << cursor;
        cursor += 1;

        result |= (flags.recursion_desired as u16) << cursor;
        cursor += 1;

        result |= (flags.resursion_available as u16) << cursor;
        cursor += 1;

        // Reserved bit
        result |= 0 << cursor;
        cursor += 1;

        result |= (flags.answer_authenticated as u16) << cursor;
        cursor += 1;

        result |= (flags.non_authenticated_data as u16) << cursor;
        cursor += 1;

        result |= (flags.response.clone() as u16) << cursor;

        result.reverse_bits()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum MessageType {
    Query = 0,
    Response = 1,
}

impl TryFrom<u16> for MessageType {
    type Error = NauticDnsPacketFlagError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::Response),
            _ => Err(NauticDnsPacketFlagError::BadField(
                "QR".to_owned(),
                value.to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum OpCode {
    StandardQuery = 0,
    InverseQuery = 1,
    ServerStatus = 2,
}

impl TryFrom<u16> for OpCode {
    type Error = NauticDnsPacketFlagError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::StandardQuery),
            1 => Ok(Self::InverseQuery),
            2 => Ok(Self::ServerStatus),
            _ => Err(NauticDnsPacketFlagError::BadField(
                "OPCODE".to_owned(),
                value.to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NoDomain = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl TryFrom<u16> for ResponseCode {
    type Error = NauticDnsPacketFlagError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerFailure),
            3 => Ok(Self::NoDomain),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err(NauticDnsPacketFlagError::BadField(
                "RCODE".to_owned(),
                value.to_string(),
            )),
        }
    }
}
