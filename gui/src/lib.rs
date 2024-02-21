mod file_dialog;
mod style;
mod tab;
mod widgets;
use tab::{FileTab, Tab};
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

pub fn start_gui() -> iced::Result {
    Blaze::run(Settings::default())
}

pub(crate) struct Blaze {
    data: SlotMap<DefaultKey, Tab>,
    panes: Panes,
}

pub(crate) struct Panes {
    active: Pane,
    data: State<PaneState>,
}

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
    OpenFolder,
    Save,
    SaveAs,
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
    //text input
    // TextInputUpdate(String),
}

impl Blaze {
    pub(crate) fn get_panestate(&self) -> &PaneState {
        self.panes.data.get(self.panes.active).unwrap()
    }

    pub(crate) fn get_mut_panestate(&mut self) -> &mut PaneState {
        self.panes.data.get_mut(self.panes.active).unwrap()
    }

    pub(crate) fn get_tab(&self) -> Option<&Tab> {
        let panestate = self.get_panestate();
        match panestate.data.get(panestate.active_tab_index) {
            Some(key) => Some(self.data.get(*key).unwrap()),
            None => None,
        }
    }

    pub(crate) fn get_mut_tab(&mut self) -> Option<&mut Tab> {
        let panestate = self.get_panestate();
        match panestate.get_active_key() {
            Some(key) => Some(self.data.get_mut(*key).unwrap()),
            None => None,
        }
    }

    pub(crate) fn insert_tab(&mut self, tab: Tab) {
        let key = self.data.insert(tab);
        self.get_mut_panestate().data.push(key);
    }
}

impl PaneState {
    pub fn get_active_key(&self) -> Option<&DefaultKey> {
        self.data.get(self.active_tab_index)
    }

    pub fn get_data<'a>(&'a self, map: &'a SlotMap<DefaultKey, Tab>) -> Vec<&Tab> {
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
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded)
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Blaze")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => {}

            Message::NewFile => self.insert_tab(Tab::File(FileTab::default())),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file_dialog();
                let Ok(text) = file_contents else {
                    return Command::none();
                };

                self.insert_tab(Tab::File(FileTab {
                    path: Some(path),
                    content: Content::with_text(&text),
                }))
            }

            Message::OpenFolder => file_dialog::pick_folder_dialog(),

            Message::Save => {
                let Some(tab) = self.get_tab() else {
                    log::warn!("TODO ERROR MESSAGE Message::Save");
                    return Command::none();
                };

                match tab {
                    Tab::File(file_tab) => {
                        if let Err(e) = file_dialog::save_file_dialog(file_tab) {
                            log::warn!("Saving error: {e}");
                            return Command::none();
                        }
                    }
                }
            }

            Message::SaveAs => {
                let Some(tab) = self.get_tab() else {
                    log::warn!("TODO ERROR MESSAGE Message::SaveAs");
                    return Command::none();
                };

                match tab {
                    Tab::File(file_tab) => {
                        if let Err(e) = file_dialog::save_file_as_dialog(file_tab) {
                            log::warn!("Saving error: {e}");
                            return Command::none();
                        }
                    }
                }
            }

            Message::Quit => return iced::window::close(iced::window::Id::MAIN),

            Message::TabSelected(id) => {
                self.get_mut_panestate().active_tab_index = id;
            }

            Message::TabClosed(id) => {
                let pane_state = self.get_mut_panestate();
                if id == pane_state.active_tab_index {
                    pane_state.active_tab_index = 0;
                }

                pane_state.data.remove(id);
                // current we arent removing the data from the program, just removing it from being visable
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
                if let Some((pane, _)) = self.panes.data.split(
                    axis,
                    pane,
                    PaneState::with_key(self.get_panestate().get_active_key().unwrap()),
                ) {
                    self.panes.active = pane;
                }
            }

            Message::PaneClosed(pane) => {
                if let Some((_, sibling)) = self.panes.data.close(pane) {
                    self.panes.active = sibling;
                }
            }
            Message::TextEditorUpdate(action) => {
                let tab = self.get_mut_tab().unwrap();
                match tab {
                    Tab::File(filetab) => filetab.content.perform(action),
                }
            } //             Message::TextInputUpdate(text) => {
              //                 let tab = self.get_mut_tab().unwrap();

              //                 match tab {
              //                     Tab::File(file_tab) => file_tab.text = text,
              //                 };
              //             }
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
