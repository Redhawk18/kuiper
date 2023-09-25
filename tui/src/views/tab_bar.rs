use cursive::{view::Nameable, views::TextView, Cursive};
use cursive_tabs::TabPanel;

pub fn tab_bar(siv: &mut Cursive) {
    let panel = TabPanel::new()
        .with_tab(TextView::new("This is the first view!").with_name("First"))
        .with_tab(TextView::new("This is the second view!").with_name("Second"));

    siv.add_layer(panel);
}
