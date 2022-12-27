use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Result};
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
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: Png,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,

        /// Your secret message
        message: String,

        /// The output for the PNG with the secret message
        output_path: Option<PathBuf>,
    },

    Decode {
        /// A Valid PNG file
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: Png,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Remove chunk from PNG
    Remove {
        /// A Valid PNG file
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: Png,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Print from PNG
    Print {
        /// A Valid PNG file
        #[arg(value_parser = png_parser, name = "FILE_PATH")]
        file: Png,
    },
}

fn png_parser(p: &str) -> Result<Png> {
    let file = fs::read(p)?;

    Png::try_from(&file[..]).map_err(|e| anyhow!("Invalid file: {e}"))
}

fn chunk_type_parser(ct: &str) -> Result<ChunkType> {
    let ct = ChunkType::from_str(ct)?;

    if !ct.is_valid() {
        bail!("Invalid ChunkType")
    }

    Ok(ct)
}
