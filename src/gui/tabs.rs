use iced::widget::row;
use iced::Element;
use iced_aw::{TabBar, TabLabel};

pub fn tab_header(tabs: &[super::FileTab], active_tab: usize) -> Element<super::Message> {
    let mut tab_bar =
        TabBar::new(active_tab, super::Message::TabSelected).on_close(super::Message::TabClosed);

    for tab in tabs.iter() {
        let filename = tab
            .path
            .file_name() // this already checks the "empty" case
            .and_then(|filename| filename.to_str())
            .unwrap_or("New Tab");

        tab_bar = tab_bar.push(TabLabel::Text(String::from(filename)));
    }

    row!(tab_bar).into()
}
