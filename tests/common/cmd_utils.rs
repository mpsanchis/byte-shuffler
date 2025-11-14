
use std::io;
use std::io::Read;
use std::process;

pub fn read_until_question_mark(process: &mut process::Child) -> Result<String, io::Error> {
    let mut stdout_handle = process.stdout.take().unwrap();

    let mut buffer = String::new();
    let mut byte = [0u8; 1];

    while let Ok(1) = stdout_handle.read(&mut byte) {
        if byte[0] == b'?' {
            break;
        }
        buffer.push(byte[0] as char);
    }

    Ok(buffer)
}