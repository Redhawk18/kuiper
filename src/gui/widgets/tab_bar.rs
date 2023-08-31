use crate::gui::{
    theme::{Element, Renderer},
    Message, Tab,
};

use iced::widget::{column, text_input, Column};
use iced_aw::{TabBar, TabLabel};
use slotmap::{DefaultKey, SlotMap};

pub fn tab_bar<'a>(
    active: usize,
    data: &[DefaultKey],
    map: &SlotMap<DefaultKey, Tab>,
) -> Column<'a, Message, Renderer> {
    if data.is_empty() {
        column!()
    } else {
        column!(head(active, data, map), body(active, data, map))
    }
}

fn head(
    active: usize,
    data: &[DefaultKey],
    map: &SlotMap<DefaultKey, Tab>,
) -> TabBar<Message, usize, Renderer> {
    let mut tab_bar = TabBar::new(Message::TabSelected).on_close(Message::TabClosed);

    for (i, key) in data.iter().enumerate() {
        let tab = map.get(*key).unwrap();
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

    tab_bar.set_active_tab(&active)
}

fn body<'a>(
    active: usize,
    data: &[DefaultKey],
    map: &SlotMap<DefaultKey, Tab>,
) -> Element<'a, Message> {
    //let key =

    let tab = map.get(*data.get(active).unwrap()).unwrap();
    match tab {
        Tab::File(file_tab) => text_input("placeholder", &file_tab.text)
            .on_input(Message::TextUpdate)
            .into(),
    }
}
