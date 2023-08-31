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
    Application, Command, Subscription,
};
use generational_array::{GenerationalArray, GenerationalIndex};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
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
    PaneSplitFocused(Axis),

    //text input
    TextUpdate(String),
}

pub struct Blaze {
    tab_data: GenerationalArray<Tab>,
    panes: Panes,
    theme: theme::Theme,
}

pub struct Panes {
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
pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone, Default)]
pub struct FileTab {
    path: PathBuf,
    text: String,
}

pub struct PaneState {
    active_tab: usize,
    data: Vec<GenerationalIndex>,
    tab: Tab,
}

impl Default for PaneState {
    fn default() -> Self {
        Self {
            active_tab: 0,
            data: Vec::default(),
            tab: Tab::File(FileTab::default()),
        }
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
                tab_data: GenerationalArray::new(),
                theme: Theme::default(),
                panes: Panes::default(),
            },
            font::load(iced_aw::graphics::icons::ICON_FONT_BYTES).map(Message::FontLoaded),
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
                // let (file_contents, path) = file_dialog::pick_file_dialog();
                // let Ok(text) = file_contents else { return Command::none() };

                // self.data.data.push(Tab::File(FileTab { text, path }));
            }

            Message::OpenFolder => file_dialog::pick_folder_dialog(),

            Message::Save => {
                // let Some(tab) = self.data.data.get(
                //     self.panes.data.get(&self.panes.active.unwrap()).unwrap().active) else {
                //     log::warn!("cannot save, data vector is empty"); 
                //     return Command::none()
                // };

                // match tab {
                //     Tab::File(file_tab) => {
                //         if let Err(e) = file_dialog::save_file_dialog(file_tab) {
                //             log::warn!("Saving error: {e}");
                //             return Command::none();
                //         }
                //     }
                // }
            }

            Message::SaveAs => {
                // let Some(tab) = self.data.data.get(self.panes.data.get(&self.panes.active.unwrap()).unwrap().active) else {
                //     log::warn!("cannot save as, data vector is empty"); 
                //     return Command::none()
                // };

                // match tab {
                //     Tab::File(file_tab) => {
                //         if let Err(e) = file_dialog::save_file_as_dialog(file_tab) {
                //             log::warn!("Saving as error: {e}");
                //             return Command::none();
                //         }
                //     }
                // }
            }

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => {
                let index = self.tab_data.insert(tab);
                self.panes.data.get_mut(&self.panes.active).unwrap().data.push(index);
            }

            Message::TabSelected(id) => {
                self.panes.data.get_mut(&self.panes.active).unwrap().active_tab = id;
                
            }

            Message::TabClosed(id) => {
                // //deleting data is hard, i'll work on it later
                if id == self.panes.data.get_mut(&self.panes.active).unwrap().active_tab {
                    self.panes.data.get_mut(&self.panes.active).unwrap().active_tab = 0;
                }
                
                self.panes.data.get_mut(&self.panes.active).unwrap().data.remove(id); // current we arent removing the data from the program, just removing it from being visable
                // self.tab_data.remove(id);
            }

            Message::TextUpdate(text) => {
                let index = self.panes.data.get(&self.panes.active).unwrap().active_tab;
                let generational_index = self.panes.data.get(&self.panes.active).unwrap().data.get(index).unwrap();
                let resul = self.tab_data.get_mut(generational_index);

                match resul {
                    generational_array::GenerationalArrayResultMut::None => todo!(),
                    generational_array::GenerationalArrayResultMut::OutDated => todo!(),
                    generational_array::GenerationalArrayResultMut::OutOfBounds => todo!(),
                    generational_array::GenerationalArrayResultMut::Some(tab) => match tab {
                        Tab::File(file_tab) => file_tab.text = text,
                    },

                }
            }

            Message::PaneClicked(pane) => self.panes.active = pane,

            Message::PaneClosed(pane) => {
                if let Some((_, sibling)) = self.panes.data.close(&pane) {
                    self.panes.active = sibling;
                }
            }

            Message::PaneDragged(DragEvent::Dropped { pane, target }) => {
                self.panes.data.drop(&pane, target);
            }

            Message::PaneDragged(_) => {}

            Message::PaneResized(ResizeEvent { split, ratio }) => {
                self.panes.data.resize(&split, ratio);
            }

            Message::PaneSplit(axis, pane) => {
                let result = self.panes.data.split(axis, &pane, PaneState::default());

                if let Some((pane, _)) = result {
                    self.panes.active = pane;
                }
            }

            Message::PaneSplitFocused(_) => todo!(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let index = self.panes.data.get(&self.panes.active).unwrap().active_tab;
        let pane_vec = &self.panes.data.get(&self.panes.active).unwrap().data;
        column!(menu_bar(), pane_grid(&self.panes, &pane_vec, &self.tab_data))
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
