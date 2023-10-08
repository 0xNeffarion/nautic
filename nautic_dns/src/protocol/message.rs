use crate::protocol::ByteScanner;
use derive_builder::Builder;

use super::{query::Query, Header, Record};

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub struct Message {
    header: Header,
    question: Query,
    answer: Vec<Record>,
}

impl ByteScanner for Message {
    type Error = String;

    fn try_scan(message: &[u8], cursor: usize) -> Result<(Self, usize), Self::Error> {
        let (header, header_len) = Header::try_scan(message, cursor)?;
        let (query, query_len) = Query::try_scan(message, cursor + header_len)?;

        let mut records = vec![];
        let mut records_len = 0;
        if header.flags.message_type == MessageType::Response {
            let mut cursor = cursor + header_len + query_len;
            for _ in 0..header.answers_size {
                let (record, record_len) = Record::try_scan(message, cursor)?;
                records.push(record);
                cursor += record_len;
                records_len += record_len;
            }
        };

        let answers = if records.is_empty() {
            None
        } else {
            Some(records)
        };

        Ok((
            Message {
                header,
                query,
                answers,
            },
            header_len + query_len + records_len,
        ))
    }
}
