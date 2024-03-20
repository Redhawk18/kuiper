mod buffer;
mod file_dialog;
mod style;
mod widgets;

use buffer::{Buffer, FileBuffer};
use widgets::{menu_bar, pane_grid};

use iced::{
    executor, font,
    widget::{
        column,
        pane_grid::{Axis, DragEvent, Pane, ResizeEvent, State},
        text_editor::{Action, Content},
    },
    Application, Command, Element, Settings, Subscription, Theme,
};
use slotmap::{DefaultKey, SlotMap};
use std::path::PathBuf;

pub fn start_gui() -> iced::Result {
    Blaze::run(Settings::default())
}

pub(crate) struct Blaze {
    data: SlotMap<DefaultKey, Buffer>,
    panes: Panes,
}

/// The active panes/windows in the application.
pub(crate) struct Panes {
    active: Pane,
    /// wraps [iced]'s [State] in the [PaneState].
    data: State<PaneState>,
}

/// The global data store to hold all [Buffer]'s for the application.
#[derive(Default)]
pub(crate) struct PaneState {
    active_tab_index: usize,
    data: Vec<DefaultKey>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    //application
    FontLoaded(Result<(), font::Error>),

    //menu bar
    NewFile,
    OpenFile,
    OpenedFile(Result<(PathBuf, String), file_dialog::Error>),
    Save,
    Saved(Result<(), file_dialog::Error>),
    SaveAs,
    SavedAs(Result<(), file_dialog::Error>),
    Quit,

    //tabs
    TabSelected(usize),
    TabClosed(usize),

    //panegrid
    PaneClicked(Pane),
    PaneClosed(Pane),
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),
    PaneSplit(Axis, Pane),

    //text editor
    TextEditorUpdate(Action),
}

impl Blaze {
    pub(crate) fn get_panestate(&self) -> &PaneState {
        self.panes.data.get(self.panes.active).unwrap()
    }

    pub(crate) fn get_mut_panestate(&mut self) -> &mut PaneState {
        self.panes.data.get_mut(self.panes.active).unwrap()
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

    pub(crate) fn insert_buffer(&mut self, buffer: Buffer) {
        let key = self.data.insert(buffer);
        self.get_mut_panestate().data.push(key);
    }
}

impl PaneState {
    pub fn get_active_key(&self) -> Option<&DefaultKey> {
        self.data.get(self.active_tab_index)
    }

    pub fn get_data<'a>(&'a self, map: &'a SlotMap<DefaultKey, Buffer>) -> Vec<&Buffer> {
        self.data.iter().map(|key| map.get(*key).unwrap()).collect()
    }

    pub fn with_key(key: &DefaultKey) -> Self {
        PaneState {
            data: vec![*key],
            ..Default::default()
        }
    }
}

impl Application for Blaze {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Blaze {
                data: SlotMap::default(),
                panes: Panes::default(),
            },
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                font::load(iced_aw::NERD_FONT_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Blaze")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => {}

            Message::NewFile => self.insert_buffer(Buffer::File(FileBuffer::default())),

            Message::OpenFile => {
                return Command::perform(file_dialog::open_file(), Message::OpenedFile)
            }

            Message::OpenedFile(result) => {
                let Ok(file) = result else {
                    return Command::none();
                };

                self.insert_buffer(Buffer::File(FileBuffer {
                    path: Some(file.0),
                    content: Content::with_text(&file.1),
                }))
            }

            Message::Save => {
                let Some(buffer) = self.get_buffer() else {
                    log::warn!("TODO ERROR MESSAGE Message::Save");
                    return Command::none();
                };

                match buffer {
                    Buffer::File(file_buffer) => {
                        return Command::perform(
                            file_dialog::save_file(
                                file_buffer.path.clone(),
                                file_buffer.content.text(),
                            ),
                            Message::Saved,
                        );
                    }
                }
            }

            Message::Saved(_) => {}

            Message::SaveAs => {
                let Some(buffer) = self.get_buffer() else {
                    log::warn!("TODO ERROR MESSAGE Message::Save");
                    return Command::none();
                };

                match buffer {
                    Buffer::File(file_buffer) => {
                        return Command::perform(
                            file_dialog::save_file_with_dialog(
                                file_buffer.path.clone(),
                                file_buffer.content.text(),
                            ),
                            Message::SavedAs,
                        )
                    }
                }
            }

            Message::SavedAs(_) => {}

            Message::Quit => return iced::window::close(iced::window::Id::MAIN),

            Message::TabSelected(id) => {
                self.get_mut_panestate().active_tab_index = id;
            }

            Message::TabClosed(id) => {
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

            Message::PaneDragged(DragEvent::Dropped { pane, target }) => {
                self.panes.data.drop(pane, target);
            }

            Message::PaneDragged(_) => {}

            Message::PaneResized(ResizeEvent { split, ratio }) => {
                self.panes.data.resize(split, ratio);
            }

            Message::PaneClicked(pane) => self.panes.active = pane,

            Message::PaneSplit(axis, pane) => {
                let panestate = match self.get_panestate().get_active_key() {
                    Some(key) => PaneState::with_key(key),
                    None => PaneState::default(),
                };

                if let Some((pane, _)) = self.panes.data.split(axis, pane, panestate) {
                    self.panes.active = pane;
                }
            }

            Message::PaneClosed(pane) => {
                if let Some((_, sibling)) = self.panes.data.close(pane) {
                    self.panes.active = sibling;
                }
            }
            Message::TextEditorUpdate(action) => {
                let buffer = self.get_mut_buffer().unwrap();
                match buffer {
                    Buffer::File(file_buffer) => file_buffer.content.perform(action),
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column!(menu_bar(), pane_grid(&self.panes, &self.data))
            .padding(8)
            .into()
    }

    fn theme(&self) -> Theme {
        iced::Theme::GruvboxDark
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
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
