use std::{fs, path::PathBuf, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};
use clap::{Parser, Subcommand};
use reqwest::blocking;

use crate::{chunk_type::ChunkType, png::Png};

/// Simple CLI tool to hide messages inside a PNG
#[derive(Debug, Parser)]
#[command(name = "pngme")]
#[command(author, version)]
#[command(about = "Simple CLI tool to hide messages inside a PNG", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct PngArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Encode a secret message into a PNG file
    #[command(arg_required_else_help = true)]
    Encode {
        /// File path or url to a png file
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
    #[command(arg_required_else_help = true)]
    Decode {
        /// File path or url to a png file
        #[arg(value_parser = Input::from_str)]
        input: Input,

        /// A chunk type, i.e. `ruSt`.
        /// Leave empty to search for potential secret messages.
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: Option<ChunkType>,
    },

    /// Remove chunk from PNG
    #[command(arg_required_else_help = true)]
    Remove {
        /// File path or url to a png file
        #[arg(value_parser = Input::from_str)]
        input: Input,

        /// A chunk type, i.e. `ruSt`
        #[arg(value_parser = chunk_type_parser)]
        chunk_type: ChunkType,
    },

    /// Print from PNG
    #[command(arg_required_else_help = true)]
    Print {
        /// File path or url to a png file
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
        let is_url = s.starts_with("https");
        
        let file_bytes = match is_url {
            true => blocking::get(s)?.bytes()?.into(),
            false => fs::read(s)?,
        };
        
        let png = Png::try_from(&file_bytes[..]).map_err(|e| anyhow!("Invalid file: {e}"))?;

        Ok(Input {
            png,
            path: (!is_url).then_some(s.to_string()),
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
