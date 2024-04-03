use crate::widgets::Content;

use std::path::PathBuf;

#[derive(Debug)]
pub enum Buffer {
    File(FileBuffer),
}

#[derive(Debug, Default)]
pub struct FileBuffer {
    pub path: Option<PathBuf>,
    pub content: Content, // This causes a bug because [`Content`] can create exact replicas of a text
                          // editor so scrolling and cusor locations are the same on each. Currently the only way I see
                          // this changing is making a custom text editor widget.
}
