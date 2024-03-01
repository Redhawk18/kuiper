use iced::widget::text_editor::Content;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Buffer {
    File(FileBuffer),
}

#[derive(Debug, Default)]
pub struct FileBuffer {
    pub path: Option<PathBuf>,
    pub content: Content,
}
