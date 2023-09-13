use std::fs::File;
use std::io::prelude::*;

mod lz_dictionary;
use lz_dictionary::{LzDictionary, EMPTY_PATTERN};
mod file_writer;
use file_writer::FileWriter;

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

struct LzCompressor {
    dictionary: LzDictionary,
}

impl LzCompressor {
    pub fn new() -> LzCompressor {
        LzCompressor {
            dictionary: LzDictionary::new(),
        }
    }

    pub fn compress_data(&mut self, data_to_compress: &[u8]) -> Vec<u16> {
        let mut ret = Vec::new();
        let mut current_pattern: u16 = EMPTY_PATTERN;
        for byte in data_to_compress {
            let new_pattern = (current_pattern, *byte as u16);
            if let Some(char_idx) = self.dictionary.is_pattern_in_dictionary(new_pattern) {
                current_pattern = char_idx;
            } else {
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
