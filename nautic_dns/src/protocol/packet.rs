use derive_builder::Builder;

use super::{question::Question, Header};

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub struct Packet {
    header: Header,
    question: Question,
}
