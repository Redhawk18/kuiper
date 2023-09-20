mod file_dialog;
mod theme;
mod widgets;
use theme::{Element, Theme};
use widgets::{menu_bar::menu_bar, pane_grid::pane_grid};

use iced::{
    font,
    widget::{
        column,
        pane_grid::{Axis, DragEvent, Pane, ResizeEvent, State},
    },
    Application, Command, Subscription, Settings
};
use slotmap::{DefaultKey, SlotMap};
use std::path::PathBuf;

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
    TabNew(Tab),
    TabSelected(usize),
    TabClosed(usize),

    //panegrid
    PaneClicked(Pane),
    PaneClosed(Pane),
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),
    PaneSplit(Axis, Pane),

    //text input
    TextUpdate(String),
}

pub(crate) struct Blaze {
    /// Holds all the data of the application
    data: SlotMap<DefaultKey, Tab>,
    panes: Panes,
    theme: theme::Theme,
}

pub(crate) struct Panes {
    active: Pane,
    data: State<PaneState>,
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

#[derive(Debug, Clone)]
pub(crate) enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FileTab {
    path: PathBuf,
    text: String,
}

#[derive(Default)]
pub(crate) struct PaneState {
    active_tab: usize,
    data: Vec<DefaultKey>,
}

pub fn start_gui() -> iced::Result {
    Blaze::run(Settings::default())
}

impl Blaze {
    pub(crate) fn get_panestate(&self) -> &PaneState {
        self.panes.data.get(&self.panes.active).unwrap()
    }

    pub(crate) fn get_mut_panestate(&mut self) -> &mut PaneState {
        self.panes.data.get_mut(&self.panes.active).unwrap()
    }

    pub(crate) fn get_tab(&self) -> Option<&Tab> {
        let panestate = self.get_panestate();
        match panestate.data.get(panestate.active_tab) {
            Some(key) => Some(self.data.get(*key).unwrap()),
            None => None,
        }
    }

    pub(crate) fn get_mut_tab(&mut self) -> Option<&mut Tab> {
        let panestate = self.get_panestate();
        match panestate.data.get(panestate.active_tab) {
            Some(key) => Some(self.data.get_mut(*key).unwrap()),
            None => None,
        }
    }
}

impl PaneState {
    pub fn get_data<'a>(&'a self, map: &'a SlotMap<DefaultKey, Tab>) -> Vec<&Tab> {
        self.data.iter().map(|key| map.get(*key).unwrap()).collect()
    }
}

impl Application for Blaze {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Blaze {
                data: SlotMap::default(),
                theme: Theme::default(),
                panes: Panes::default(),
            },
            Command::batch(vec![
                font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded)
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Blaze")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FontLoaded(_) => {}

            Message::NewFile => return self.update(Message::TabNew(Tab::File(FileTab::default()))),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file_dialog();
                let Ok(text) = file_contents else { return Command::none() };

                return self.update(Message::TabNew(Tab::File(FileTab { path, text })));
            }

            Message::OpenFolder => file_dialog::pick_folder_dialog(),

            Message::Save => {
                let Some(tab) = self.get_tab() else {
                    log::warn!("TODO ERROR MESSAGE Message::Save");
                    return Command::none()
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
                    return Command::none()
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

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => {
                let key = self.data.insert(tab);
                self.get_mut_panestate().data.push(key);
            }

            Message::TabSelected(id) => {
                self.get_mut_panestate().active_tab = id;
            }

            Message::TabClosed(id) => {
                let pane_state = self.get_mut_panestate();
                if id == pane_state.active_tab {
                    pane_state.active_tab = 0;
                }

                pane_state.data.remove(id);
                // current we arent removing the data from the program, just removing it from being visable
            }

            Message::TextUpdate(text) => {
                let tab = self.get_mut_tab().unwrap();

                match tab {
                    Tab::File(file_tab) => file_tab.text = text,
                };
            }
            Message::PaneDragged(DragEvent::Dropped { pane, target }) => {
                self.panes.data.drop(&pane, target);
            }
            Message::PaneDragged(_) => {}
            Message::PaneResized(ResizeEvent { split, ratio }) => {
                self.panes.data.resize(&split, ratio);
            }
            Message::PaneClicked(pane) => self.panes.active = pane,
            Message::PaneSplit(axis, pane) => {
                if let Some((pane, _)) = self.panes.data.split(axis, &pane, PaneState::default()) {
                    self.panes.active = pane;
                }
            }
            Message::PaneClosed(pane) => {
                if let Some((_, sibling)) = self.panes.data.close(&pane) {
                    self.panes.active = sibling;
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
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
