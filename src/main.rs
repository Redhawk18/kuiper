use iced::{Application, Settings};

mod gui;

pub use gui::ApplicationState;

fn main() -> iced::Result {
    ApplicationState::run(Settings::default())
}
