use std::fs::File;
use std::io;
use std::io::prelude::*;


pub const MAGIC_WORD : [u8;2]= [0x1f, 0x9d];

pub struct FileWriter<W: std::io::Write> {
    file: W,
}

impl FileWriter<File> {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = File::create(path)?;
        Ok(FileWriter{ file: file })
    }

}

impl<W: Write> FileWriter<W> {

    fn write_header(&mut self) -> io::Result<()> {
        self.file.write(&MAGIC_WORD)?;
        Ok(())
    }
    
    pub fn write_packed_contents_to_file(&mut self, data_to_write: &[u16]) -> io::Result<()> {
        let mut last_char: u16 = 0;
        let current_nof_bits = 9;
        let mut stitching_bit = 0;
        for code in data_to_write.iter() {
            let current_char = *code;
            let byte_out: u8 = (current_char << stitching_bit) as u8 | (last_char >> (current_nof_bits - stitching_bit)) as u8;
            self.file.write(&[byte_out])?;
            last_char = current_char;
            stitching_bit = (stitching_bit + current_nof_bits % u8::BITS) % u8::BITS;
        }
        if stitching_bit != 0 {
            self.file.write(&[(last_char >> (current_nof_bits - stitching_bit)) as u8])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockedFileWriter {
        contents: Vec<u8>
    }

    impl Write for MockedFileWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.contents.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())           
        }
    }

    macro_rules! writer_tests {
        ($($name:ident: $data_in:expr,$data_out:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let mut mock_writer = Vec::new();
                let mut writer_to_test = FileWriter {file: &mut mock_writer};
                let data_in : &[u16] = &$data_in;
                let data_out : &[u8] = &$data_out;
                writer_to_test.write_packed_contents_to_file(data_in).unwrap();
                assert_eq!(data_out, mock_writer);
            }
        )*
        }
    }

    writer_tests! {
        test_empty: [], [],
        test_single: [1], [1, 0],
        test_double: [1, 1], [1, 2, 0],
        test_triple: [1, 2, 3], [1, 4, 0xC, 0x00],
    }

    #[test]
    fn test_write_header() {
        let mut mock_writer = Vec::new();
        let mut writer_to_test = FileWriter {file: &mut mock_writer};
        writer_to_test.write_header().unwrap();
        assert_eq!(vec![0x1f, 0x9d, 0x09], mock_writer);
    }

}
