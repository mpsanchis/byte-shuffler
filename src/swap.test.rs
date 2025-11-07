#[cfg(test)]
mod tests {
    use super::super::{SwappingDirection, swap_bytes};
    use crate::util::init_logger;

    #[ctor::ctor] // runs at the beginning
    fn init_dummy_logger() {
        init_logger(crate::util::Verbosity::Off);
    }

    #[test]
    fn it_swaps_bytes_right_zeros() {
        let b: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(swap_bytes(b, SwappingDirection::Right), vec![0, 0, 0, 0]);
    }
    #[test]
    fn it_swaps_bytes_right_one_byte() {
        let b: Vec<u8> = vec![0b1000_0000];
        assert_eq!(swap_bytes(b, SwappingDirection::Right), vec![0b0100_0000]);
    }
    #[test]
    fn it_swaps_bytes_right_one_byte_around() {
        let b: Vec<u8> = vec![0b0000_0011];
        assert_eq!(swap_bytes(b, SwappingDirection::Right), vec![0b1000_0001]);
    }
    #[test]
    fn it_swaps_bytes_right_one_one() {
        let b: Vec<u8> = vec![0b1000_0000, 0b0000_0001];
        assert_eq!(
            swap_bytes(b, SwappingDirection::Right),
            vec![0b1000_0000, 0b0100_0000]
        );
    }
    #[test]
    fn it_swaps_bytes_right_letter_b() {
        let input = Vec::from("b".as_bytes());
        assert_eq!(input, vec![0b0110_0010]); // just to document
        assert_eq!(
            swap_bytes(input, SwappingDirection::Right),
            vec![0b0011_0001]
        );
    }
    #[test]
    fn it_swaps_bytes_right_letter_c() {
        let input = Vec::from("c".as_bytes());
        assert_eq!(
            swap_bytes(input, SwappingDirection::Right),
            "c".as_bytes()
                .iter()
                .map(|b| b.rotate_right(1))
                .collect::<Vec<u8>>()
        );
    }
    #[test]
    fn it_swaps_bytes_right_letters_ab() {
        let input = Vec::from("ab".as_bytes());
        assert_eq!(input, vec![0b0110_0001, 0b0110_0010]); // just to document
        assert_eq!(
            swap_bytes(input.clone(), SwappingDirection::Right),
            vec![0b0011_0001, 0b1011_0000]
        );
        assert_eq!(
            swap_bytes(input.clone(), SwappingDirection::Right),
            "ba".as_bytes()
                .iter()
                .map(|b| b.rotate_right(1))
                .collect::<Vec<u8>>()
        );
    }
    #[test]
    fn it_swaps_bytes_right_letters_abc() {
        let input = Vec::from("abc".as_bytes());
        let expected = "cab"
            .as_bytes()
            .iter()
            .map(|b| b.rotate_right(1))
            .collect::<Vec<u8>>();
        assert_eq!(
            swap_bytes(input.clone(), SwappingDirection::Right),
            expected
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
        assert_eq!(swap_bytes(input, SwappingDirection::Right), expected);

        //let b: Vec<u8> = vec![0b1000_0000, 0b0000_0001];
        //assert_eq!(swap_bytes(b, SwappingDirection::Right), vec![0b1000_0000, 0b0100_0000]);
    }
    #[test]
    fn it_swaps_bytes_left_zeros() {
        let b: Vec<u8> = vec![0, 0, 0, 0];
        assert_eq!(swap_bytes(b, SwappingDirection::Left), vec![0, 0, 0, 0]);
    }
    #[test]
    fn it_swaps_bytes_left_one_byte() {
        let b: Vec<u8> = vec![0b1000_0000];
        assert_eq!(swap_bytes(b, SwappingDirection::Left), vec![0b0000_0001]);
    }
    #[test]
    fn it_swaps_bytes_left_one_one() {
        let b: Vec<u8> = vec![0b1000_0000, 0b0000_0001];
        assert_eq!(
            swap_bytes(b, SwappingDirection::Left),
            vec![0b0000_0010, 0b0000_0001]
        );
    }
}
