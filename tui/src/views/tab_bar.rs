use cursive::{view::Nameable, views::{TextView, Dialog}, Cursive};
use cursive_tabs::{TabView, TabPanel};

pub fn tab_bar(siv: &mut Cursive) {
    let mut panel = TabPanel::new()
    .with_tab(TextView::new("This is the first view!").with_name("First"))
    .with_tab(TextView::new("This is the second view!").with_name("Second"));

    siv.add_layer(panel);
}