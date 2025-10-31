use std::path;
use std::env;
use clap::{Parser, Subcommand};
use crate::util;

#[derive(Parser)]
struct CypherCli {
    #[clap(value_enum)]
    verbosity: util::Verbosity,
    #[command(subcommand)]
    command: CypherCliCommand,
}

#[derive(Subcommand)]
enum CypherCliCommand {
    Enc { file: String },
    Dec { file: String },
}

pub enum CypherCommand {
    Enc,
    Dec,
}

pub struct CypherInputs {
    pub file_path: path::PathBuf,
    pub command: CypherCommand,
    pub verbosity: util::Verbosity,
}

pub fn get_inputs() -> CypherInputs {
    let cli = CypherCli::parse();

    let verbosity = cli.verbosity;
    util::init_logger(&verbosity);
    let (command, file) = match cli.command {
        CypherCliCommand::Enc { file } => (CypherCommand::Enc, file),
        CypherCliCommand::Dec { file } => (CypherCommand::Dec, file),
    };
    CypherInputs {
        command,
        file_path: get_file_path(file),
        verbosity
    }
}

fn get_file_path(input: String) -> path::PathBuf {
    if input.starts_with("/") {
        return path::PathBuf::from(input)
    }
    let cwd = env::current_dir().expect("could not resolve current directory");
    let file_path = cwd.join(input);
    match file_path.try_exists() {
        Ok(true) => {
            return file_path
        },
        _ => panic!("Invalid path {:?}", file_path)
    }
}
