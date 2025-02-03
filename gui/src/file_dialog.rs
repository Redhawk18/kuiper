use rfd::AsyncFileDialog;
use snafu::{ResultExt, Snafu};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;

pub type File = (Option<PathBuf>, String);
pub type Folder = PathBuf;

#[derive(Debug, Clone, Snafu)]
pub enum Error {
    #[snafu(display("File dialog was closed by user"))]
    DialogClosed,
    Read {
        #[snafu(source(from(std::io::Error, Arc::new)))]
        source: Arc<std::io::Error>,
        path: PathBuf,
    },
    Write {
        #[snafu(source(from(std::io::Error, Arc::new)))]
        source: Arc<std::io::Error>,
        path: PathBuf,
    },
}

/// Opens a file dialog from [AsyncFileDialog] and reads the contents and returns it.
pub async fn open_file() -> Result<File, Error> {
    let handle = AsyncFileDialog::new()
        .set_title("Open File")
        .pick_file()
        .await;

    let Some(file) = handle else {
        return Err(Error::DialogClosed);
    };

    let path = file.path();
    let contents = fs::read_to_string(path).await.context(ReadSnafu { path });

    Ok((Some(path.to_path_buf()), contents.unwrap()))
}

/// Opens a file dialog from [AsyncFileDialog] and returns the path.
pub async fn open_folder() -> Result<Folder, Error> {
    let handle = AsyncFileDialog::new()
        .set_title("Open Folder")
        .pick_folder()
        .await;

    let Some(folder) = handle else {
        return Err(Error::DialogClosed);
    };

    Ok(folder.path().to_path_buf())
}

/// Opens a file dialog if path is [None] from [AsyncFileDialog] and save the content of the file chosen to the filesystem.
pub async fn save_file(file: File) -> Result<(), Error> {
    let path = match file.0 {
        Some(path) => path,
        None => {
            let Some(handle) = AsyncFileDialog::new().save_file().await else {
                return Err(Error::DialogClosed);
            };
            handle.path().to_path_buf()
        }
    };

    fs::write(&path, file.1).await.context(WriteSnafu { path })
}

/// Opens a file dialog from [AsyncFileDialog] and save the content of the file chosen to the filesystem
pub async fn save_file_as(file: File) -> Result<(), Error> {
    let handle = match file.0 {
        Some(path) => {
            AsyncFileDialog::new()
                .add_filter("Text Filies | *.txt", &["txt"])
                .set_file_name(path.to_string_lossy())
                .set_title("Save As")
                .save_file()
                .await
        }
        None => {
            AsyncFileDialog::new()
                .set_title("Save As")
                .save_file()
                .await
        }
    };

    let Some(handle) = handle else {
        return Err(Error::DialogClosed);
    };
    let path = handle.path().to_path_buf();

    fs::write(&path, file.1).await.context(WriteSnafu { path })
}
