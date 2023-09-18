use crate::FileTab;
use blaze_core::io::{read_file, save_file};

use rfd::FileDialog;
use std::{io::Result, path::PathBuf, string::String};

/// Displays the user's os native file dialog to pick a file to open.
/// Returns the contents of the file and the absolute path.
pub fn pick_file_dialog() -> (Result<String>, PathBuf) {
    let path_opt = FileDialog::new().pick_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not picked");
            return (Err(std::io::ErrorKind::NotFound.into()), PathBuf::default());
        };

    log::info!("File path: {:?}", path);
    (read_file(&path), path)
}

/// Displays the user's os native file dialog to pick a folder to open.
pub fn pick_folder_dialog() {
    let path_opt = FileDialog::new().pick_folder();

    let Some(path) = path_opt
        else { return };

    println!("{:?}", path);

    todo!(); //update gui
}

/// Opens the file dialog if the path given is blank,
/// Otherwise it simpley saves the file.
pub fn save_file_dialog(file_tab: &FileTab) -> Result<()> {
    // incase we know where the current file is,
    // just save instead of opening the dialog.
    if file_tab.path != PathBuf::default() {
        return save_file(&file_tab.path, &file_tab.text);
    }

    let path_opt = FileDialog::new().save_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not saved"); 
            return Err(std::io::ErrorKind::Interrupted.into());
        };

    log::info!("File path: {:?}", path);
    save_file(&path, &file_tab.text)
}

/// Opens the file picker to choose file name and location,
/// if a location is picked the file is saved.
pub fn save_file_as_dialog(file_tab: &FileTab) -> Result<()> {
    let path_opt = FileDialog::new()
        .set_file_name(file_tab.path.to_str().unwrap())
        .save_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not saved"); 
            return Err(std::io::ErrorKind::Interrupted.into());
        };

    log::info!("File path: {:?}", path);
    save_file(&path, &file_tab.text)
}
