use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand};

use crate::{chunk_type::ChunkType, file::File, png::Png};

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
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: File,

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
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: File,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Remove chunk from PNG
    Remove {
        /// A Valid PNG file
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: File,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Print from PNG
    Print {
        /// A Valid PNG file
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: File,
    },
}

fn png_parser(p: &str) -> Result<File> {
    let file_bytes = fs::read(p)?;

    let file = Png::try_from(&file_bytes[..]).map_err(|e| anyhow!("Invalid file: {e}"))?;

    Ok(File {
        png: file,
        path: p.to_string(),
    })
}

fn chunk_type_parser(ct: &str) -> Result<ChunkType> {
    let ct = ChunkType::from_str(ct)?;

    if !ct.is_valid() {
        bail!("Invalid ChunkType")
    }

    Ok(ct)
}
