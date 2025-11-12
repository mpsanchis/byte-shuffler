pub const SIGNATURE: &[u8] = b"ByteShuffler";

pub fn are_bytes_written_by_byte_shuffler(file_bytes: &[u8]) -> bool {
    file_bytes.starts_with(SIGNATURE)
}

pub fn trim_signature(bytes: &[u8]) -> &[u8] {
    bytes.strip_prefix(SIGNATURE).unwrap_or(bytes)
}
