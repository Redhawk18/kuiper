use iced::{Application, Settings};

mod core;
mod gui;

pub use gui::Blaze;

fn main() -> iced::Result {
    pretty_env_logger::init();

    Blaze::run(Settings::default())
}
