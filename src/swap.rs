use crate::util::prompt::ask_yes_no as user_confirms;
use crate::util::signature::{SIGNATURE, are_bytes_written_by_byte_shuffler, trim_signature};
use crate::{input::CypherCommand, util::logger};

mod byte_bit_rotation;
mod swap_util;

use byte_bit_rotation::{rotate_bits, rotate_bytes_left, rotate_bytes_right};
pub use swap_util::SwappingDirection;

fn swap_bytes_internal(mut file_bytes: Vec<u8>, direction: SwappingDirection) -> Vec<u8> {
    logger().logn(&format!("Swapping bytes to the {:?}", direction));

    file_bytes = rotate_bits(file_bytes, &direction);
    match direction {
        SwappingDirection::Left => rotate_bytes_left(file_bytes),
        SwappingDirection::Right => rotate_bytes_right(file_bytes),
    }
}

fn force_decode() -> bool {
    user_confirms(
        "It seems that the file was not encoded by Byte Shuffler. Do you want to force decode it?",
    )
}

fn force_encode() -> bool {
    user_confirms(
        "It seems that the file was already encoded by Byte Shuffler. Do you want to force encode it?",
    )
}

pub fn swap_bytes(file_bytes: Vec<u8>, command: &CypherCommand) -> Vec<u8> {
    match command {
        CypherCommand::Enc => {
            let file_bytes_swapped = if are_bytes_written_by_byte_shuffler(&file_bytes) {
                if force_encode() {
                    swap_bytes_internal(file_bytes, SwappingDirection::Right)
                } else {
                    file_bytes
                }
            } else {
                swap_bytes_internal(file_bytes, SwappingDirection::Right)
            };
            [SIGNATURE, &file_bytes_swapped].concat()
        },
        CypherCommand::Dec => {
            if are_bytes_written_by_byte_shuffler(&file_bytes) || force_decode() {
                swap_bytes_internal(Vec::from(trim_signature(&file_bytes)), SwappingDirection::Left)
            } else {
                file_bytes
            }
        }
    }
}

// CODE TO TEST
// TODO: simplify this swapping function, and use it (if it makes sense) to encode/decode
// can substitute "swap_bytes_internal"
#[allow(dead_code)]
fn swap<T>(mut v: Vec<T>, dir: SwappingDirection) -> Vec<T>
where
    T: Clone + Copy,
{
    let n = v.len();

    let (dir, range, i_to_save): (isize, Box<dyn Iterator<Item = usize>>, usize) = match dir {
        SwappingDirection::Right => (1, Box::new(0..n), 0),
        SwappingDirection::Left => (-1, Box::new((0..n).rev()), n - 1),
    };
    let to_save = v[i_to_save];
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

#[cfg(test)]
mod tests {
    use super::{SwappingDirection, swap_bytes_internal};
    use crate::util::init_logger;

    #[ctor::ctor] // runs at the beginning
    fn init_dummy_logger() {
        init_logger(crate::util::Verbosity::Off);
    }

    // Swapping right
    #[test]
    fn it_swaps_bytes_right_zeros() {
        let b: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Right),
            vec![0, 0, 0, 0]
        );
    }
    #[test]
    fn it_swaps_bytes_right_one_byte() {
        let b: Vec<u8> = vec![0b1000_0000];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Right),
            vec![0b0100_0000]
        );
    }
    #[test]
    fn it_swaps_bytes_right_one_byte_around() {
        let b: Vec<u8> = vec![0b0000_0011];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Right),
            vec![0b1000_0001]
        );
    }
    #[test]
    fn it_swaps_bytes_right_one_one() {
        let b: Vec<u8> = vec![0b1000_0000, 0b0000_0001];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Right),
            vec![0b1000_0000, 0b0100_0000]
        );
    }
    #[test]
    fn it_swaps_bytes_right_letter_b() {
        let input = Vec::from("b".as_bytes());
        assert_eq!(input, vec![0b0110_0010]); // just to document
        assert_eq!(
            swap_bytes_internal(input, SwappingDirection::Right),
            vec![0b0011_0001]
        );
    }
    #[test]
    fn it_swaps_bytes_right_letters_ab() {
        let input = Vec::from("ab".as_bytes());
        assert_eq!(input, vec![0b0110_0001, 0b0110_0010]); // just to document
        assert_eq!(
            swap_bytes_internal(input.clone(), SwappingDirection::Right),
            vec![0b0011_0001, 0b1011_0000]
        );
        assert_eq!(
            swap_bytes_internal(input.clone(), SwappingDirection::Right),
            "ba".as_bytes()
                .iter()
                .map(|b| b.rotate_right(1))
                .collect::<Vec<u8>>()
        );
    }
    #[test]
    fn it_swaps_bytes_right_letters_abcd() {
        let input = Vec::from("abcd".as_bytes());
        let expected: Vec<u8> = "dabc"
            .as_bytes()
            .iter()
            .map(|b| b.rotate_right(1))
            .collect();
        assert_eq!(
            swap_bytes_internal(input, SwappingDirection::Right),
            expected
        );
    }
    #[test]
    fn it_swaps_bytes_right_hello_world() {
        let input = Vec::from("hello world".as_bytes());
        let expected: Vec<u8> = "dhello worl"
            .as_bytes()
            .iter()
            .map(|b| b.rotate_right(1))
            .collect();
        assert_eq!(
            swap_bytes_internal(input, SwappingDirection::Right),
            expected
        );
    }
    // Swapping left
    #[test]
    fn it_swaps_bytes_left_zeros() {
        let b: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Left),
            vec![0, 0, 0, 0]
        );
    }
    #[test]
    fn it_swaps_bytes_left_one_byte() {
        let b: Vec<u8> = vec![0b1000_0000];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Left),
            vec![0b0000_0001]
        );
    }
    #[test]
    fn it_swaps_bytes_left_one_one() {
        let b: Vec<u8> = vec![0b1000_0000, 0b0000_0001];
        assert_eq!(
            swap_bytes_internal(b, SwappingDirection::Left),
            vec![0b0000_0010, 0b0000_0001]
        );
    }
    #[test]
    fn it_swaps_bytes_left_letters_abcd() {
        let input = Vec::from("abcd".as_bytes());
        let expected: Vec<u8> = "bcda".as_bytes().iter().map(|b| b.rotate_left(1)).collect();
        assert_eq!(
            swap_bytes_internal(input, SwappingDirection::Left),
            expected
        );
    }
}
