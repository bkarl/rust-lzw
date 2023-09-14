use std::fs::File;
use std::io::prelude::*;

mod lz_dictionary;
use lz_dictionary::{LzDictionary, EMPTY_PATTERN};
mod file_writer;
use file_writer::FileWriter;
mod lz_compressor;
use lz_compressor::{LzCompressor, LzCompressorParameters};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut file_in = &File::open("file_in.bin")?;
    let mut input_buffer = Vec::new();
    let mut compressor = LzCompressor::new();
    let mut file_writer = FileWriter::new("data/data_out.bin")?;
    file_in.read_to_end(&mut input_buffer)?;

    let compressed_data = compressor.compress_data(input_buffer.as_ref());
    file_writer.write_packed_contents_to_file(&compressed_data)?;
    Ok(())
}

