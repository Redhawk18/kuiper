use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Subscription};
use iced::widget::column;
use iced_aw::menu::MenuBar;
use rfd::FileDialog;

mod menu_bar;

pub use menu_bar::file;

#[derive(Debug, Clone)]
pub enum Message {
    TextUpdate(String),

    //menu bar
    NewFile(),
    OpenFile(),
    OpenFolder(),
    Save(),
    SaveAs(),    
}

pub struct State {
    text: String,
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

            Message::NewFile() => todo!(),

            Message::OpenFile() => {
                let files = FileDialog::new()
                    .set_directory("~/")
                    .pick_file();

                println!("{:?}", files);
            }
            
            Message::OpenFolder() => todo!(),

            Message::Save() => todo!(),

            Message::SaveAs() => todo!(),

        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = MenuBar::new(vec![file(self),]);

        let text_input = TextInput::new("code...", &self.text).on_input(Message::TextUpdate);

        column![menu_bar, text_input].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
