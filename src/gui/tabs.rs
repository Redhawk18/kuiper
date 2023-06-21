use std::path::PathBuf;

use iced::Element;
use iced::{widget::row, Application};
use iced_aw::{TabBar, TabLabel};

pub fn tab_header(tabs: &Vec<super::FileTab>, active_tab: usize) -> Element<super::Message> {
    let mut tab_bar =
        TabBar::new(active_tab, super::Message::TabSelected).on_close(super::Message::TabClosed);

    for _tab in tabs.iter() {
        //TODO if let some tab.path is "" else name it new tab
        tab_bar = tab_bar.push(TabLabel::Text(String::from("tab name!")));
    }

    row!(tab_bar).into()
}

// Abstracts the logic of getting the current
// pub fn get_active_tab(
//     tabs: &Vec<super::FileTab>,
//     active_tab: &usize,
//     state: super::State,
// ) -> &super::FileTab {
//     let option = tabs.get(active_tab);

//     match option {
//         Some(v) => return v,
//         None => {
//             let tab = super::FileTab {
//                 text: "get_active_tab".to_string(),
//                 path: PathBuf::default(),
//             };
//             state.update(super::Message::NewTab(tab));
//             tabs.get(active_tab).unwrap()
//         }
//     }
// }
