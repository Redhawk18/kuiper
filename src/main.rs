use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Settings, Subscription};

fn main() -> iced::Result {
    ApplicationState::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
}

struct ApplicationState {
    text: String,
}

impl Application for ApplicationState {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            ApplicationState {
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
            Message::Update(new_text) => {
                self.text = new_text;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Element::new(TextInput::new("", &self.text).on_input(|new_text| Message::Update(new_text)))
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}
