use std::fs;

use anyhow::{bail, Result};
use clap::Parser;
use pngme::{
    args::{self, Commands, PngArgs},
    chunk::Chunk,
};

fn main() -> Result<()> {
    let args = PngArgs::parse();

    match &args.command {
        Commands::Encode {
            input,
            chunk_type,
            message,
            output_path,
        } => {
            let chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());
            let mut new_png = input.png.clone();
            new_png.append_chunk(chunk);

            if let Some(output) = output_path {
                fs::write(output, new_png.as_bytes())?;
                return Ok(());
            };

            if let Some(path) = &input.path {
                fs::write(path, new_png.as_bytes())?;
            }

            Ok(())
        }

        args::Commands::Decode { input, chunk_type } => {
            if let Some(chunk_type) = chunk_type {
                let chunk = match input.png.chunk_by_type(&chunk_type.to_string()) {
                    Some(c) => c,
                    None => bail!("Chunk not found"),
                };

                println!("{chunk}");

                return Ok(());
            };

            let messages: Vec<_> = input
                .png
                .chunks()
                .iter()
                .filter_map(|c| c.data_as_string().ok())
                .filter(|s| !s.is_empty())
                .collect();

            match messages[..] {
                [] => bail!("No potential secret messages found."),
                _ => println!("{messages:?}"),
            }

            Ok(())
        }

        args::Commands::Remove { input, chunk_type } => {
            let new_png = input.png.clone().remove_chunk(&chunk_type.to_string())?;

            if let Some(path) = &input.path {
                fs::write(path, new_png.as_bytes())?;
            }

            Ok(())
        }

        args::Commands::Print { input } => {
            let file = &input.png;
            print!("{file}");
            Ok(())
        }
    }
}
