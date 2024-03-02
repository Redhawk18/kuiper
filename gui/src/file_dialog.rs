use rfd::AsyncFileDialog;
use snafu::{ResultExt, Snafu};
use std::{path::PathBuf, sync::Arc};
use tokio::fs;

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

pub async fn open_file() -> Result<(PathBuf, String), Error> {
    let handle = AsyncFileDialog::new()
        .set_title("my title")
        .pick_file()
        .await;

    let Some(file) = handle else {
        return Err(Error::DialogClosed);
    };

    let path = file.path();
    let contents = fs::read_to_string(path).await.context(ReadSnafu { path });

    Ok((path.to_path_buf(), contents.unwrap()))
}

pub async fn save_file(path: Option<PathBuf>, contents: String) -> Result<(), Error> {
    let path = match path {
        Some(path) => path,
        None => {
            let Some(handle) = AsyncFileDialog::new().save_file().await else {
                return Err(Error::DialogClosed);
            };
            handle.path().to_path_buf()
        }
    };

    fs::write(&path, contents)
        .await
        .context(WriteSnafu { path })
}

pub async fn save_file_with_dialog(path: Option<PathBuf>, contents: String) -> Result<(), Error> {
    let handle = match path {
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

    fs::write(&path, contents)
        .await
        .context(WriteSnafu { path })
}
