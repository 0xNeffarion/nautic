use derive_builder::Builder;

use super::{query::Query, Header, Record};

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub struct Packet {
    header: Header,
    question: Query,
    answer: Option<Vec<Record>>,
}
