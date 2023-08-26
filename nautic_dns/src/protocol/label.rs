use std::ops::{Deref, DerefMut};

use super::BitParseError;
use bitter::BitReader;
use bytes::BufMut;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelSequence(String, u32);

impl LabelSequence {
    pub fn new(name: &str) -> Self {
        let labels = name.trim().split('.').collect::<Vec<&str>>();
        let bits = (8 * labels.len() as u32)
            + (labels.into_iter().map(|x| x.len() * 8).sum::<usize>() as u32)
            + 8;
        Self(name.to_owned(), bits)
    }

    pub fn total_bits(&self) -> u32 {
        self.1
    }
}

impl Deref for LabelSequence {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LabelSequence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<str> for LabelSequence {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&[u8]> for LabelSequence {
    type Error = BitParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(value);
        let mut name = String::new();
        let mut bits = 0;

        for i in 0..64 {
            // Read the the lenght of the next label
            let length = reader
                .read_u8()
                .ok_or_else(|| BitParseError::MalformedBits("Label Lenght".into()))?;

            bits += 8;

            // If the length is 0, we've reached the end of the domain name.
            if length == 0 {
                break;
            }

            // Add a '.' to the name if it isnt the first iteration
            if i != 0 {
                name.push('.');
            }

            for _ in 0..length {
                let char_bits = reader
                    .read_u8()
                    .ok_or_else(|| BitParseError::MalformedBits("Label Character".into()))?;

                name.push(char_bits as char);
                bits += 8;
            }
        }

        Ok(Self(name, bits))
    }
}

impl From<LabelSequence> for Vec<u8> {
    fn from(value: LabelSequence) -> Self {
        let mut buffer = Vec::new();

        let labels = value.trim().split('.').collect::<Vec<&str>>();

        for label in labels {
            let length = label.len() as u8;
            buffer.put_u8(length);

            let chars = label.chars().collect::<Vec<char>>();
            for i in 0..length {
                buffer.put_u8(chars[i as usize] as u8);
            }
        }

        buffer.put_u8(0);

        buffer
    }
}

#[cfg(test)]
mod tests {

    use super::LabelSequence;
    use std::convert::TryFrom;

    #[test]
    fn sample_domain_label_sequence_to_bytes_and_back_success() {
        let label_sequence = LabelSequence::new("www.github.com");
        let bytes: Vec<u8> = label_sequence.clone().into();
        let label_sequence2 =
            LabelSequence::try_from(bytes.as_slice()).expect("Failed to parse label sequence");

        assert_eq!(label_sequence, label_sequence2);
    }

    #[test]
    fn sample_sub_domain_label_sequence_to_bytes_and_back_success() {
        let label_sequence = LabelSequence::new("this.is.a.subdomain.github.com");
        let bytes: Vec<u8> = label_sequence.clone().into();
        let label_sequence2 =
            LabelSequence::try_from(bytes.as_slice()).expect("Failed to parse label sequence");

        assert_eq!(label_sequence, label_sequence2);
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
    fn parse_bytes_sample_sub_domain_sucesss() {
        let bytes = [
            0x04, 0x74, 0x68, 0x69, 0x73, 0x02, 0x69, 0x73, 0x01, 0x61, 0x09, 0x73, 0x75, 0x62,
            0x64, 0x6f, 0x6d, 0x61, 0x69, 0x6e, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03,
            0x63, 0x6f, 0x6d, 0x00,
        ];

        let label_sequence =
            LabelSequence::try_from(&bytes[..]).expect("Failed to parse label sequence");

        assert_eq!(*label_sequence, "this.is.a.subdomain.github.com");
    }

    #[test]
    fn parse_bytes_sample_domain_sucesss() {
        let bytes = [
            0x03, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00,
        ];

        let label_sequence =
            LabelSequence::try_from(&bytes[..]).expect("Failed to parse label sequence");

        assert_eq!(*label_sequence, "www.github.com");
    }

    #[test]
    fn parse_bytes_sample_malformed_bytes_fails() {
        let bytes = [0x03, 0x77, 0x03, 0x63, 0x6f, 0x6d];
        let label_sequence = LabelSequence::try_from(&bytes[..]);

        assert!(label_sequence.is_err());
    }

    #[test]
    fn parse_bytes_sample_malformed_domain_fails() {
        let bytes = [
            0x04, 0x77, 0x77, 0x77, 0x06, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x03, 0x63, 0x6f,
            0x6d, 0x00,
        ];

        let label_sequence = LabelSequence::try_from(&bytes[..]);

        assert!(label_sequence.is_err());
    }
}
