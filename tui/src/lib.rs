mod theme;
mod views;
use theme::theme;
use views::{menu_bar, tab_bar};

use blaze_core::data::Tab;
use cursive::{event::Key, Cursive, CursiveRunnable};

pub fn start_tui() {
    let mut siv = CursiveRunnable::default();
    siv.set_user_data(Blaze);
    run(&mut siv)
}

pub(crate) struct Blaze;

pub(crate) enum Message {
    // menu bar
    NewFile,
    OpenFile,
    OpenFolder,
    Save,
    SaveAs,
}

impl Blaze {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::NewFile => todo!(),
            Message::OpenFile => todo!(),
            Message::OpenFolder => todo!(),
            Message::Save => todo!(),
            Message::SaveAs => todo!(),
        }
    }
}

fn custom_theme(siv: &mut CursiveRunnable) {
    let theme = theme(siv.current_theme().clone());
    siv.set_theme(theme);
}

fn global_callbacks(siv: &mut CursiveRunnable) {
    siv.add_global_callback('q', Cursive::quit);
}

fn run(siv: &mut CursiveRunnable) {
    global_callbacks(siv);
    menu_bar(siv);
    custom_theme(siv);
    view(siv);
    siv.run()
}

fn view(siv: &mut CursiveRunnable) {
    let theme = siv.current_theme().clone();
    siv.add_layer(tab_bar(theme));
}
