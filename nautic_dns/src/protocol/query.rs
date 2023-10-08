use crate::protocol::{ByteScan, ByteScanner, ScanResult};
use bitter::BitReader;
use bytes::{BufMut, Bytes, BytesMut};

use super::{label::LabelSequence, types::*, BitParseError, Class};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    name: LabelSequence,
    r#type: RecordType,
    class: Class,
}

impl Query {
    pub fn new(name: LabelSequence, r#type: RecordType, class: Class) -> Self {
        Self {
            name,
            r#type,
            class,
        }
    }

    pub fn name(&self) -> &LabelSequence {
        &self.name
    }
    pub fn r#type(&self) -> &RecordType {
        &self.r#type
    }
    pub fn class(&self) -> &Class {
        &self.class
    }
}

impl ByteScanner for Query {
    type Error = BitParseError;

    fn try_scan(message: &[u8], cursor: usize) -> ScanResult<Self, Self::Error> {
        let name = LabelSequence::try_scan(message, cursor)?.value().clone();
        let name_bytes = name.total_bits() / 8;

        let value = &message[name_bytes..];
        let mut reader = bitter::BigEndianReader::new(value);

        let r#type: RecordType = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Record Type".into()))?
            .try_into()?;

        let class: Class = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Class Type".into()))?
            .try_into()?;

        Ok(ByteScan::new(
            Query::new(name, r#type, class),
            name_bytes + 4,
        ))
    }
}

impl From<Query> for Bytes {
    fn from(value: Query) -> Self {
        (&value).into()
    }
}

impl From<&Query> for Bytes {
    fn from(value: &Query) -> Self {
        let mut buffer = BytesMut::new();
        let value = value.clone();
        let name = Bytes::from(value.name);

        buffer.put_slice(&name);
        buffer.put_u16(value.r#type as u16);
        buffer.put_u16(value.class as u16);

        buffer.freeze()
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{label::LabelSequence, ByteScanner, Class, RecordType};
    use bytes::Bytes;

    #[test]
    fn sample_domain_query_a_to_bytes_success() {
        let name = "www.github.com";
        let r#type = RecordType::A;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Bytes = query.clone().into();

        assert_eq!(bytes.len(), 20);
        assert_eq!(
            bytes,
            Bytes::copy_from_slice(&[
                0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
                0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
            ])
        );
    }

    #[test]
    fn sample_sub_domain_query_a_to_bytes_success() {
        let name = "this.is.a.subdomain.github.com";
        let r#type = RecordType::A;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Bytes = query.clone().into();

        assert_eq!(bytes.len(), 36);
        assert_eq!(
            bytes,
            Bytes::copy_from_slice(&[
                0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
                0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
                0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01
            ])
        );
    }

    #[test]
    fn sample_domain_query_aaaa_to_bytes_success() {
        let name = "www.github.com";
        let r#type = RecordType::AAAA;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Bytes = query.clone().into();

        assert_eq!(bytes.len(), 20);
        assert_eq!(
            bytes,
            Bytes::copy_from_slice(&[
                0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
                0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01
            ])
        );
    }

    #[test]
    fn parse_bytes_query_a_sample_domain_success() {
        let bytes = Bytes::copy_from_slice(&[
            0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
        ]);

        let query = super::Query::try_scan(&bytes, 0).expect("Failed to parse query");
        let query = query.value();

        assert_eq!(query.name().label(), "www.github.com");
        assert_eq!(query.r#type(), &RecordType::A);
        assert_eq!(query.class(), &Class::IN);
    }

    #[test]
    fn parse_bytes_query_aaaa_sample_sub_domain_success() {
        let bytes = Bytes::copy_from_slice(&[
            0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
            0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
            0x63, 0x6f, 0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01,
        ]);

        let query = super::Query::try_scan(&bytes, 0).expect("Failed to parse query");
        let query = query.value();

        assert_eq!(query.name().label(), "this.is.a.subdomain.github.com");
        assert_eq!(query.r#type(), &RecordType::AAAA);
        assert_eq!(query.class(), &Class::IN);
    }
}
