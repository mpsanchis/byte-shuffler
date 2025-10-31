use std::fs;
use std::io;

mod input;
mod util;
mod swap;

use input::{CypherInputs, get_inputs};
use swap::swap_bytes;

fn main() -> Result<(), io::Error> {

    let CypherInputs { command, file_path } = get_inputs();

    let mut file_bytes = fs::read(&file_path)?;

    if file_bytes.is_empty() {
        println!("No Bytes could be read from file. Doing nothing");
        return Ok(());
    }
    // TODO: use "command" to choose swapping direction (encode/decode)
    file_bytes = swap_bytes(file_bytes);

    fs::write(file_path, file_bytes)?;

    Ok(())
}
