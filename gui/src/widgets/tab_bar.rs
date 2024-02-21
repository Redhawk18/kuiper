use crate::{Message, Tab};

use iced::{
    widget::{column, text_editor, Column},
    Element, Length, Renderer, Theme,
};
use iced_aw::{TabBar, TabLabel};

pub(crate) fn tab_bar<'a>(active: usize, data: &[&'a Tab]) -> Column<'a, Message, Theme, Renderer> {
    if data.is_empty() {
        column!()
    } else {
        column!(head(active, data), body(active, data)).padding(1)
    }
}

fn head(active: usize, data: &[&Tab]) -> TabBar<Message, usize, Theme, Renderer> {
    let mut tab_bar = TabBar::new(Message::TabSelected).on_close(Message::TabClosed);

    for (i, tab) in data.iter().enumerate() {
        match tab {
            Tab::File(file_tab) => {
                let filename = match &file_tab.path {
                    Some(path) => path.file_name().unwrap().to_str().unwrap(),
                    None => "New Tab",
                };

                tab_bar = tab_bar.push(i, TabLabel::Text(String::from(filename)));
            }
        }
    }

    tab_bar.set_active_tab(&active).tab_width(Length::Shrink)
}

fn body<'a>(active: usize, data: &[&'a Tab]) -> Element<'a, Message> {
    let active_tab = data.get(active).unwrap(); //wrong

    match active_tab {
        Tab::File(file_tab) => text_editor(&file_tab.content)
            .height(Length::Fill)
            .on_action(Message::TextEditorUpdate)
            .into(),
    }
}
