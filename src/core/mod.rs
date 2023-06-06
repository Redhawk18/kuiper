use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf) {
    println!("In file {:?}", path);

    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
