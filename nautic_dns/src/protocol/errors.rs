use thiserror::*;

use super::{flags::FlagsBuilderError, HeaderBuilderError};

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Failed to read bits from packet at: {0}")]
    MalformedBits(String),

    #[error("Failed to build packet header flags: {0}")]
    FlagsBuilderValidationFailure(#[from] FlagsBuilderError),

     #[error("Failed to build packet header: {0}")]
    HeaderBuilderValidationFailure(#[from] HeaderBuilderError),

    #[error("Failed to read flag field: {0}")]
    FlagFieldMalformed(#[from] BitParseError),
}

#[derive(Debug, Error)]
pub enum BitParseError {
    #[error("Bad field value found. Field: '{0}'. Value: '{1}'")]
    BadField(String, u64),

    #[error("Failure while attempting to read bits at entry: '{0}'")]
    MalformedBits(String),
}

#[derive(Debug, Error)]
pub enum TransportPacketRecordsError {
    #[error("Failed to recognize packet record type: {0}")]
    UnknownRecord(u16),
}
