use std::io::Result;
use std::path::{Path, PathBuf};
use std::string::String;

use log;
use rfd::FileDialog;

use crate::core;

/// Displays the user's os native file dialog to pick a file to open.
/// Returns the contents of the file and the absolute path.
pub fn pick_file() -> (Result<String>, PathBuf) {
    let path_opt = FileDialog::new().pick_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not picked"); 
            return (Err(std::io::ErrorKind::NotFound.into()), PathBuf::default());
        };

    log::info!("File path: {:?}", path);
    (core::read_file(&path), path)
}

/// Displays the user's os native file dialog to pick a folder to open.
pub fn pick_folder() {
    let path_opt = FileDialog::new().pick_folder();

    let Some(path) = path_opt
        else { return };

    println!("{:?}", path);

    todo!(); //update gui
}

/// Opens the file dialog if the path given is blank,
/// Otherwise it simpley saves the file.
pub fn pick_file_to_save(contents: &str, path: &Path) -> Result<()> {
    if path != PathBuf::default() {
        return core::save_file(&path, contents);
    }

    let path_opt = FileDialog::new().save_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not saved"); 
            return Err(std::io::ErrorKind::Interrupted.into());
        };

    log::info!("File path: {:?}", path);
    core::save_file(&path, contents)
}
