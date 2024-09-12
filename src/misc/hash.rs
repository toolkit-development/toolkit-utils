use sha2::{digest::Digest, Sha256};

pub fn generate_checksum(bytes: &Vec<u8>) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}
