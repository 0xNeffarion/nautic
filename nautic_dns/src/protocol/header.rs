use bitter::BitReader;
use bytes::BufMut;
use derive_builder::Builder;

use super::{Flags, PacketError};

#[derive(Debug, Clone, Builder, PartialEq, Eq)]
pub struct Header {
    #[builder(default = "rand::random::<u16>()")]
    id: u16,
    flags: Flags,
    questions_size: u16,
    answers_size: u16,
    name_servers_size: u16,
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

impl TryFrom<Vec<u8>> for Header {
    type Error = PacketError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let mut reader = bitter::BigEndianReader::new(&bytes);

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
