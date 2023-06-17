use iced::widget::{button, column, row, text, text_input};
use iced::Element;
use iced_aw::{TabBar, TabLabel};

pub fn tab_header(tabs: &Vec<super::FileTab>) -> Element<super::Message> {
    let mut tab_labels: Vec<Element<super::Message>> = Vec::new();

    for (index, _tab) in tabs.iter().enumerate() {
        let button = button(text("NAME OF TAB")).on_press(super::Message::TabSelected(index));
        tab_labels.push(button.into());
    }

    column!(row(tab_labels)).into()
}
