use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType {
    ancillary: bool,
    private: bool,
    reserved: bool,
    safe_to_copy: bool,
    code: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let code = String::from_utf8(value.to_vec())?;
        let code: Vec<char> = code.chars().collect();

        Ok(ChunkType {
            ancillary: code[0].is_lowercase(),
            private: code[1].is_lowercase(),
            reserved: code[2].is_uppercase(),
            safe_to_copy: code[3].is_lowercase(),
            code: value,
        })
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            bail!("str has to have a length of 4");
        };

        if !s.chars().all(char::is_alphabetic) {
            bail!("string can only be alphanumeric");
        }

        let code = match s.as_bytes().try_into() {
            Ok(c) => c,
            Err(_) => bail!("weird error happened"),
        };

        let s: Vec<char> = s.chars().collect();

        Ok(ChunkType {
            ancillary: s[0].is_lowercase(),
            private: s[1].is_lowercase(),
            reserved: s[2].is_uppercase(),
            safe_to_copy: s[3].is_lowercase(),
            code,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = String::from_utf8(self.code.into()).unwrap();

        write!(f, "{code}")
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.code
    }

    pub fn is_valid(&self) -> bool {
        self.reserved
    }

    pub fn is_critical(&self) -> bool {
        !self.ancillary
    }

    pub fn is_public(&self) -> bool {
        !self.private
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.reserved
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy
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
        let _chunk_string = format!("{chunk_type_1}");
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
