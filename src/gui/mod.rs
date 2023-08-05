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
    TabNew(FileTab),
    TabSelected(elements::TabId),
    TabClosed(elements::TabId),
}

pub struct Blaze {
    tabs: Tabs,
}

struct Tabs {
    tab_bar: iced_aw::TabBar<Message, elements::TabId>,
    data: Vec<Tab>,
}

impl Default for Tabs {
    fn default() -> Self {
        Self {
            tab_bar: iced_aw::TabBar::new(Message::TabSelected),
            data: Vec::new(),
        }
    }
}

pub enum Tab {
    File(FileTab),
}

#[derive(Debug, Clone)]
pub struct FileTab {
    id: elements::TabId,
    text: String,
    path: PathBuf,
}

impl Default for FileTab {
    fn default() -> Self {
        Self {
            id: elements::TabId::File,
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

            Message::NewFile => return self.update(Message::TabNew(FileTab::default())),

            Message::OpenFile => todo!(),

            Message::OpenFolder => file_dialog::pick_folder(),

            Message::Save => todo!(),

            Message::SaveAs => todo!(),

            Message::Quit => return iced::window::close(),

            Message::TabNew(tab) => todo!(),

            Message::TabSelected(id) => todo!(),

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
            elements::tab_header(&self.tabs.data, &mut self.tabs.tab_bar)
        }

        c.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
