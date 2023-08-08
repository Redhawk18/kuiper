use std::path::PathBuf;

use iced::widget::text_input;
use iced::widget::Column;
use iced::{theme, Application, Command, Element, Subscription};

mod elements;
mod file_dialog;

pub use elements::menu_bar;
use iced_aw::style::tab_bar;

use self::elements::TabId;

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

struct Tabs {
    tab_bar: Vec<elements::TabId>,
    active: usize,
    data: Vec<Tab>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            active: 0,
            tab_bar: Vec::new(),
            data: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone)]
pub struct FileTab {
    text: String,
    path: PathBuf,
}

impl Default for FileTab {
    fn default() -> Self {
        Self {
            text: String::default(),
            path: PathBuf::default(),
        }
    }
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
        String::from("code editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextUpdate(text) => todo!(),

            Message::NewFile => return self.update(Message::TabNew(Tab::File(FileTab::default()))),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file();
                let Ok(text) = file_contents else { return Command::none() };

                self.tabs.data.push(Tab::File(FileTab {
                    text: text,
                    path: path,
                }));
            }

            Message::OpenFolder => file_dialog::pick_folder(),

            Message::Save => todo!(),

            Message::SaveAs => todo!(),

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => {
                self.tabs.data.push(tab);
            }

            Message::TabSelected(id) => {
                self.tabs.active = id;
            }

            Message::TabClosed(id) => todo!(),
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut c = Column::new().push(menu_bar());

        // if !self.tabs.is_empty() {
        //     c = c.push(elements::tab_header(&self.tabs, self.active_tab.unwrap()));
        //     c = c.push(
        //         text_input(
        //             "",
        //             self.tabs
        //                 .get(self.active_tab.unwrap())
        //                 .unwrap()
        //                 .text
        //                 .as_str(),
        //         )
        //         .on_input(Message::TextUpdate),
        //     );
        // }

        if !self.tabs.data.is_empty() {
            let tab_bar = elements::tab_header(self.tabs.active, &self.tabs.data);
            c = c.push(tab_bar);
        }

        c.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
