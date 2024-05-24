#![allow(unused_variables)]

use std::{fmt::Display, str::FromStr};

use crate::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }

    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }
        for b in self.bytes() {
            if !ChunkType::is_valid_byte(b) {
                return false;
            }
        }
        true
    }

    pub fn is_valid_byte(byte: u8) -> bool {
        byte.is_ascii_uppercase() || byte.is_ascii_lowercase()
    }

    pub fn is_critical(&self) -> bool {
        let bin = format!("{:b}", self.ancillary);
        println!("{}, {}", bin.chars().nth(6).unwrap(), bin);
        bin.chars().nth(6) == Some('0')
    }

    pub fn is_public(&self) -> bool {
        let bin = format!("{:b}", self.private);
        bin.chars().nth(6) == Some('0')
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let bin = format!("{:b}", self.reserved);
        println!("{}, {}", bin.chars().nth(6).unwrap(), bin);
        bin.chars().nth(6) == Some('0')
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let bin = format!("{:b}", self.safe_to_copy);
        bin.chars().nth(6) == Some('1')
    }
}

// [u8; 4] to ChunkType
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<ChunkType, Self::Error> {
        Ok(ChunkType {
            ancillary: value[0],
            private: value[1],
            reserved: value[2],
            safe_to_copy: value[3],
        })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let mut res: Vec<u8> = Vec::new();
            for char in s.chars() {
                res.push(char as u8);
            }
            Ok(ChunkType {
                ancillary: res[0],
                private: res[1],
                reserved: res[2],
                safe_to_copy: res[3],
            })
        } else {
            Err("Length not correct".into())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.ancillary as char, self.private as char, self.reserved as char, self.safe_to_copy as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
