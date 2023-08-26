use bitter::BitReader;
use bytes::BufMut;

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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn r#type(&self) -> &RecordType {
        &self.r#type
    }

    pub fn class(&self) -> &Class {
        &self.class
    }
}

impl TryFrom<&[u8]> for Query {
    type Error = BitParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let name = LabelSequence::try_from(value)?;
        let name_bytes = (name.total_bits() / 8) as usize;

        let value = &value[name_bytes..];
        let mut reader = bitter::BigEndianReader::new(value);

        let r#type: RecordType = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Record Type".into()))?
            .try_into()?;

        let class: Class = reader
            .read_u16()
            .ok_or_else(|| BitParseError::MalformedBits("Class Type".into()))?
            .try_into()?;

        Ok(Query {
            name,
            r#type,
            class,
        })
    }
}

impl From<Query> for Vec<u8> {
    fn from(value: Query) -> Self {
        let mut buffer = vec![];
        let name: Vec<u8> = value.name.into();

        buffer.put_slice(name.as_slice());
        buffer.put_u16(value.r#type as u16);
        buffer.put_u16(value.class as u16);

        buffer
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{label::LabelSequence, Class, RecordType};

    #[test]
    fn sample_domain_query_a_to_bytes_success() {
        let name = "www.github.com";
        let r#type = RecordType::A;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Vec<u8> = query.clone().into();

        assert_eq!(bytes.len(), 20);
        assert_eq!(
            bytes,
            [
                0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
                0x6d, 0x00, 0x00, 0x01, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn sample_sub_domain_query_a_to_bytes_success() {
        let name = "this.is.a.subdomain.github.com";
        let r#type = RecordType::A;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Vec<u8> = query.clone().into();

        assert_eq!(bytes.len(), 36);
        assert_eq!(
            bytes,
            [
                0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
                0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
                0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn sample_domain_query_aaaa_to_bytes_success() {
        let name = "www.github.com";
        let r#type = RecordType::AAAA;
        let class = Class::IN;

        let query = super::Query::new(LabelSequence::new(name), r#type, class);
        let bytes: Vec<u8> = query.clone().into();

        assert_eq!(bytes.len(), 20);
        assert_eq!(
            bytes,
            [
                0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
                0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01
            ]
        );
    }

    #[test]
    fn parse_bytes_query_a_sample_domain_success() {
        let bytes = [
            0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00, 0x00, 0x01, 0x00, 0x01,
        ];

        let query = super::Query::try_from(bytes.as_slice()).expect("Failed to parse query");

        assert_eq!(query.name(), "www.github.com");
        assert_eq!(query.r#type(), &RecordType::A);
        assert_eq!(query.class(), &Class::IN);
    }

    #[test]
    fn parse_bytes_query_aaaa_sample_sub_domain_success() {
        let bytes = [
            0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
            0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
            0x63, 0x6f, 0x6d, 0x00, 0x00, 0x1c, 0x00, 0x01,
        ];

        let query = super::Query::try_from(bytes.as_slice()).expect("Failed to parse query");

        assert_eq!(query.name(), "this.is.a.subdomain.github.com");
        assert_eq!(query.r#type(), &RecordType::AAAA);
        assert_eq!(query.class(), &Class::IN);
    }
}
