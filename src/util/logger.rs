use crate::util::signature::trim_signature;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Verbosity {
    Off,   // never log
    Error, // only log errors
    On,    // always log
}

#[derive(Debug)]
pub struct Logger {
    verbosity: Verbosity,
}

impl Logger {
    pub fn from(verbosity: Verbosity) -> Logger {
        Logger { verbosity }
    }

    pub fn log(&self, msg: &str) {
        if matches!(self.verbosity, Verbosity::On) {
            print!("{}", msg);
        }
    }
    pub fn logn(&self, msg: &str) {
        if matches!(self.verbosity, Verbosity::On) {
            println!("{}", msg);
        }
    }

    pub fn log_bytes(&self, raw_bytes: &[u8]) {
        if matches!(self.verbosity, Verbosity::On) {
            let bytes = trim_signature(raw_bytes);
            let n = bytes.len();

            // Keep a little array in the stack to avoid the heap (as an exercise)
            let mut bytes_summary = [0u8; 4];
            let count = if n > 4 {
                bytes_summary = [bytes[0], bytes[1], bytes[n - 2], bytes[n - 1]];
                4
            } else {
                bytes_summary[..n].copy_from_slice(bytes);
                n
            };
            for (i, byte) in &mut bytes_summary[..count].iter().enumerate() {
                match i {
                    2 => print!(".."),
                    1 | 3 => print!(" "),
                    _ => (),
                }
                print!("0x{:02X}", byte);
            }
        }
    }

    pub fn log_bytesn(&self, bytes: &[u8]) {
        self.log_bytes(bytes);
        if matches!(self.verbosity, Verbosity::On) {
            println!();
        }
    }

    pub fn err(&self, msg: &str) {
        if !matches!(self.verbosity, Verbosity::Off) {
            eprintln!("{}", msg);
        }
    }
}
