use std::{fs, io::Result, path::Path, string::String};

/// Reads the contents of the file and returns it's contents.
pub fn read_file(path: &Path) -> Result<String> {
    log::info!("reading file at: {:?}", path);
    fs::read_to_string(path)
}

/// Saves the contents to the file and returns the results.
pub fn save_file(path: &Path, contents: &str) -> Result<()> {
    log::info!("saving file at: {:?}", path);
    fs::write(path, contents)
}
