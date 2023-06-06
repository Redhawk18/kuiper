use std::fs;
use std::path::PathBuf;
use std::string::String;

/// Returns the contents of the file given a valid PathBuf, errors are not handled.
pub fn read_file(path: PathBuf) -> String {
    println!("In file {:?}", path);

    let contents: String = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    contents
}
