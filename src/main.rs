use iced::{Application, Settings};

mod gui;

pub use gui::State;

fn main() -> iced::Result {
    State::run(Settings::default())
}
