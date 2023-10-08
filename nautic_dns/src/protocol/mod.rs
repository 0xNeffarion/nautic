mod class;
mod errors;
mod flags;
mod header;
mod label;
mod message;
mod query;
mod record;
mod types;

pub use class::*;
pub use errors::*;
pub use flags::*;
pub use header::*;
pub use label::*;
pub use message::*;
pub use query::*;
pub use record::*;
pub use types::*;

pub type ScanResult<T, E> = Result<ByteScan<T>, E>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ByteScan<T> {
    value: T,
    total_bytes: usize,
}

impl<T> ByteScan<T> {
    pub fn new(value: T, total_bytes: usize) -> Self {
        Self { value, total_bytes }
    }
    
    pub fn value(&self) -> &T {
        &self.value
    }
    
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
}

pub trait ByteScanner: Sized {
    type Error;

    fn try_scan(message: &[u8], cursor: usize) -> ScanResult<Self, Self::Error>;
}
