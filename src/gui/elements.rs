use iced::widget::{button, row};
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{TabBar, TabLabel};

use crate::gui::theme::{Element, Renderer};

pub fn menu_bar<'a>() -> MenuBar<'a, super::Message, Renderer> {
    MenuBar::new(vec![file()])
}

fn file<'a>() -> MenuTree<'a, super::Message, Renderer> {
    let new_file = MenuTree::new(button("New File").on_press(super::Message::TabNew(
        super::FileTab {
            text: std::string::String::default(),
            path: std::path::PathBuf::default(),
        },
    )));

    let open_file = MenuTree::new(button("Open File").on_press(super::Message::OpenFile));

    let open_folder = MenuTree::new(button("Open Folder").on_press(super::Message::OpenFolder));

    let save = MenuTree::new(button("Save").on_press(super::Message::Save));

    let save_as = MenuTree::new(button("Save As").on_press(super::Message::SaveAs));

    let quit = MenuTree::new(button("Quit").on_press(super::Message::Quit));

    let root = MenuTree::with_children(
        button("File"),
        vec![new_file, open_file, open_folder, save, save_as, quit],
    );

    root
}

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
