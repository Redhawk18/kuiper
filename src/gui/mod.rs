mod file_dialog;
mod theme;
mod widgets;
use theme::{Element, Theme};
use widgets::{menu_bar::menu_bar, tab_bar::tab_bar};

use iced::{
    font,
    widget::{column, pane_grid, text},
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

    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),

    //text input
    TextUpdate(String),
}

pub struct Blaze {
    tabs: Tabs,
    theme: theme::Theme,
    pane: pane_grid::State<PaneState>,
}

#[derive(Default)]
struct Tabs {
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

enum PaneState {
    SomePane,
    AnotherKindOfPane,
}

impl Application for Blaze {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let (state, _) = pane_grid::State::new(PaneState::SomePane);
        (
            Blaze {
                tabs: Tabs::default(),
                theme: Theme::default(),
                pane: state,
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
            Message::PaneDragged(_) => todo!(),
            Message::PaneResized(_) => todo!(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        // column!(menu_bar(), tab_bar(self.tabs.active, &self.tabs.data)).into()

        let pane_grid = pane_grid::PaneGrid::new(&self.pane, |pane, state, is_maximized| {
            pane_grid::Content::new(match state {
                PaneState::SomePane => text("This is some pane"),
                PaneState::AnotherKindOfPane => text("This is another kind of pane"),
            })
        })
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized);

        column!(pane_grid).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
