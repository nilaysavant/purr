pub mod errors;
mod print;

use anyhow::anyhow;
use clap::Parser;
use encoding_rs::UTF_8;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use std::{
    fs::File,
    io::{self, BufReader, Stdin},
    path::{Path, PathBuf},
};

pub use print::PrintManager;

/// Purr(cat) FILE(s) to standard output.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct PurrArgs {
    /// File(s) to concatenate to standard output.
    pub files: Option<Vec<String>>,
    /// Number all output lines, starting with 1.
    #[arg(short = 'n', long = "number")]
    pub number_output_lines: bool,
}

/// Read file from given `path`.
pub fn read_file<P>(
    path: P,
) -> anyhow::Result<BufReader<DecodeReaderBytes<std::fs::File, std::vec::Vec<u8>>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let file_decoder = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(file);
    let buf_reader = BufReader::new(file_decoder);
    Ok(buf_reader)
}

/// Read standard input.
pub fn read_stdin() -> anyhow::Result<BufReader<Stdin>> {
    let stdin = BufReader::new(io::stdin());
    Ok(stdin)
}

/// Takes file path string and returns a validated path.
///
/// If path string is `-` or does not exist, returns `None`.
pub fn validate_file_path(path_str: String) -> anyhow::Result<PathBuf> {
    let path = Path::new(&path_str);
    if path.exists() {
        Ok(path.into())
    } else {
        Err(anyhow!("file path does not exist!"))
    }
}
