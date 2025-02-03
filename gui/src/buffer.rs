use iced::widget::text_editor;
use std::path::PathBuf;

use crate::file_dialog::File;

#[derive(Debug)]
pub enum Buffer {
    File(FileBuffer),
}

#[derive(Debug, Default)]
pub struct FileBuffer {
    pub path: Option<PathBuf>,
    pub content: text_editor::Content, // This causes a bug because [`Content`] can create exact replicas of a text
                                       // editor so scrolling and cursor locations are the same on each. Currently the only way I see
                                       // this changing is making a custom text editor widget.
}

impl FileBuffer {
    pub fn to_file(&self) -> File {
        (self.path.clone(), self.content.text().to_string())
    }
}

impl From<File> for FileBuffer {
    fn from((path, content): File) -> Self {
        Self {
            path,
            content: text_editor::Content::with_text(&content),
        }
    }
}

impl From<FileBuffer> for Buffer {
    fn from(file_buffer: FileBuffer) -> Self {
        Buffer::File(file_buffer)
    }
}
