use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone, Default)]
pub struct FileTab {
    pub path: Option<PathBuf>,
    pub text: String,
}
