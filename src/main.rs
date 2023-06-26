use iced::{Application, Settings};

mod core;
mod gui;

pub use gui::State;

fn main() -> iced::Result {
    pretty_env_logger::init();

    State::run(Settings::default())
}
