use thiserror::*;
use tokio::io;

use crate::flags::FlagsBuilderError;

#[derive(Debug, Error)]
pub enum NauticDnsError {
    #[error("Connection to the server has failed")]
    ConnectionFailure(#[from] io::Error),

    #[error("Failed to bind server to hostname")]
    ServerBindingFailure,

    #[error("Failed to parse target url")]
    InvalidTargetParse(#[from] url::ParseError),

    #[error("Target is not a valid hostname: {0}")]
    InvalidTarget(String),
}

#[derive(Debug, Error)]
pub enum NauticDnsPacketFlagError {
    #[error("Error with packet header flag: '{0}' | value: {0}")]
    BadField(String, String),

    #[error("Flag builder failed to build: {0}")]
    FlagBuilderFailure(#[from] FlagsBuilderError),
}

#[derive(Debug, Error)]
pub enum NauticDnsPacketQuestionError {
    #[error("Error with question field: '{0}' | value: {0}")]
    BadField(String, String),
}
