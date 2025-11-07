use crate::util::logger;
#[path = "swap.test.rs"]
mod swap_tests;

fn rotate_bits(mut bytes: Vec<u8>, direction: &SwappingDirection) -> Vec<u8> {
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
fn rotate_bytes_right(mut file_bytes: Vec<u8>) -> Vec<u8> {
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
fn rotate_bytes_left(mut file_bytes: Vec<u8>) -> Vec<u8> {
    let n = file_bytes.len();
    let first_byte = file_bytes[0];

    for i in 0..(n - 1) {
        file_bytes[i] = file_bytes[i + 1];
    }
    file_bytes[n - 1] = first_byte;
    file_bytes
}

#[derive(Debug)]
pub enum SwappingDirection {
    Left,
    Right,
}

pub fn swap_bytes(mut file_bytes: Vec<u8>, direction: SwappingDirection) -> Vec<u8> {
    logger().logn(&format!("Swapping bytes to the {:?}", direction));

    file_bytes = rotate_bits(file_bytes, &direction);
    match direction {
        SwappingDirection::Left => rotate_bytes_left(file_bytes),
        SwappingDirection::Right => rotate_bytes_right(file_bytes),
    }
}

// CODE TO TEST
#[allow(dead_code)]
enum Direction {
    Right,
    Left,
}

// TODO: simplify this swapping function, and use it (if it makes sense) to encode/decode
// can substitute "swap_bytes"
#[allow(dead_code)]
fn swap<T>(mut v: Vec<T>, dir: Direction) -> Vec<T>
where
    T: Clone + Copy,
{
    let n = v.len();

    let (dir, range, i_to_save): (isize, Box<dyn Iterator<Item = usize>>, usize) = match dir {
        Direction::Right => (1, Box::new(0..n), 0),
        Direction::Left => (-1, Box::new((0..n).rev()), n - 1),
    };
    let to_save = v[i_to_save].clone();
    for i in range {
        let j: usize = (((i as isize) + dir).rem_euclid(n as isize))
            .try_into()
            .unwrap();
        println!("i = {i}, j = {j}");
        if j == i_to_save {
            v[i] = to_save
        } else {
            v[i] = v[j]
        }
    }
    v
}
