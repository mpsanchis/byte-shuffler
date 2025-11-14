use std::io::Write;
use std::{fs, io::Read};
use std::process::{Command, Stdio};
mod common;

use common::{BINARY_NAME, setup, VecDisplay, read_until_question_mark};
use byte_shuffler::SIGNATURE;

#[test]
fn test_signature_added() {
    // given
    let ctx = setup();
    let txt_file = ctx.get_path().join("foo.txt");
    fs::write(&txt_file, "hello world").unwrap();

    // when
    let mut bs = Command::new(BINARY_NAME);
    let cmd = bs
        .arg("enc")
        .arg(&txt_file.to_str().unwrap())
        .arg("--overwrite");
    // .status() waits for the command to finish (.spawn() would not) -> we need to wait because we will read foo.txt after the command finishes
    let status = cmd.status().unwrap();

    // then
    assert!(status.success(), "CLI failed with status {}", status);
    let txt_file_encoded = fs::read(txt_file).unwrap();
    assert!(txt_file_encoded.starts_with(SIGNATURE), "Signature not found. File contains {}", VecDisplay::new(&txt_file_encoded));
}

#[test]
fn test_signature_removed() {
    // given
    let ctx = setup();
    let txt_file = ctx.get_path().join("foo.txt");
    let hello_world_encoded: [u8; 11] = std::array::from_fn(|i| {
        b"dhello worl"[i].rotate_right(1)
    });
    // fake file encoded by the CLI
    fs::write(&txt_file, [SIGNATURE, &hello_world_encoded].concat()).unwrap();

    // when
    let mut bs = Command::new(BINARY_NAME);
    let cmd = bs
        .arg("dec")
        .arg(&txt_file.to_str().unwrap())
        .arg("--overwrite");
    // .status() waits for the command to finish (.spawn() would not) -> we need to wait because we will read foo.txt after the command finishes
    let status = cmd.status().unwrap();

    // then
    assert!(status.success(), "CLI failed with status {}", status);
    let txt_file_raw = fs::read(&txt_file).unwrap();
    let txt_file_decoded = String::from_utf8_lossy(&txt_file_raw).to_owned();
    assert_eq!(txt_file_decoded, "hello world", "Signature found. File contains {} ({})", VecDisplay::new(&txt_file_raw), txt_file_decoded);
}

#[test]
fn test_user_prompted_when_decoding_unsigned_file() {
    // given
    let ctx = setup();
    let txt_file = ctx.get_path().join("foo.txt");
    let hello_world_encoded: [u8; 11] = std::array::from_fn(|i| {
        b"dhello worl"[i].rotate_right(1)
    });
    // fake file encoded by the CLI
    fs::write(&txt_file, &hello_world_encoded).unwrap();

    // when
    let mut bs = Command::new(BINARY_NAME);
    let cmd = bs
        .arg("dec")
        .arg(&txt_file.to_str().unwrap())
        .arg("--overwrite")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped());

    let mut bs_process = cmd.spawn().unwrap();
    let stdout = read_until_question_mark(&mut bs_process).unwrap();
    if stdout.contains("Do you want to force decode") {
        let mut stdin = bs_process.stdin.take().unwrap();
        stdin.write_all(b"y\n").unwrap();
    } else {
        panic!("Unexpected output: {}", stdout);
    }

    // then
    let txt_file_raw = fs::read(&txt_file).unwrap();
    let txt_file_decoded = String::from_utf8_lossy(&txt_file_raw).to_owned();
    assert_eq!(txt_file_decoded, "hello world", "Signature found. File contains {} ({})", VecDisplay::new(&txt_file_raw), txt_file_decoded);
}
