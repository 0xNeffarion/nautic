use bitter::BitReader;
use bytes::BufMut;
use derive_builder::Builder;

use super::{Flags, PacketError};

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub struct Header {
    #[builder(default = "rand::random::<u16>()")]
    id: u16,

    #[builder(default = "crate::protocol::FlagsBuilder::default().build().unwrap()")]
    flags: Flags,

    #[builder(default = "0")]
    questions_size: u16,

    #[builder(default = "0")]
    answers_size: u16,

    #[builder(default = "0")]
    name_servers_size: u16,

    #[builder(default = "0")]
    additional_size: u16,
}

impl Header {
    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn flags(&self) -> &Flags {
        &self.flags
    }

    pub fn questions_size(&self) -> u16 {
        self.questions_size
    }

    pub fn answers_size(&self) -> u16 {
        self.answers_size
    }

    pub fn name_servers_size(&self) -> u16 {
        self.name_servers_size
    }

    pub fn additional_size(&self) -> u16 {
        self.additional_size
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = PacketError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(bytes);

        let id = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("ID".into()))?;

        let flags_bytes = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("Flags".into()))?;

        let flags = Flags::try_from(flags_bytes.to_le_bytes().to_vec())?;

        let questions_size = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("Questions Size".into()))?;

        let answers_size = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("Answers Size".into()))?;

        let name_servers_size = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("Name Servers Size".into()))?;

        let additional_size = reader
            .read_u16()
            .ok_or_else(|| PacketError::MalformedBits("Additional Size".into()))?;

        let header = HeaderBuilder::default()
            .id(id)
            .flags(flags)
            .questions_size(questions_size)
            .answers_size(answers_size)
            .name_servers_size(name_servers_size)
            .additional_size(additional_size)
            .build()?;

        Ok(header)
    }
}

impl From<Header> for Vec<u8> {
    fn from(header: Header) -> Self {
        let mut result = vec![];
        let flags: Vec<u8> = header.flags().clone().into();

        result.put_u16(header.id());
        result.put_slice(&flags);
        result.put_u16(header.questions_size());
        result.put_u16(header.answers_size());
        result.put_u16(header.name_servers_size());
        result.put_u16(header.additional_size());

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{FlagsBuilder, MessageType};

    use super::*;

    #[test]
    fn header_from_binary_1_success() {
        let bytes = vec![
            0xa6, 0x29, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00,
        ];

        let header = Header::try_from(bytes.as_slice()).unwrap();

        assert_eq!(header.id(), 0xa629);
        assert_eq!(header.flags(), &FlagsBuilder::default().build().unwrap());
        assert_eq!(header.questions_size(), 1);
        assert_eq!(header.answers_size(), 0);
        assert_eq!(header.name_servers_size(), 2);
        assert_eq!(header.additional_size(), 0);
    }

    #[test]
    fn header_from_binary_2_success() {
        let bytes = vec![
            0xab,
            0xaa,
            0x00,
            0b1000_0000,
            0x00,
            0x01,
            0x00,
            0x00,
            0x00,
            0x02,
            0x00,
            0x00,
        ];

        let header = Header::try_from(bytes.as_slice()).unwrap();

        assert_eq!(header.id(), 0xabaa);
        assert_eq!(
            header.flags(),
            &FlagsBuilder::default()
                .message_type(MessageType::Response)
                .build()
                .unwrap()
        );
        assert_eq!(header.questions_size(), 1);
        assert_eq!(header.answers_size(), 0);
        assert_eq!(header.name_servers_size(), 2);
        assert_eq!(header.additional_size(), 0);
    }

    #[test]
    fn header_into_binary_success() {
        let header = HeaderBuilder::default()
            .id(0xf555)
            .flags(FlagsBuilder::default().build().unwrap())
            .questions_size(1)
            .answers_size(1)
            .name_servers_size(3)
            .additional_size(1)
            .build()
            .unwrap();

        let bytes: Vec<u8> = header.into();

        assert_eq!(
            bytes,
            vec![0xf5, 0x55, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01]
        );
    }

    #[test]
    fn header_to_binary_and_back_success() {
        let header = HeaderBuilder::default()
            .flags(FlagsBuilder::default().build().unwrap())
            .questions_size(1)
            .answers_size(1)
            .name_servers_size(3)
            .additional_size(1)
            .build()
            .unwrap();

        let bytes: Vec<u8> = header.clone().into();
        let header_from_bytes = Header::try_from(bytes.as_slice()).unwrap();

        assert_eq!(header, header_from_bytes);
    }
}
