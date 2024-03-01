use crate::{Buffer, Message};

use iced::{
    widget::{column, text_editor, Column},
    Element, Length, Renderer, Theme,
};
use iced_aw::{TabBar, TabLabel};

pub(crate) fn tab_bar<'a>(
    active: usize,
    data: &[&'a Buffer],
) -> Column<'a, Message, Theme, Renderer> {
    if data.is_empty() {
        column!()
    } else {
        column!(head(active, data), body(active, data)).padding(1)
    }
}

fn head(active: usize, data: &[&Buffer]) -> TabBar<Message, usize, Theme, Renderer> {
    let mut tab_bar = TabBar::new(Message::TabSelected).on_close(Message::TabClosed);

    for (i, tab) in data.iter().enumerate() {
        match tab {
            Buffer::File(file_buffer) => {
                let filename = match &file_buffer.path {
                    Some(path) => path.file_name().unwrap().to_str().unwrap(),
                    None => "New Tab",
                };

                tab_bar = tab_bar.push(i, TabLabel::Text(String::from(filename)));
            }
        }
    }

    tab_bar.set_active_tab(&active).tab_width(Length::Shrink)
}

fn body<'a>(active: usize, data: &[&'a Buffer]) -> Element<'a, Message> {
    let active_tab = data.get(active).unwrap(); //wrong

    match active_tab {
        Buffer::File(file_buffer) => text_editor(&file_buffer.content)
            .height(Length::Fill)
            .on_action(Message::TextEditorUpdate)
            .into(),
    }
}
