mod buffer;
mod file_dialog;
mod messages;
mod style;
mod widgets;

use buffer::{Buffer, FileBuffer};
pub use messages::{Button, LanguageServer, Message, PaneGrid, Tab, TextEditor, Widgets};
use widgets::{menu_bar, pane_grid};

use kuiper_lsp::{
    client::LSPClient,
    commands::{initialize, synchronize},
};

use iced::{
    application, font,
    widget::{
        column,
        pane_grid::{DragEvent, Pane, ResizeEvent, State},
        text_editor::Content,
    },
    Element, Task, Theme,
};
use slotmap::{DefaultKey, SlotMap};
use std::path::PathBuf;

pub fn start_gui() -> iced::Result {
    application(Kuiper::title, Kuiper::update, Kuiper::view)
        .theme(Kuiper::theme)
        .run_with(Kuiper::new)
}

pub struct Kuiper {
    data: SlotMap<DefaultKey, Buffer>,
    lsp_client: Option<LSPClient>,
    panes: Panes,
    workspace_folder: Option<PathBuf>,
}

/// The active panes/windows in the application.
pub struct Panes {
    active: Pane,
    /// wraps [iced]'s [State] in the [PaneState].
    data: State<PaneState>,
}

/// The global data store to hold all [Buffer]'s for the application.
#[derive(Default)]
pub struct PaneState {
    active_tab_index: usize,
    data: Vec<DefaultKey>,
}

