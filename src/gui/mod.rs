use std::path::PathBuf;

use iced::widget::text_input;
use iced::widget::Column;
use iced::{theme, Application, Command, Element, Subscription};

mod elements;
mod file_dialog;

pub use elements::menu_bar;

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
    TabNew(Tab),
    TabSelected(usize),
    TabClosed(usize),
}

pub struct Blaze {
    tabs: Tabs,
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

impl Application for Blaze {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Blaze {
                tabs: Tabs::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Blaze")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextUpdate(text) => {
                let tab = self.tabs.data.get_mut(self.tabs.active).unwrap();

                match tab {
                    Tab::File(file_tab) => file_tab.text = text,
                }
            }

            Message::NewFile => return self.update(Message::TabNew(Tab::File(FileTab::default()))),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file();
                let Ok(text) = file_contents else { return Command::none() };

                self.tabs.data.push(Tab::File(FileTab { text, path }));
            }

            Message::OpenFolder => file_dialog::pick_folder(),

            Message::Save => {
                let tab = self.tabs.data.get(self.tabs.active).unwrap();
                match tab {
                    Tab::File(file_tab) => file_dialog::save_file(file_tab).unwrap(),
                }
            }

            Message::SaveAs => {
                let tab = self.tabs.data.get(self.tabs.active).unwrap();
                match tab {
                    Tab::File(file_tab) => file_dialog::save_file_as(file_tab).unwrap(),
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
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut c = Column::new().push(menu_bar());

        let tab_bar = elements::tab_header(self.tabs.active, &self.tabs.data);
        c = c.push(tab_bar);

        let tab = self.tabs.data.get(self.tabs.active);

        if let Some(active_tab) = tab {
            match active_tab {
                Tab::File(file_tab) => {
                    c = c.push(
                        text_input("placeholder", &file_tab.text).on_input(Message::TextUpdate),
                    );
                }
            }
        }

        c.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
