use std::path::PathBuf;

use iced::widget::text;
use iced::widget::text_input;
use iced::widget::Column;
use iced::{Application, Command, Subscription};
mod elements;
mod file_dialog;
mod theme;

pub use elements::menu_bar;
use theme::{Element, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    TextUpdate(String),

    //menu bar
    NewFile,
    OpenFile,
    OpenFolder,
    Save,
    SaveAs,
    Quit,

    //tabs
    TabNew(FileTab),
    TabSelected(usize),
    TabClosed(usize),
}

#[derive(Debug, Clone)]
pub struct FileTab {
    text: String,
    path: PathBuf,
}

pub struct State {
    active_tab: Option<usize>,
    tabs: Vec<FileTab>,
    theme: Theme,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                active_tab: None,
                tabs: Vec::new(),
                theme: Theme::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("code editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextUpdate(text) => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get_mut(index).unwrap();
                    tab.text = text;
                }
                None => {
                    return self.update(Message::TabNew(FileTab {
                        text: String::default(),
                        path: PathBuf::default(),
                    }))
                }
            },

            Message::NewFile => {
                return self.update(Message::TabNew(FileTab {
                    text: String::default(),
                    path: PathBuf::default(),
                }))
            }

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file();

                match file_contents {
                    Ok(text) => match self.active_tab {
                        Some(index) => {
                            let tab = self.tabs.get_mut(index).unwrap();
                            tab.path = path;
                            return self.update(Message::TextUpdate(text));
                        }
                        None => return self.update(Message::TabNew(FileTab { text, path })),
                    },
                    Err(_e) => {
                        return Command::none();
                    }
                }
            }

            Message::OpenFolder => file_dialog::pick_folder(),

            Message::Save => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get(index).unwrap();
                    file_dialog::save_file(tab.text.as_str(), tab.path.as_path()).unwrap();
                }
                None => return Command::none(),
            },

            Message::SaveAs => match self.active_tab {
                Some(index) => {
                    let tab = self.tabs.get(index).unwrap();
                    file_dialog::save_as(tab.text.as_str(), tab.path.as_path()).unwrap();
                }
                None => return Command::none(),
            },

            Message::Quit => std::process::exit(0),

            Message::TabNew(tab) => {
                log::info!("New tab");
                self.tabs.push(tab);
                self.active_tab = Some(self.tabs.len() - 1);
            }

            Message::TabSelected(index) => {
                log::info!("Selected tab {}", index);
                self.active_tab = Some(index);
            }

            Message::TabClosed(index) => {
                log::info!("Closed tab {}", index);
                self.tabs.remove(index);
                self.active_tab = if self.tabs.is_empty() {
                    Some(0)
                } else {
                    Some(usize::max(
                        0,
                        usize::min(self.active_tab.unwrap(), self.tabs.len() - 1),
                    ))
                };
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut c = Column::new().push(menu_bar());

        if !self.tabs.is_empty() {
            c = c.push(elements::tab_header(&self.tabs, self.active_tab.unwrap()));
            c = c.push(
                text_input(
                    "",
                    self.tabs
                        .get(self.active_tab.unwrap())
                        .unwrap()
                        .text
                        .as_str(),
                )
                .on_input(Message::TextUpdate),
            );
        }
        c = c.push(text("TEXTBOX")); //temp

        c.into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
