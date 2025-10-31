enum Direction {
    Right,
    Left,
}

// TODO: simplify this swapping function, and use it (if it makes sense) to encode/decode
// can substitute "swap_bytes"
fn swap<T>(mut v: Vec<T>, dir: Direction) -> Vec<T>
where
    T: Clone + Copy,
{
    let N = v.len();

    let (dir, range, i_to_save): (isize, Box<dyn Iterator<Item = usize>>, usize) = match dir {
        Direction::Right => (1, Box::new(0..N), 0),
        Direction::Left => (-1, Box::new((0..N).rev()), N-1),
    };
    let to_save = v[i_to_save].clone();
    for i in range {
        let j: usize = (((i as isize) + dir).rem_euclid(N as isize)).try_into().unwrap();
        println!("i = {i}, j = {j}");
        if j == i_to_save { v[i] = to_save }
        else { v[i] = v[j] }
    }
    v
}

/**
 * Swap the bytes of a file, and the bits of each byte.
 * Everything is rotated right one unit (one byte and one bit).
 */
pub fn swap_bytes(mut file_bytes: Vec<u8>) -> Vec<u8> {
    let first_byte = file_bytes[0];
    let num_bytes = file_bytes.len();

    for i in 0..(num_bytes-2) {
        file_bytes[i] = file_bytes[i+1].rotate_right(1);
    }
    file_bytes[num_bytes-1] = first_byte;
    file_bytes
}
