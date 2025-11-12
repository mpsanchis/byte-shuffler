use super::swap_util::SwappingDirection;

pub fn rotate_bits(mut bytes: Vec<u8>, direction: &SwappingDirection) -> Vec<u8> {
    let rotator = match direction {
        SwappingDirection::Left => |b: &u8| b.rotate_left(1),
        SwappingDirection::Right => |b: &u8| b.rotate_right(1),
    };
    for b in bytes.iter_mut() {
        *b = rotator(b);
    }
    bytes
}

/**
 * Swap the bytes of a file, and the bits of each byte.
 * Everything is rotated right one unit (one byte and one bit).
 */
pub fn rotate_bytes_right(mut file_bytes: Vec<u8>) -> Vec<u8> {
    let n = file_bytes.len();
    let last_byte = file_bytes[n - 1];

    for i in (1..n).rev() {
        file_bytes[i] = file_bytes[i - 1];
    }
    file_bytes[0] = last_byte;
    file_bytes
}

/**
 * Swap the bytes of a file, and the bits of each byte.
 * Everything is rotated left one unit (one byte and one bit).
 */
pub fn rotate_bytes_left(mut file_bytes: Vec<u8>) -> Vec<u8> {
    let n = file_bytes.len();
    let first_byte = file_bytes[0];

    for i in 0..(n - 1) {
        file_bytes[i] = file_bytes[i + 1];
    }
    file_bytes[n - 1] = first_byte;
    file_bytes
}
