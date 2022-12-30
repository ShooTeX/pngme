use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Result, Error};
use clap::{Parser, Subcommand};

use crate::{chunk_type::ChunkType, png::Png};

/// Simple CLI tool to hide messages inside a PNG
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(author, version)]
#[command(about = "Simple CLI tool to hide messages inside a PNG", long_about = None)]
pub struct PngArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a secret message into a PNG file
    Encode {
        /// A Valid PNG file
        #[arg(value_parser = Input::from_str)]
        input: Input,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,

        /// Your secret message
        message: String,

        /// The output for the PNG with the secret message
        output_path: Option<PathBuf>,
    },

    /// Decode a secret message from a PNG file
    Decode {
        /// A Valid PNG file
        #[arg(value_parser = Input::from_str)]
        input: Input,

        /// A chunk type, i.e. `ruSt`.
        /// Leave empty to search for potential secret messages.
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: Option<ChunkType>,
    },

    /// Remove chunk from PNG
    Remove {
        /// A Valid PNG file
        #[arg(value_parser = Input::from_str)]
        input: Input,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Print from PNG
    Print {
        /// A Valid PNG file
        #[arg(value_parser = Input::from_str)]
        input: Input,
    },
}

#[derive(Clone, Debug)]
pub struct Input {
    pub png: Png,
    pub path: Option<String>,
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let file_bytes = fs::read(s)?;

        let png = Png::try_from(&file_bytes[..]).map_err(|e| anyhow!("Invalid file: {e}"))?;

        Ok(Input {
            png,
            path: Some(s.to_string()),
        })
    }
}

fn chunk_type_parser(ct: &str) -> Result<ChunkType> {
    let ct = ChunkType::from_str(ct)?;

    if !ct.is_valid() {
        bail!("Invalid ChunkType")
    }

    Ok(ct)
}
