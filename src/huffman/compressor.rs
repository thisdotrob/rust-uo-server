mod buffer;

use super::huffman_table;
use buffer::CompressorBuffer;

const TERMINAL_CODE_BIT_COUNT: u8 = 4;
const TERMINAL_CODE_VALUE: u32 = 0xD;

pub struct Compressor {
    buffer: CompressorBuffer,
}

impl Compressor {
    pub fn new() -> Self {
        Compressor {
            buffer: CompressorBuffer::new(),
        }
    }

    pub fn compress_byte(&mut self, byte: u8) {
        let value = huffman_table::get_compressed_value(byte);
        let bit_count = huffman_table::get_compressed_value_bit_count(byte);
        self.buffer.write_bits(value, bit_count);
    }

    fn get_compressed_byte(&mut self) -> Option<u8> {
        self.buffer.read_byte()
    }

    pub fn append_terminal_code(&mut self) {
        self.buffer
            .write_bits(TERMINAL_CODE_VALUE, TERMINAL_CODE_BIT_COUNT);
    }

    pub fn end(&mut self) {
        let byte_boundary_offset = self.buffer.byte_boundary_offset();

        if byte_boundary_offset != 0 {
            let padding_value = 0b0;
            let padding_bit_count = 8 - byte_boundary_offset;
            self.buffer.write_bits(padding_value, padding_bit_count);
        }
    }
}

impl Iterator for Compressor {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        self.get_compressed_byte()
    }
}
