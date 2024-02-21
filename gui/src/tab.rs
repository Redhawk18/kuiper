use std::path::PathBuf;

#[derive(Debug)]
pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Default)]
pub struct FileTab {
    pub path: Option<PathBuf>,
    pub text: String,
}
