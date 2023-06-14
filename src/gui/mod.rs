use std::path::PathBuf;

use iced::widget::{text_input};
use iced::{widget::Column};

//use iced::widget::column;
//use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Subscription};
use iced_aw::menu::MenuBar;

use log;

mod file_dialog;
mod menu_bar;
mod tabs;

pub use menu_bar::file;

#[derive(Debug, Clone)]
pub enum Message {
    TextUpdate(String),

    //menu bar
    NewFile,
    OpenFile,
    OpenFolder,
    Save,
    SaveAs,

    //tabs
    NewTab,
    TabSelected(usize),
    TabClosed(usize),
    TabLabelInputChanged(String),
    TabContentInputChanged(String),
}

pub struct FileTab {
    text: String,
    path: PathBuf,
}

pub struct State {
    active_tab: usize, //TODO make a option for a no tab case
    new_tab_label: String,
    new_tab_content: String,
    tabs: Vec<FileTab>,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                active_tab: 0,
                new_tab_label: String::new(),
                new_tab_content: String::new(),
                tabs: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("code editor")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextUpdate(text) => {
                self.tabs.get_mut(self.active_tab).unwrap().text = text;
            }

            Message::NewFile => return self.update(Message::NewTab),

            Message::OpenFile => {
                // let (file_contents, path) = file_dialog::pick_file();

                // self.path = path;
                // match file_contents {
                //     Ok(v) => {
                //         return self.update(Message::TextUpdate(v));
                //     }
                //     Err(_e) => {
                //         return Command::none();
                //     }
                // }
            }

            Message::OpenFolder => {
                // file_dialog::pick_folder();
            }

            Message::Save => {
                // file_dialog::save_file(self.text.as_str(), &self.path).unwrap();
            }

            Message::SaveAs => {
                // file_dialog::save_as(self.text.as_str(), &self.path).unwrap();
            }
            Message::TabSelected(index) => {
                log::info!("{}", index);
                self.active_tab = index;
            }
            Message::TabClosed(index) => {
                self.tabs.remove(index);
                println!("active tab before: {}", self.active_tab);
                self.active_tab = if self.tabs.is_empty() {
                    0
                } else {
                    usize::max(0, usize::min(self.active_tab, self.tabs.len() - 1))
                };
                println!("active tab after: {}", self.active_tab);
            }
            Message::TabLabelInputChanged(value) => self.new_tab_label = value,
            Message::TabContentInputChanged(value) => self.new_tab_content = value,
            Message::NewTab => {
                println!("New");

                println!("Create");
                self.tabs
                    .push(FileTab{
                        text: self.tabs.len().to_string(),
                        path: PathBuf::default(),
                    });
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = MenuBar::new(vec![file(self)]);

        Column::new()
            .push(menu_bar)
            .push(tabs::tab_header(&self.tabs))
            //problem is that at the start we dont have any active tabs
            .push(text_input(self.tabs.len().to_string().as_str(), self.tabs.get(self.active_tab).unwrap_or(
                &FileTab { text: String::default(), path: PathBuf::default() }
            ).text.as_str()
            
                ).on_input(Message::TextUpdate))
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
