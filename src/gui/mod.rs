use std::path::PathBuf;

use iced::{widget::Column, Length, Sandbox};

//use iced::widget::column;
//use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Subscription};
use iced_aw::menu::MenuBar;
use iced_aw::{TabBar, TabLabel};

mod file_dialog;
mod menu_bar;

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

pub struct State {
    text: String,
    path: PathBuf,

    //tab
    active_tab: usize,
    new_tab_label: String,
    new_tab_content: String,
    tabs: Vec<(String, String)>,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = theme::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                text: String::from(""),
                path: PathBuf::default(),

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
                self.text = text;
            }

            Message::NewFile => return self.update(Message::NewTab),

            Message::OpenFile => {
                let (file_contents, path) = file_dialog::pick_file();

                self.path = path;
                match file_contents {
                    Ok(v) => {
                        return self.update(Message::TextUpdate(v));
                    }
                    Err(_e) => {
                        return Command::none();
                    }
                }
            }

            Message::OpenFolder => {
                file_dialog::pick_folder();
            }

            Message::Save => {
                file_dialog::save_file(self.text.as_str(), &self.path).unwrap();
            }

            Message::SaveAs => {
                file_dialog::save_as(self.text.as_str(), &self.path).unwrap();
            }
            Message::TabSelected(index) => self.active_tab = index,
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
                // if !self.new_tab_label.is_empty() && !self.new_tab_content.is_empty() {
                println!("Create");
                self.tabs
                    .push(("name of tab".to_string(), "contents of tab".to_string()));
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = MenuBar::new(vec![file(self)]);

        // let placeholder = "Deleted code is debugged code.";
        // let text_input = TextInput::new(placeholder, &self.text).on_input(Message::TextUpdate);

        // column![menu_bar, text_input].into()
        Column::new()
            .push(menu_bar)
            .push(
                self.tabs
                    .iter()
                    .fold(
                        TabBar::new(self.active_tab, Message::TabSelected),
                        |tab_bar, (tab_label, _)| {
                            tab_bar.push(TabLabel::Text(tab_label.to_owned()))
                        },
                    )
                    .on_close(Message::TabClosed)
                    .tab_width(Length::Shrink)
                    .spacing(5.0)
                    .padding(5.0)
                    .text_size(32.0),
            )
            .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
