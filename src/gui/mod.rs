use iced::widget::column;
use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Subscription};
use iced_aw::menu::MenuBar;

mod file_dialog;
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

            Message::NewFile() => {
                todo!();
            }

            Message::OpenFile() => {
                let file_contents = file_dialog::pick_file();
                match file_contents {
                    Ok(v) => {
                        return self.update(Message::TextUpdate(v));
                    }
                    Err(_e) => {
                        return Command::none();
                    }
                }
                
            }

            Message::OpenFolder() => {
                file_dialog::pick_folder();
            }

            Message::Save() => {
                todo!();
            }

            Message::SaveAs() => {
                todo!();
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = MenuBar::new(vec![file(self)]);

        let placeholder = "Deleted code is debugged code.";
        let text_input = TextInput::new(placeholder, &self.text).on_input(Message::TextUpdate);

        column![menu_bar, text_input].into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
