mod core;
mod gui;
pub use gui::Blaze;

use iced::{Application, Settings};

fn main() -> iced::Result {
    pretty_env_logger::init();

    Blaze::run(Settings::default())
}
