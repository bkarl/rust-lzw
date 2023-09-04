use std::io;
use std::io::prelude::*;
use std::fs::File;

pub struct FileWriter {
   file: File 
}

impl FileWriter {
    pub fn new(filename: &str) -> std::io::Result<FileWriter> {
        Ok (FileWriter { file: File::create(filename)? })
    }
    
    pub fn write_packed_contents_to_file(&mut self, data_to_write: &[u16]) -> io::Result<()> {
        let mut last_char : u16 = 0;
        let current_nof_bits = 9;
        let mut stitching_bit = 0;
        for code in data_to_write.iter() {
            let current_char = *code;
            let byte_out : u8 = (current_char << stitching_bit) as u8 | (last_char >> (current_nof_bits - stitching_bit)) as u8;
            self.file.write(&[byte_out])?;
            last_char = current_char;
            stitching_bit = (stitching_bit + current_nof_bits % u8::BITS) % u8::BITS;
        }
        Ok(())
    } 
}
