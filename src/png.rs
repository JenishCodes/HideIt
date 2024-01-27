use core::fmt;

use crate::{chunk::Chunk, Error, Result};

#[derive(Debug)]
pub struct Png {
    pub header: [u8; 8],
    pub chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk)
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        for (index, chunk) in self.chunks.iter().enumerate() {
            if chunk.chunk_type().as_string() == chunk_type {
                return Result::Ok(self.chunks.remove(index));
            }
        }
        Err(Error::from("Chunk type not found!"))
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        for chunk in self.chunks.iter() {
            if chunk.chunk_type().as_string() == chunk_type {
                return Some(chunk);
            }
        }
        None
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let chunk_bytes: Vec<u8> = self
            .chunks
            .clone()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        self.header
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if Self::STANDARD_HEADER != bytes[..8] {
            return Err(Error::from("Invalid header"));
        }

        let (header, bytes) = bytes.split_at(8);

        let mut png = Self {
            header: header.try_into()?,
            chunks: Vec::new(),
        };

        let mut index = 0;
        while index < bytes.len() {
            let length_bytes = bytes[index..index + 4].try_into()?;
            let length = u32::from_be_bytes(length_bytes) + 4 + 4 + 4;

            let chunk_bytes: Vec<u8> = bytes[index..index + length as usize].to_vec();
            let chunk = Chunk::try_from(chunk_bytes.as_ref())?;

            png.append_chunk(chunk);

            index += length as usize;
        }

        Ok(png)
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_bytes())
    }
}
