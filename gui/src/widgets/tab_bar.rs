use crate::{
    theme::{Element, Renderer},
    Message, Tab,
};

use iced::{
    widget::{column, text_input, Column},
    Length,
};
use iced_aw::{TabBar, TabLabel};

pub(crate) fn tab_bar(active: usize, data: &[&Tab]) -> Column<'static, Message, Renderer> {
    if data.is_empty() {
        column!()
    } else {
        column!(head(active, data), body(active, data)).padding(1)
    }
}

fn head(active: usize, data: &[&Tab]) -> TabBar<Message, usize, Renderer> {
    let mut tab_bar = TabBar::new(Message::TabSelected).on_close(Message::TabClosed);

    for (i, tab) in data.iter().enumerate() {
        match tab {
            Tab::File(file_tab) => {
                let filename = file_tab
                    .path
                    .file_name() // this already checks the "empty" case
                    .and_then(|filename| filename.to_str())
                    .unwrap_or("New Tab");

                tab_bar = tab_bar.push(i, TabLabel::Text(String::from(filename)));
            }
        }
    }

    tab_bar.set_active_tab(&active).tab_width(Length::Shrink)
}

fn body(active: usize, data: &[&Tab]) -> Element<'static, Message> {
    let active_tab = data.get(active).unwrap(); //wrong

    match active_tab {
        Tab::File(file_tab) => text_input("placeholder", &file_tab.text)
            .on_input(Message::TextUpdate)
            .into(),
    }
}
