use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;

mod lz_dictionary;
use lz_dictionary::{LzDictionary, EMPTY_PATTERN};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let mut file_in = &File::open("file_in.bin")?; 
    let mut input_buffer = Vec::new();
    let mut compressor = LzCompressor::new();
    
    file_in.read_to_end(&mut input_buffer)?;
 
    let compressed_data = compressor.compress_data(input_buffer.as_ref());
    for code in compressed_data.iter() {
        
    }
    Ok(())
}

struct FileWriter {
    
}

impl FileWriter {
    pub fn write_packed_contents_to_file(data_to_write: &[u16]) -> io::Result<()> {
        let mut file_out = File::create("data_out.bin")?;
        let mut last_char : u16 = 0;
        let current_nof_bits = 9;
        let mut stitching_bit = 0;
        for code in data_to_write.iter() {
            let current_char = *code;
            let byte_out : u8 = (current_char << stitching_bit) as u8 | (last_char >> (current_nof_bits - stitching_bit)) as u8;
            file_out.write(&[byte_out])?;
            last_char = current_char;
            stitching_bit = (stitching_bit + current_nof_bits % u8::BITS) % u8::BITS;
        }
        Ok(())
    } 
}

struct LzCompressor {
    dictionary: LzDictionary
}

impl LzCompressor {
    pub fn new() -> LzCompressor {
        LzCompressor { dictionary: LzDictionary::new() }
    }
    
    pub fn compress_data(&mut self, data_to_compress: &[u8]) -> Vec<u16> {
        let mut ret = Vec::new();
        let mut current_pattern :  u16 = EMPTY_PATTERN;
        for byte in data_to_compress {
            let new_pattern = (current_pattern, *byte as u16);
            if let Some(char_idx) = self.dictionary.is_pattern_in_dictionary(new_pattern) {
                current_pattern = char_idx;
            }
            else {
                self.dictionary.add_pattern_to_dictionary(new_pattern);
                ret.push(current_pattern);
                current_pattern = *byte as u16;
            }
            println!("current_pattern {}", current_pattern);
        }
        if current_pattern != EMPTY_PATTERN {
            ret.push(current_pattern);
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! compression_tests {
        ($($name:ident: $data_in:expr,$data_out:expr,)*) => {
        $(
            #[test]
            fn $name() {

                let mut compressor_to_test = LzCompressor::new();
                let data_in : &[u8] = &$data_in;
                let data_out : &[u16] = &$data_out;
                assert_eq!(data_out, compressor_to_test.compress_data(&data_in));
            }
        )*
        }
    }

    compression_tests! {
        test_empty: [], [],
        test_single: [1], [1],
        test_double: [1,2], [1,2],
        test_triple: [1,2,3], [1,2,3],
        test_actual_compression: [1,2,3,1,2,3], [1,2,3,256,3],
    }
}