use iced::widget::{button, row};
use iced::Element;
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{TabBar, TabLabel};

use super::Tab;

pub fn menu_bar<'a>() -> MenuBar<'a, super::Message, iced::Renderer> {
    MenuBar::new(vec![file()])
}

fn file<'a>() -> MenuTree<'a, super::Message, iced::Renderer> {
    let new_file = MenuTree::new(
        button("New File").on_press(super::Message::TabNew(super::Tab::File(super::FileTab::default()))),
    );

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    File,
}

pub fn tab_header(active: usize, data: &[Tab]) -> TabBar<super::Message, usize> {
    let mut tab_bar = TabBar::new(super::Message::TabSelected);

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

    tab_bar.set_active_tab(&active)
}
