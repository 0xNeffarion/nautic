use super::{BitParseError, ByteScanner, ScanResult};
use crate::protocol::ByteScan;
use bitter::BitReader;
use bytes::{BufMut, Bytes, BytesMut};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelSequence(Rc<str>);

impl LabelSequence {
    pub fn new(value: &str) -> Self {
        Self(Rc::from(value))
    }

    pub fn total_bits(&self) -> usize {
        self.0.clone().len()
    }

    pub fn label(&self) -> &str {
        self.0.as_ref()
    }
}

impl ByteScanner for LabelSequence {
    type Error = BitParseError;

    fn try_scan(message: &[u8], cursor: usize) -> ScanResult<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(&message[cursor..]);

        let mut name = String::new();
        let mut label_byte_size = 0;

        for _ in 0..u8::MAX {
            let length = reader
                .read_u8()
                .ok_or_else(|| BitParseError::MalformedBits("Label length".into()))?;

            label_byte_size += 1;
            if length == 0 {
                break;
            }

            if !name.is_empty() {
                name.push('.');
            }

            // Check if it is a pointer. If so, read it and exit loop
            if length & 0b11000000 != 0 {
                // Obtain offset for label
                let offset = reader
                    .read_u8()
                    .ok_or_else(|| BitParseError::MalformedBits("Label pointer offset".into()))?;

                let offset = u16::from_be_bytes([length & 0b00111111, offset]) as usize;

                // Read label from message with cursor set with pointer offset
                let label_scan = LabelSequence::try_scan(message, offset)?;
                name.push_str(label_scan.value().label());
                label_byte_size += 1;
                break;
            }

            for _ in 0..length {
                let character = reader
                    .read_u8()
                    .ok_or_else(|| BitParseError::MalformedBits("Label Character".into()))?
                    as char;

                label_byte_size += 1;
                name.push(character);
            }
        }

        Ok(ByteScan::new(LabelSequence::new(&name), label_byte_size))
    }
}

impl From<LabelSequence> for Bytes {
    fn from(value: LabelSequence) -> Self {
        (&value).into()
    }
}

impl From<&LabelSequence> for Bytes {
    fn from(value: &LabelSequence) -> Self {
        let mut buffer = BytesMut::new();
        let labels = value.label().trim().split('.').collect::<Vec<&str>>();

        for label in labels {
            let length = label.len() as u8;
            buffer.put_u8(length);

            let chars = label.chars().collect::<Vec<char>>();
            for i in 0..length {
                buffer.put_u8(chars[i as usize] as u8);
            }
        }

        buffer.put_u8(0);
        buffer.freeze()
    }
}

#[cfg(test)]
mod tests {

    use super::LabelSequence;
    use crate::protocol::ByteScanner;
    use bytes::Bytes;

    #[test]
    fn sample_domain_label_sequence_to_bytes_and_back_success() {
        let label_sequence = LabelSequence::new("www.github.com");
        let bytes: Bytes = label_sequence.clone().into();
        let label_sequence2 =
            LabelSequence::try_scan(&bytes, 0).expect("Failed to parse label sequence");
        let label_sequence2 = label_sequence2.value();

        assert_eq!(label_sequence.label(), label_sequence2.label());
    }

    #[test]
    fn sample_sub_domain_label_sequence_to_bytes_and_back_success() {
        let label_sequence = LabelSequence::new("this.is.a.subdomain.github.com");
        let bytes: Bytes = label_sequence.clone().into();
        let label_sequence2 =
            LabelSequence::try_scan(&bytes, 0).expect("Failed to parse label sequence");

        let label_sequence2 = label_sequence2.value();

        assert_eq!(label_sequence.label(), label_sequence2.label());
    }

    #[test]
    fn total_bits_sample_domain_is_correct_success() {
        let label_sequence = LabelSequence::new("www.github.com");
        assert_eq!(label_sequence.total_bits(), 16 * 8);
    }

    #[test]
    fn total_bits_sample_sub_domain_is_correct_success() {
        let label_sequence = LabelSequence::new("this.is.a.subdomain.github.com");
        assert_eq!(label_sequence.total_bits(), 32 * 8);
    }

    #[test]
    fn parse_bytes_sample_sub_domain_success() {
        let bytes = [
            0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
            0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
            0x63, 0x6f, 0x6d, 0x00,
        ];

        let label_sequence =
            LabelSequence::try_scan(&bytes[..], 0).expect("Failed to parse label sequence");

        let label_sequence = label_sequence.value();

        assert_eq!(label_sequence.label(), "this.is.a.subdomain.github.com");
    }

    #[test]
    fn parse_bytes_sample_domain_success() {
        let bytes = [
            0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00,
        ];

        let label_sequence =
            LabelSequence::try_scan(&bytes[..], 0).expect("Failed to parse label sequence");

        let label_sequence = label_sequence.value().label();

        assert_eq!(label_sequence, "www.github.com");
    }

    #[test]
    fn parse_bytes_sample_malformed_bytes_fails() {
        let bytes = [0x03, 0x77, 0x03, 0x63, 0x6f, 0x6d];
        let label_sequence = LabelSequence::try_scan(&bytes[..], 0);

        assert!(label_sequence.is_err());
    }

    #[test]
    fn parse_bytes_sample_malformed_domain_fails() {
        let bytes = [
            0x04, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00,
        ];

        let label_sequence = LabelSequence::try_scan(&bytes[..], 0);

        assert!(label_sequence.is_err());
    }
}
