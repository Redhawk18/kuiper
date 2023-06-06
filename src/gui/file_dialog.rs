use std::string::String;

use rfd::FileDialog;

use crate::core::read_file;

pub fn open_file() -> String {
    let path_opt = FileDialog::new().pick_file();

    let Some(path) = path_opt
        else { return "".to_string()};

    println!("{:?}", path);

    // todo!(); //update gui
    read_file(path)
}

pub fn open_folder() {
    let path_opt = FileDialog::new().pick_folder();

    let Some(path) = path_opt
        else { return };

    println!("{:?}", path);

    todo!(); //update gui
}
