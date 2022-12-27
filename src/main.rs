use std::{fs::File, os::unix::prelude::FileExt};

use anyhow::Result;
use args::PngArgs;
use chunk::Chunk;
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

fn main() -> Result<()> {
    let args = PngArgs::parse();

    match &args.command {
        args::Commands::Encode {
            file,
            chunk_type,
            message,
            output_path,
        } => {
            let chunk = Chunk::new(chunk_type.clone(), message.as_bytes().to_vec());
            let mut new_png = file.clone();
            new_png.append_chunk(chunk);

            if let Some(output) = output_path {
                let file = File::create(output)?;

                file.write_all_at(&new_png.as_bytes(), 0)?;
            };
            Ok(())
        }
        args::Commands::Decode { file, chunk_type } => todo!(),
        args::Commands::Remove { file, chunk_type } => todo!(),
        args::Commands::Print { file } => {
            print!("{file}");
            Ok(())
        },
    }
}
