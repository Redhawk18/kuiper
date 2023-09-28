use cursive::{view::{Nameable, Resizable}, views::{TextView, TextArea, PaddedView}, Cursive};
use cursive_tabs::TabPanel;

pub fn tab_bar(siv: &mut Cursive) {
    let panel = TabPanel::new()
        .with_tab(TextView::new("This is the first view!").with_name("First"))
        .with_tab(TextView::new("This is the second view!").with_name("Second"))
        .with_tab(PaddedView::lrtb(1, 1, 1, 1, TextArea::new()).with_name("4"));

    siv.add_layer(panel.full_screen());
}
