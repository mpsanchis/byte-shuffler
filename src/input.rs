use std::path;
use std::env;
use std::fmt::Display;
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

impl Display for CypherCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CypherCommand::Enc => write!(f, "encode"),
            CypherCommand::Dec => write!(f, "decode"),
        }
    }
}

pub struct CypherInputs {
    pub file_path: path::PathBuf,
    pub command: CypherCommand,
}

pub fn get_inputs() -> CypherInputs {
    let cli = CypherCli::parse();

    let verbosity = cli.verbosity;
    util::init_logger(verbosity);
    let (command, file) = match cli.command {
        CypherCliCommand::Enc { file } => (CypherCommand::Enc, file),
        CypherCliCommand::Dec { file } => (CypherCommand::Dec, file),
    };
    let file_path = get_file_path(file);

    let logger = util::get_logger();
    logger.log(&format!("running command {}", command));
    logger.log(&format!("file path {}", file_path.display()));
    CypherInputs {
        command,
        file_path,
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
