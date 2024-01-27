use std::fmt::Display;

use crate::{chunk_type::ChunkType, Error, Result};

pub const ALGO: crc::Crc<u32> = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);

#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    message: Vec<u8>,
    checksum: u32,
    chunk_type: ChunkType,
}

impl Chunk {
    fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.message
    }

    fn crc(&self) -> u32 {
        self.checksum
    }

    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.message.to_vec()).unwrap())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.message.iter())
            .chain(self.checksum.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(values: &[u8]) -> Result<Self> {
        let length_bytes = values[..4].try_into()?;
        let chunk_type_bytes: [u8; 4] = values[4..8].try_into()?;
        let message_bytes: Vec<u8> = values[8..(values.len() - 4)].try_into()?;
        let checksum_bytes = values[(values.len() - 4)..].try_into()?;
        let bytes_for_checksum: Vec<u8> = values[4..(values.len() - 4)].try_into()?;

        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        let length = u32::from_be_bytes(length_bytes);
        let checksum = u32::from_be_bytes(checksum_bytes);

        if length != message_bytes.len() as u32 {
            return Err(Error::from("Invalid length"));
        }

        if ALGO.checksum(&bytes_for_checksum) != checksum {
            return Err(Error::from("Invalid checksum"));
        }

        Ok(Self {
            length,
            checksum,
            chunk_type,
            message: message_bytes,
        })
    }
}
