use base64::Engine;

pub fn base64_to_bytes(logo: &String) -> Option<Vec<u8>> {
    let engine: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;
    engine.decode(logo).ok()
}

pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let engine: base64::engine::GeneralPurpose = base64::engine::general_purpose::STANDARD;
    engine.encode(&bytes)
}
