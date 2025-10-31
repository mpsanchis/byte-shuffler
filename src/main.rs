use std::fs;
use std::io;

mod input;
mod util;

use input::get_inputs;

use crate::input::CypherInputs;

fn swap_bytes(mut file_bytes: Vec<u8>) -> Vec<u8> {
    let first_byte = file_bytes[0];
    let num_bytes = file_bytes.len();

    for i in 0..(num_bytes-2) {
        file_bytes[i] = file_bytes[i+1].rotate_right(1);
    }
    file_bytes[num_bytes-1] = first_byte;
    file_bytes
}

fn main() -> Result<(), io::Error> {

    let CypherInputs { command, file_path, verbosity } = get_inputs();
    print!("Getting file from: {:?}", &file_path);

    let mut file_bytes = fs::read(&file_path)?;

    if file_bytes.is_empty() {
        println!("No Bytes could be read from file. Doing nothing");
        return Ok(());
    }
    file_bytes = swap_bytes(file_bytes);

    fs::write(file_path, file_bytes)?;

    Ok(())
}
