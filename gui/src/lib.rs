use iced::{application, exit, font, widget::column, Element, Task, Theme};
use slotmap::{DefaultKey, SlotMap};
use std::path::PathBuf;

mod buffer;
mod file_dialog;
pub mod icon;
mod lsp;
mod panes;
mod style;
mod toolbar;

use buffer::Buffer;

pub fn start_gui() -> iced::Result {
    application(Kuiper::title, Kuiper::update, Kuiper::view)
        .theme(Kuiper::theme)
        .run_with(Kuiper::new)
}

pub type Key = DefaultKey;
pub type Map = SlotMap<Key, Buffer>;

#[derive(Default)]
pub struct Kuiper {
    data: Map,
    lsp_client: Option<kuiper_lsp::client::LSPClient>,
    panes: panes::Panes,
    workspace_folder: Option<PathBuf>,
}

#[derive(Debug)]
pub enum Message {
    FontLoaded(Result<(), font::Error>),

    LanguageServer(lsp::Message),
    Toolbar(toolbar::Message),
    Panes(panes::Message),
}

impl Kuiper {
    fn new() -> (Self, Task<Message>) {
        (
            Kuiper::default(),
            Task::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Kuiper")
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => Task::none(),
            Message::Toolbar(message) => {
                if let Some(action) = toolbar::update(message) {
                    match action {
                        toolbar::Action::InsertFileBuffer(buffer) => {
                            let key = self.data.insert(buffer.into());
                            self.panes
                                .active_pane_mut()
                                .map(|pane| pane.insert_buffer(key));
                        }
                        toolbar::Action::SetWorkspacePath(path) => {
                            self.workspace_folder = Some(path);
                        }
                        toolbar::Action::SaveBuffer => {
                            if let Some(buffer) = self
                                .panes
                                .active_buffer_key()
                                .and_then(|key| self.data.get_mut(*key))
                            {
                                match buffer {
                                    Buffer::File(file_buffer) => {
                                        return Task::perform(
                                            file_dialog::save_file(file_buffer.to_file()),
                                            toolbar::Message::FileSaved,
                                        )
                                        .map(Message::Toolbar);
                                    }
                                }
                            }
                        }
                        toolbar::Action::SaveBufferAs => {
                            if let Some(buffer) = self
                                .panes
                                .active_buffer_key()
                                .and_then(|key| self.data.get(*key))
                            {
                                match buffer {
                                    Buffer::File(file_buffer) => {
                                        return Task::perform(
                                            file_dialog::save_file_as(file_buffer.to_file()),
                                            toolbar::Message::FileSavedAs,
                                        )
                                        .map(Message::Toolbar);
                                    }
                                }
                            }
                        }
                        toolbar::Action::Quit => {
                            let tasks = Vec::new();

                            if let Some(_) = &self.lsp_client {
                                // let task =
                                //     Task::perform(shutdown(client.clone().socket.clone()), |_| {
                                //         Message::LanguageServer(LanguageServer::Shutdown())
                                //     });
                                tracing::info!("Shutting down lsp");
                                // tasks.push(task);
                            }

                            return Task::batch(tasks).chain(exit());
                        }
                        toolbar::Action::Run(task) => return task.map(Message::Toolbar),
                    }
                }

                Task::none()
            }
            Message::Panes(message) => self
                .panes
                .update(message, &mut self.data, &mut self.lsp_client)
                .map(Message::Panes),
            Message::LanguageServer(message) => {
                lsp::update(&mut self.lsp_client, message).map(Message::LanguageServer)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column!(
            toolbar::view().map(Message::Toolbar),
            self.panes.view(&self.data).map(Message::Panes)
        )
        .spacing(2)
        .padding(8)
        .into()
    }
}
