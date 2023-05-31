use iced::widget::TextInput;
use iced::{theme, Application, Command, Element, Subscription};
use iced::widget::button;
use iced::widget::column;
use iced_aw::menu::{CloseCondition, ItemHeight, ItemWidth, MenuBar, MenuTree, PathHighlight};

#[derive(Debug, Clone)]
pub enum Message {
    Update(String),
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
            Message::Update(new_text) => {
                self.text = new_text;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let menu_bar = MenuBar::new(vec![file(self),]);

        let text_input = Element::new(TextInput::new("", &self.text).on_input(Message::Update));

        let c = column!(menu_bar, text_input);

        c.into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

fn file<'a>(app: &State) -> MenuTree<'a, Message, iced::Renderer> {
    let root = MenuTree::new(button("File",));

    root
}
