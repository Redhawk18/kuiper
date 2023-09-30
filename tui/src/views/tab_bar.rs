use crate::views::text_area;

use cursive::{
    theme::Theme,
    view::{Nameable, Resizable},
    views::{PaddedView, ResizedView, TextView},
};
use cursive_tabs::TabPanel;

pub fn tab_bar(theme: Theme) -> ResizedView<TabPanel> {
    let panel = TabPanel::new()
        .with_tab(TextView::new("This is the first view!").with_name("First"))
        .with_tab(TextView::new("This is the second view!").with_name("Second"))
        .with_tab(PaddedView::lrtb(1, 1, 1, 1, text_area(theme)).with_name("4"));

    panel.full_screen()
}
