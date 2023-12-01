use crate::compressor::Compressor;

pub fn compress(src: Vec<u8>, output: &mut Vec<u8>) {
    let mut compressor = Compressor::new();

    for byte in src {
        compressor.compress_byte(byte);

        for compressed_byte in &mut compressor {
            output.push(compressed_byte);
        }
    }

    compressor.append_terminal_code();

    compressor.end();

    for compressed_byte in &mut compressor {
        output.push(compressed_byte);
    }
}
