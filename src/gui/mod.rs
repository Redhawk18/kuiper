mod file_dialog;
mod theme;
mod widgets;
use theme::{Element, Theme};
use widgets::{menu_bar::menu_bar, pane_grid::pane_grid};

use iced::{
    font,
    widget::{
        column,
        pane_grid::{DragEvent, Pane, ResizeEvent, State},
    },
    Application, Command, Subscription,
};
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

    PaneClicked(Pane),
    PaneDragged(DragEvent),
    PaneResized(ResizeEvent),

    //text input
    TextUpdate(String),
}

pub struct Blaze {
    panes: Panes,
    tabs: Tabs,
    theme: theme::Theme,
}

pub struct Panes {
    active: Option<Pane>,
    data: State<PaneState>,
}

impl Default for Panes {
    fn default() -> Self {
        let (state, _) = State::new(PaneState::Tab);
        Self {
            active: None,
            data: state,
        }
    }
}

#[derive(Default)]
pub struct Tabs {
    active: usize,
    data: Vec<Tab>,
}

#[derive(Debug, Clone)]
pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone, Default)]
pub struct FileTab {
    text: String,
    path: PathBuf,
}

pub enum PaneState {
    Tab,
}

impl Application for Blaze {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Blaze {
                tabs: Tabs::default(),
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
                let (file_contents, path) = file_dialog::pick_file_dialog();
                let Ok(text) = file_contents else { return Command::none() };

                self.tabs.data.push(Tab::File(FileTab { text, path }));
            }

            Message::OpenFolder => file_dialog::pick_folder_dialog(),

            Message::Save => {
                let Some(tab) = self.tabs.data.get(self.tabs.active) else {
                    log::warn!("cannot save, data vector is empty"); 
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
                let Some(tab) = self.tabs.data.get(self.tabs.active) else {
                    log::warn!("cannot save as, data vector is empty"); 
                    return Command::none()
                };

                match tab {
                    Tab::File(file_tab) => {
                        if let Err(e) = file_dialog::save_file_as_dialog(file_tab) {
                            log::warn!("Saving as error: {e}");
                            return Command::none();
                        }
                    }
                }
            }

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => {
                self.tabs.data.push(tab);
            }

            Message::TabSelected(id) => {
                self.tabs.active = id;
            }

            Message::TabClosed(id) => {
                if id == self.tabs.active {
                    self.tabs.active = 0;
                }

                self.tabs.data.remove(id);
            }

            Message::TextUpdate(text) => {
                let tab = self.tabs.data.get_mut(self.tabs.active).unwrap();

                match tab {
                    Tab::File(file_tab) => file_tab.text = text,
                }
            }
            Message::PaneDragged(DragEvent::Dropped { pane, target }) => {
                self.panes.data.drop(&pane, target);
            }
            Message::PaneDragged(_) => {}
            Message::PaneResized(_) => todo!(),

            Message::PaneClicked(pane) => self.panes.active = Some(pane),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        column!(menu_bar(), pane_grid(&self.panes, &self.tabs))
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
