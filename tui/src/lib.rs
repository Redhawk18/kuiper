mod views;
use views::menu_bar::menu_bar;

use cursive::theme::{Color, PaletteColor, Theme};
use cursive::{event::Key, Cursive};

//pub(crate) struct Blaze {}

pub fn start_tui() {
    tui()
}

fn tui() {
    let mut siv = cursive::default();

    let theme = custom_theme_from_cursive(&siv);
    siv.set_theme(theme);

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    menu_bar(&mut siv);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    // Run the event loop
    siv.run();
}

fn custom_theme_from_cursive(siv: &Cursive) -> Theme {
    // We'll return the current theme with a small modification.
    let mut theme = siv.current_theme().clone();

    theme.palette[PaletteColor::Background] = Color::TerminalDefault;
    theme
}
