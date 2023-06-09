use std::io::Result;
use std::fs;
use std::path::PathBuf;
use std::string::String;

use log;

/// Returns the contents of the file given a valid PathBuf.
pub fn read_file(path: PathBuf) -> Result<String> {
    log::info!("reading file at: {:?}", path);

    let contents = fs::read_to_string(path);

    contents
}
