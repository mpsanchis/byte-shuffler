use crate::util::logger::Verbosity;

use clap::{Parser, ValueEnum};
use std::env;
use std::fmt::Display;
use std::io;
use std::path;
use std::path::PathBuf;

#[derive(Parser)]
pub struct CypherCli {
    #[arg(value_enum)]
    pub command: CypherCommand,
    #[arg(value_parser = get_file_path, help = "If not absolute, will be assumed to be relative to cwd")]
    pub file_path: PathBuf,
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Whether to overwrite the file being read"
    )]
    pub overwrite: bool,
    #[arg(short, long, value_enum, default_value = "on")]
    pub verbosity: Verbosity,
}

#[derive(Clone, ValueEnum)]
pub enum CypherCommand {
    Enc,
    Dec,
}

impl Display for CypherCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CypherCommand::Enc => write!(f, "encode"),
            CypherCommand::Dec => write!(f, "decode"),
        }
    }
}

fn get_file_path(input: &str) -> Result<path::PathBuf, io::Error> {
    let file_path = if input.starts_with("/") {
        path::PathBuf::from(input)
    } else {
        let cwd = env::current_dir()?;
        cwd.join(input)
    };

    match file_path.try_exists() {
        Ok(true) => Ok(file_path),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Invalid path {:?}", file_path),
        )),
    }
}
