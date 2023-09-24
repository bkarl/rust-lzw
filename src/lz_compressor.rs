use crate::lz_dictionary::{EMPTY_PATTERN, LzDictionary};


pub struct LzCompressorParameters {
    pub dict_size_bits: u8,
    pub use_block_code: bool
}

impl LzCompressorParameters {
    pub fn get_default_parameters() -> LzCompressorParameters {
        LzCompressorParameters { dict_size_bits: 9, use_block_code: false }        
    }
}

pub struct LzCompressor {
    pub parameters: LzCompressorParameters,
    dictionary: LzDictionary,
}

impl LzCompressor {
    pub fn new() -> LzCompressor {
        LzCompressor {
            parameters: LzCompressorParameters::get_default_parameters(),
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
                if self.dictionary.get_size() < 1 << self.parameters.dict_size_bits {                   
                    self.dictionary.add_pattern_to_dictionary(new_pattern);
                }
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
