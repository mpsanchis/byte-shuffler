use byte_shuffler::shuffle_bytes;
use byte_shuffler::CypherCli;
use clap::Parser;

fn main() {
    let CypherCli {
        command,
        file_path,
        overwrite,
        verbosity,
    } = CypherCli::parse();
    shuffle_bytes(command, file_path, overwrite, verbosity);
}
