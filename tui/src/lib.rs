mod theme;
mod views;
use theme::theme;
use views::menu_bar::menu_bar;

use cursive::{event::Key, Cursive};

//pub(crate) struct Blaze {}

pub fn start_tui() {
    tui()
}

fn tui() {
    let mut siv = cursive::default();

    let theme = theme(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    menu_bar(&mut siv);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // Run the event loop
    siv.run();
}
