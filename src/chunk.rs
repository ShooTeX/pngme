use std::{
    fmt::Display,
    io::{BufReader, Read},
};

use anyhow::{bail, Error, Result};
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::ChunkType;

struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        Self { chunk_type, data }
    }

    fn length(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        let crc: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .chain(self.data.iter())
            .copied()
            .collect();

        crc.checksum(&bytes)
    }

    fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Ok(s) => Ok(s),
            Err(_) => bail!("Failed to format string"),
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, self::Error> {
        let mut reader = BufReader::new(value);
        let mut length: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut length)?;

        let length = u32::from_be_bytes(length);

        let mut chunk_type: [u8; 4] = [0, 0, 0, 0];
        reader.read_exact(&mut chunk_type)?;
        let chunk_type = ChunkType::try_from(chunk_type)?;

        if !chunk_type.is_valid() {
            bail!("invalid chunk type")
        };

        let mut data = vec![0u8; length.try_into()?];

        reader.read_exact(&mut data)?;

        let mut crc: [u8; 4] = [0, 0, 0, 0];

        reader.read_exact(&mut crc)?;

        let crc = u32::from_be_bytes(crc);

        let chunk = Self {
            chunk_type,
            data,
        };

        if chunk.crc() != crc {
            bail!("invalid crc")
        };

        Ok(chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = String::from_utf8(self.data.clone()).unwrap();

        write!(f, "{}", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