impl Kuiper {
    fn new() -> (Self, Task<Message>) {
        (
            Kuiper {
                data: SlotMap::default(),
                lsp_client: None,
                panes: Panes::default(),
                workspace_folder: None,
            },
            Task::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Kuiper")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {}
            Message::LanguageServer(lsp) => match lsp {
                LanguageServer::Initalize(result) => {
                    let Ok(server) = result else {
                        log::error!("Error initializing language server");
                        return Task::none();
                    };

                    self.lsp_client = Some(LSPClient::new(server));
                }
                LanguageServer::Shutdown() => todo!(),
                LanguageServer::Syncronize(server) => log::warn!("{:#?}", server),
            },
            Message::Widgets(widget) => match widget {
                Widgets::Button(button) => match button {
                    Button::NewFile => self.insert_buffer(Buffer::File(FileBuffer::default())),
                    Button::OpenFile => {
                        return Task::perform(file_dialog::open_file(), |x| {
                            Message::Widgets(Widgets::Button(Button::OpenedFile(x)))
                        });
                    }
                    Button::OpenedFile(result) => {
                        let Ok(file) = result else {
                            return Task::none();
                        };

                        self.insert_buffer(Buffer::File(FileBuffer {
                            path: Some(file.0),
                            content: Content::with_text(&file.1),
                        }));

                        return Task::perform(initialize(), |x| {
                            Message::LanguageServer(LanguageServer::Initalize(x))
                        });
                    }
                    Button::OpenFolder => {
                        return Task::perform(file_dialog::open_folder(), |x| {
                            Message::Widgets(Widgets::Button(Button::OpenedFolder(x)))
                        })
                    }
                    Button::OpenedFolder(result) => {
                        let Ok(folder) = result else {
                            return Task::none();
                        };

                        self.workspace_folder = Some(folder);
                    }
                    Button::Save => {
                        let Some(buffer) = self.get_buffer() else {
                            log::warn!("No file open to save");
                            return Task::none();
                        };

                        match buffer {
                            Buffer::File(file_buffer) => {
                                return Task::perform(
                                    file_dialog::save_file(
                                        file_buffer.path.clone(),
                                        file_buffer.content.text(),
                                    ),
                                    |x| Message::Widgets(Widgets::Button(Button::Saved(x))),
                                );
                            }
                        }
                    }
                    Button::Saved(_) => {}
                    Button::SaveAs => {
                        let Some(buffer) = self.get_buffer() else {
                            log::warn!("No file open to save");
                            return Task::none();
                        };

                        match buffer {
                            Buffer::File(file_buffer) => {
                                return Task::perform(
                                    file_dialog::save_file_with_dialog(
                                        file_buffer.path.clone(),
                                        file_buffer.content.text(),
                                    ),
                                    |x| Message::Widgets(Widgets::Button(Button::SavedAs(x))),
                                )
                            }
                        }
                    }
                    Button::SavedAs(_) => {}
                    Button::Quit => todo!(),
                },
                Widgets::PaneGrid(pane_grid) => match pane_grid {
                    PaneGrid::PaneClicked(pane) => self.panes.active = pane,
                    PaneGrid::PaneClosed(pane) => {
                        if let Some((_, sibling)) = self.panes.data.close(pane) {
                            self.panes.active = sibling;
                        }
                    }
                    PaneGrid::PaneDragged(DragEvent::Dropped { pane, target }) => {
                        self.panes.data.drop(pane, target);
                    }
                    PaneGrid::PaneDragged(_) => {}
                    PaneGrid::PaneResized(ResizeEvent { split, ratio }) => {
                        self.panes.data.resize(split, ratio);
                    }
                    PaneGrid::PaneSplit(axis, pane) => {
                        let panestate = match self.get_panestate().get_active_key() {
                            Some(key) => PaneState::with_key(key),
                            None => PaneState::default(),
                        };

                        if let Some((pane, _)) = self.panes.data.split(axis, pane, panestate) {
                            self.panes.active = pane;
                        }
                    }
                },
                Widgets::Tab(tab) => match tab {
                    Tab::TabSelected(id) => self.get_mut_panestate().active_tab_index = id,
                    Tab::TabClosed(id) => {
                        let pane_state = self.get_mut_panestate();
                        if id == pane_state.active_tab_index {
                            pane_state.active_tab_index = 0;
                        }

                        // current we arent removing the data from the program, just removing it from being visable. This is a memory leak
                        pane_state.data.remove(id);

                        // set the new active tab index
                        if pane_state.data.is_empty() {
                            pane_state.active_tab_index = 0;
                        } else {
                            pane_state.active_tab_index = pane_state.data.len() - 1;
                        }
                    }
                },
                Widgets::TextEditor(text_editor) => match text_editor {
                    TextEditor::TextEditorUpdate(action) => {
                        let buffer = self.get_mut_buffer().unwrap();
                        match buffer {
                            Buffer::File(buffer) => {
                                buffer.content.perform(action);

                                let path: PathBuf;
                                match &buffer.path {
                                    Some(pathbuf) => path = pathbuf.to_path_buf(),
                                    None => {
                                        return Task::none();
                                    }
                                }

                                match &self.lsp_client {
                                    Some(client) => {
                                        return Task::perform(
                                            synchronize(path, client.clone().socket),
                                            |x| {
                                                Message::LanguageServer(LanguageServer::Syncronize(
                                                    x,
                                                ))
                                            },
                                        );
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                },
            },
        }

        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column!(menu_bar(), pane_grid(&self.panes, &self.data))
            .padding(8)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }

    pub(crate) fn get_buffer(&self) -> Option<&Buffer> {
        let panestate = self.get_panestate();
        match panestate.data.get(panestate.active_tab_index) {
            Some(key) => Some(self.data.get(*key).unwrap()),
            None => None,
        }
    }

    pub(crate) fn get_mut_buffer(&mut self) -> Option<&mut Buffer> {
        let panestate = self.get_panestate();
        match panestate.get_active_key() {
            Some(key) => Some(self.data.get_mut(*key).unwrap()),
            None => None,
        }
    }

    pub(crate) fn get_panestate(&self) -> &PaneState {
        self.panes.data.get(self.panes.active).unwrap()
    }

    pub(crate) fn get_mut_panestate(&mut self) -> &mut PaneState {
        self.panes.data.get_mut(self.panes.active).unwrap()
    }

    pub(crate) fn insert_buffer(&mut self, buffer: Buffer) {
        let key = self.data.insert(buffer);
        self.get_mut_panestate().data.push(key);
    }
}

impl PaneState {
    pub fn get_active_key(&self) -> Option<&DefaultKey> {
        self.data.get(self.active_tab_index)
    }

    pub fn get_data<'a>(&self, map: &'a SlotMap<DefaultKey, Buffer>) -> Vec<&'a Buffer> {
        self.data.iter().map(|key| map.get(*key).unwrap()).collect()
    }

    pub fn with_key(key: &DefaultKey) -> Self {
        PaneState {
            data: vec![*key],
            ..Default::default()
        }
    }
}

impl Default for Panes {
    fn default() -> Self {
        let (state, pane) = State::new(PaneState::default());
        Self {
            active: pane,
            data: state,
        }
    }
}
