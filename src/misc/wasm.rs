use std::io::Read;

use flate2::read::GzDecoder;
use wasmparser::{types::Types, Validator};

pub fn validate_gzipped_wasm(wasm_bytes: &[u8]) -> Result<Types, String> {
    let decompressed = decompress_wasm_gz(wasm_bytes)?;
    let mut validator = Validator::new();
    validator
        .validate_all(decompressed.as_slice())
        .map_err(|e| e.to_string())
}

pub fn decompress_wasm_gz(gzipped_wasm: &[u8]) -> Result<Vec<u8>, String> {
    let mut decoder = GzDecoder::new(gzipped_wasm);
    let mut decompressed_wasm = Vec::new();
    decoder
        .read_to_end(&mut decompressed_wasm)
        .map_err(|e| e.to_string())?;
    Ok(decompressed_wasm)
}
