use std::convert::TryFrom;
use std::fs;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::{Chunk, ALGO};
use crate::png::Png;
use crate::{Error, Result};

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let file = fs::read(&args.file_name)?;

    let length_bytes = (args.message.len() as u32).to_be_bytes();

    let chunk_type_bytes: Vec<u8> = args.chunk_type.bytes().collect();

    let message_bytes: Vec<u8> = args.message.bytes().collect();

    let bytes_for_checksum: Vec<u8> = chunk_type_bytes
        .iter()
        .chain(message_bytes.iter())
        .copied()
        .collect();

    let checksum_bytes = ALGO.checksum(&bytes_for_checksum).to_be_bytes();

    let chunk_type: Vec<u8> = length_bytes
        .iter()
        .chain(bytes_for_checksum.iter())
        .chain(checksum_bytes.iter())
        .copied()
        .collect();

    let chunk = Chunk::try_from(chunk_type.as_ref())?;

    if chunk.chunk_type().is_critical()
        || chunk.chunk_type().is_public()
        || !chunk.chunk_type().is_safe_to_copy()
    {
        return Err(Error::from("Invalid chunk type string."));
    }

    let mut png = Png::try_from(file.as_ref())?;

    png.append_chunk(chunk);

    let mut filename = args.file_name;

    if let Some(output_filename) = args.output_file_name {
        filename = output_filename;
    }

    match fs::write(filename, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::from("Error in writing file!")),
    }
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let file = fs::read(&args.file_name)?;

    let png = Png::try_from(file.as_ref())?;

    let chunk = png.chunk_by_type(&args.chunk_type);

    if let Some(chunk) = chunk {
        return Ok(println!("Found message: {}", chunk.data_as_string()?));
    }

    println!(
        "No hidden message found with chunk type: {}",
        args.chunk_type
    );

    Ok(())
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file = fs::read(&args.file_name)?;

    let mut png = Png::try_from(file.as_ref())?;

    match png.remove_chunk(&args.chunk_type) {
        Ok(chunk) => {
            println!("Chunk has been removed from file");
            println!("Message: {}", chunk.data_as_string()?)
        }
        Err(_) => println!("No message found with chunk type: {}", args.chunk_type),
    }

    match fs::write(args.file_name, png.as_bytes()) {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::from("Error in writing file!")),
    }
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file = fs::read(&args.file_name)?;

    let png = Png::try_from(file.as_ref())?;

    for chunk in png.chunks().iter() {
        println!("{}", chunk);
    }

    Ok(())
}
