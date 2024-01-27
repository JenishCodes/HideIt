use std::{fmt::Display, str::FromStr};

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl FromStr for ChunkType {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let mut bytes: [u8; 4] = [0; 4];

        for (i, b) in s.chars().enumerate() {
            if !b.is_ascii_alphabetic() {
                return Err(Error::from("Invalid string"))
            }
            bytes[i] = b as u8;
        }

        Ok(ChunkType{
            bytes,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.bytes().as_mut()).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        let chunk = ChunkType {bytes: value};

        match chunk.is_valid() {
            true => Ok(chunk),
            false => Err(Error::from("Invalid chunk type string")),
        }
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }
    
    fn is_valid(&self) -> bool {
        let mut ans: bool = self.is_reserved_bit_valid();

        for byte in self.bytes() {
            ans = ans & Self::is_valid_byte(byte);
        }

        ans
    }

    pub fn as_string(&self) -> String {
        String::from_utf8(self.bytes.to_vec()).unwrap()
    }

    fn is_valid_byte(bytes: u8) -> bool {
        bytes.is_ascii_alphabetic()
    }

    pub fn is_critical(&self) -> bool {
        (self.bytes[0] as char).is_uppercase()
    }

    pub fn is_public(&self) -> bool {
        (self.bytes[1] as char).is_uppercase()
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] as char).is_uppercase()
    }
    
    pub fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] as char).is_lowercase()
    }
}
