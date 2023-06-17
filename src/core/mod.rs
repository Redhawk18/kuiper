use std::fs;
use std::io::Result;
use std::path::Path;
use std::string::String;



/// Reads the contents of the file at the path given and returns it
pub fn read_file(path: &Path) -> Result<String> {
    log::info!("reading file at: {:?}", path);
    fs::read_to_string(path)
}

/// Saves the contents to the file and returns the results.
pub fn save_file(path: &Path, contents: &str) -> Result<()> {
    log::info!("saving file at: {:?}", path);
    fs::write(path, contents)
}
