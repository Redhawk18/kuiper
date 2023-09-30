mod theme;
mod views;
use theme::theme;
use views::{menu_bar, tab_bar};

use blaze_core::data::Tab;
use cursive::{event::Key, Cursive, CursiveRunnable};

pub fn start_tui() {
    let mut application = Blaze::new();
    application.run()
}

pub(crate) struct Blaze {
    siv: CursiveRunnable,
}

pub(crate) enum Message {
    // menu bar
    NewFile,
    OpenFile,
    OpenFolder,
    Save,
    SaveAs,
    Quit,

    //tabs
    TabNew(Tab),
    TabSelected(usize),
    TabClosed(usize),
}

impl Blaze {
    fn new() -> Self {
        Self {
            siv: cursive::default(),
        }
    }

    fn global_callbacks(&mut self) {
        self.siv.add_global_callback('q', Cursive::quit);
    }

    fn menu_bar(&mut self) {
        menu_bar(&mut self.siv);
        self.siv
            .add_global_callback(Key::Esc, |s| s.select_menubar());
    }

    pub fn run(&mut self) {
        self.global_callbacks();
        self.menu_bar();
        self.theme();
        self.view();
        self.siv.run()
    }

    fn theme(&mut self) {
        let theme = theme(self.siv.current_theme().clone());
        self.siv.set_theme(theme);
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::NewFile => todo!(),
            Message::OpenFile => todo!(),
            Message::OpenFolder => todo!(),
            Message::Save => todo!(),
            Message::SaveAs => todo!(),
            Message::Quit => todo!(),
            Message::TabNew(_) => todo!(),
            Message::TabSelected(_) => todo!(),
            Message::TabClosed(_) => todo!(),
        }
    }

    fn view(&mut self) {
        let theme = self.siv.current_theme().clone();
        self.siv.add_layer(tab_bar(theme));
    }
}
