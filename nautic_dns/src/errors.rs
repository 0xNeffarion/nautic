use thiserror::*;
use tokio::io;

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
