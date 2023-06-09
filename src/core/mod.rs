use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::string::String;

use log;

/// Returns the contents of the file given a valid PathBuf.
pub fn read_file(path: PathBuf) -> Result<String> {
    log::info!("reading file at: {:?}", path);

    let contents = fs::read_to_string(path);

    contents
}

pub fn save_file(path: PathBuf, contents: &str) -> Result<()> {
    log::info!("saving file at: {:?}", path);
    let result = fs::write(path, contents);
    
    result
}
