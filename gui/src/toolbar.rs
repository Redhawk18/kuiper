use iced::{
    widget::{button, row},
    Element, Task,
};

use crate::buffer::{File, FileBuffer, Folder};
use crate::file_dialog::{self, Error};

#[derive(Debug, Clone)]
pub enum Message {
    NewFile,
    OpenFile,
    OpenFolder,
    SaveFile,
    SaveFileAs,

    FileOpened(Result<File, Error>),
    FolderOpened(Result<Folder, Error>),
    #[allow(dead_code)]
    FileSaved(Result<(), Error>),
    #[allow(dead_code)]
    FileSavedAs(Result<(), Error>),

    Quit,
}

pub enum Action {
    InsertFileBuffer(FileBuffer),
    SetWorkspacePath(Folder),
    SaveBuffer,
    SaveBufferAs,
    Quit,
    Run(Task<Message>),
}

pub(crate) fn view() -> Element<'static, Message> {
    let new_file = button("New File").on_press(Message::NewFile).width(150);

    let open_file = button("Open File").on_press(Message::OpenFile).width(150);

    let open_folder = button("Open Folder")
        .on_press(Message::OpenFolder)
        .width(150);

    let save = button("Save").on_press(Message::SaveFile).width(150);

    let save_as = button("Save As").on_press(Message::SaveFileAs).width(150);

    let quit = button("Quit").on_press(Message::Quit).width(150);

    row![new_file, open_file, open_folder, save, save_as, quit]
        .spacing(2)
        .into()
}

pub fn update(message: Message) -> Option<Action> {
    match message {
        Message::NewFile => Some(Action::InsertFileBuffer(FileBuffer::default())),
        Message::OpenFile => Some(Action::Run(Task::perform(
            file_dialog::open_file(),
            Message::FileOpened,
        ))),
        Message::OpenFolder => Some(Action::Run(Task::perform(
            file_dialog::open_folder(),
            Message::FolderOpened,
        ))),
        Message::SaveFile => Some(Action::SaveBuffer),
        Message::SaveFileAs => Some(Action::SaveBufferAs),

        Message::FileOpened(result) => result
            .ok()
            .map(|file| Action::InsertFileBuffer(file.into())),
        Message::FolderOpened(result) => result.ok().map(Action::SetWorkspacePath),
        Message::FileSaved(_) => {
            tracing::info!("File saved");
            None
        }
        Message::FileSavedAs(_) => {
            tracing::info!("File saved as");
            None
        }

        Message::Quit => Some(Action::Quit),
    }
}
