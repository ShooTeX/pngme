use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Result};
use clap::{Parser, Subcommand};

use crate::{chunk_type::ChunkType, png::Png};

/// Simple CLI tool to hide messages inside a PNG
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(author = "Erik Simon", version)]
#[command(about = "Simple CLI tool to hide messages inside a PNG", long_about = None)]
pub struct PngArgs {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Encode a secret message into a PNG file
    Encode {
        #[arg(value_parser = png_parser)]
        file_path: Png,

        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
        message: String,

        output_path: Option<PathBuf>,
    },
    Decode {
        #[arg(value_parser = png_parser)]
        file_path: PathBuf,

        #[arg(value_parser = chunk_type_parser)]
        chunk_type: String,
    },
    /// Remove chunk from PNG
    Remove {
        #[arg(value_parser = png_parser)]
        file_path: PathBuf,

        #[arg(value_parser = chunk_type_parser)]
        chunk_type: String,
    },
    /// Print from PNG
    Print {
        #[arg(value_parser = png_parser)]
        file_path: PathBuf,
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
