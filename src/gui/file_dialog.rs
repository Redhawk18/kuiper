use std::io::Result;
use std::string::String;

use log;
use rfd::FileDialog;

use crate::core::read_file;

pub fn open_file() -> Result<String> {
    let path_opt = FileDialog::new().pick_file();

    let Some(path) = path_opt
        else {
            log::warn!("File not picked"); 
            return Err(std::io::ErrorKind::NotFound.into());
        };

    log::info!("File path: {:?}", path);

    read_file(path)
}

pub fn open_folder() {
    let path_opt = FileDialog::new().pick_folder();

    let Some(path) = path_opt
        else { return };

    println!("{:?}", path);

    todo!(); //update gui
}
