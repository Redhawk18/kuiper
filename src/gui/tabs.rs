use iced::Element;
use iced::widget::row;
use iced_aw::{TabBar, TabLabel};

pub fn tab_header(tabs: &Vec<super::FileTab>, active_tab: usize) -> Element<super::Message> {
    let mut tab_bar =
        TabBar::new(active_tab, super::Message::TabSelected).on_close(super::Message::TabClosed);

    for tab in tabs.iter() {
        //TODO if let some tab.path is "" else name it new tab
        tab_bar = tab_bar.push(TabLabel::Text(String::from("tab name!")));
    }

    row!(tab_bar).into()
}
