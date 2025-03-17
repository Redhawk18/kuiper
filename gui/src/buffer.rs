use iced::widget::text_editor;
use std::path::PathBuf;

pub type File = (String, Option<PathBuf>);
pub type Folder = PathBuf;

#[derive(Debug)]
pub enum Buffer {
    File(FileBuffer),
}

// TODO remove public
#[derive(Debug, Default)]
pub struct FileBuffer {
    pub content: text_editor::Content, // This causes a bug because [`Content`] can create exact replicas of a text
    // editor so scrolling and cursor locations are the same on each. Currently the only way I see
    // this changing is making a custom text editor widget.
    _dirty: bool,
    pub path: Option<PathBuf>,
}

impl FileBuffer {
    pub fn new(content: text_editor::Content, path: Option<PathBuf>) -> Self {
        Self {
            content,
            path,
            ..Default::default()
        }
    }

    pub fn to_file(&self) -> File {
        (self.content.text(), self.path.clone())
    }
}

impl From<File> for FileBuffer {
    fn from((content, path): File) -> Self {
        Self {
            content: text_editor::Content::with_text(&content),
            path,
            ..Default::default()
        }
    }
}

impl From<FileBuffer> for Buffer {
    fn from(file_buffer: FileBuffer) -> Self {
        Buffer::File(file_buffer)
    }
}
