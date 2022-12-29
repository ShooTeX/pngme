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
            file,
            chunk_type,
            message,
            output_path,
        } => {
            let chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());
            let mut new_png = file.png().clone();
            new_png.append_chunk(chunk);

            if let Some(output) = output_path {
                fs::write(output, new_png.as_bytes())?;
                return Ok(());
            };

            fs::write(file.path(), new_png.as_bytes())?;

            Ok(())
        }
        args::Commands::Decode { file, chunk_type } => {
            if let Some(chunk_type) = chunk_type {
                let chunk = match file.png().chunk_by_type(&chunk_type.to_string()) {
                    Some(c) => c,
                    None => bail!("Chunk not found"),
                };

                println!("{chunk}");

                return Ok(());
            };

            let messages: Vec<_> = file
                .png()
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
        args::Commands::Remove { file, chunk_type } => {
            let new_png = file.png().clone().remove_chunk(&chunk_type.to_string())?;

            fs::write(file.path(), new_png.as_bytes())?;

            Ok(())
        }
        args::Commands::Print { file } => {
            let file = file.png();
            print!("{file}");
            Ok(())
        }
    }
}
