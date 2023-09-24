use std::fs::File;
use std::io::prelude::*;
use std::env;

mod lz_dictionary;
mod file_writer;
use file_writer::FileWriter;
mod lz_compressor;
use lz_compressor::LzCompressor;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let args: Vec<String> = env::args().collect();
    let path_to_file = &args[1];
    println!("Compressing file {}", path_to_file);
    let mut file_in = &File::open(path_to_file)?;
    let mut input_buffer = Vec::new();
    let mut compressor = LzCompressor::new();
    let mut file_writer = FileWriter::new("data/data_out.bin")?;
    file_in.read_to_end(&mut input_buffer)?;

    let compressed_data = compressor.compress_data(input_buffer.as_ref());
    file_writer.write_header(&compressor.parameters)?;
    file_writer.write_packed_contents_to_file(&compressed_data)?;
    Ok(())
}

