#![allow(unused_variables, dead_code, unused_imports)]

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    str::FromStr,
};

use clap::Parser;
use commands::Commands;

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    match &cli.command {

        Some(Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        }) => {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(file_path.as_path())?;

            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            // create png instance and add new chunk with the message
            let mut png = Png::try_from(bytes.as_ref())?;
            png.append_chunk(Chunk::new(
                ChunkType::from_str(&chunk_type)?,
                message.as_bytes().into(),
            ));

            // write png with the message
            if let Some(output_file) = output_file {
                let mut outfile = File::create_new(output_file)?;
                outfile.write(&png.as_bytes())?;

                println!(
                    "The message has been added to '{}'.",
                    output_file.file_name().unwrap().to_string_lossy()
                );
            } else {
                fs::write(file_path, png.as_bytes())?;
                // file.write(&png.as_bytes())?; doesn't work ?

                println!(
                    "The message has been added to '{}'.",
                    file_path.file_name().unwrap().to_string_lossy()
                );
            }
        }

        Some(Commands::Decode {
            file_path,
            chunk_type,
        }) => {
            // println!("Decoding...");
            let png = Png::from_path(file_path)?;

            match png.chunk_by_type(chunk_type) {
                Some(chunk) => {
                    println!(
                        "The message in '{}' is \"{}\".",
                        file_path.file_name().unwrap().to_string_lossy(),
                        chunk.data_as_string()?
                    );
                }
                None => println!("Message not found"),
            };
        }

        Some(Commands::Remove {
            file_path,
            chunk_type,
        }) => {
            let mut png = Png::from_path(&file_path)?;

            match png.remove_chunk(chunk_type) {
                Ok(chunk) => {
                    fs::write(file_path, png.as_bytes())?;
                    println!("\"{}\" message has been removed.", chunk.data_as_string()?);
                },
                Err(e) => println!("{}", e)
            }
        }

        Some(Commands::Print { file_path }) => {
            let png = Png::from_path(&file_path)?;

            // filter chunks to get only secret chunk
            let secret_chunk = png.chunks().into_iter().filter(|chunk| {
                if let Ok(string) = chunk.data_as_string() {
                    let chunk_type = chunk.chunk_type().to_string();
                    if !chunk_type.eq("sBIT") && !chunk_type.eq("IEND") {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            });

            if secret_chunk.clone().count() > 0 {
                for chunk in secret_chunk.clone() {
                    println!("Key '{}' has secret : \"{}\"", chunk.chunk_type(), chunk.data_as_string()?);
                }
            } else {
                println!("No secret found.");
            }

        }

        None => {}

    }

    Ok(())
}
