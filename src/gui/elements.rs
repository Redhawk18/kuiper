use iced::widget::button;
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{TabBar, TabLabel};

use super::{FileTab, Message, Tab};

use crate::gui::theme::Renderer;

pub fn menu_bar<'a>() -> MenuBar<'a, Message, Renderer> {
    MenuBar::new(vec![file()])
}

fn file<'a>() -> MenuTree<'a, Message, Renderer> {
    let new_file =
        MenuTree::new(button("New File").on_press(Message::TabNew(Tab::File(FileTab::default()))).width(150),);

    let open_file = MenuTree::new(
        button("Open File")
            .on_press(super::Message::OpenFile)
            .width(150),
    );

    let open_folder = MenuTree::new(
        button("Open Folder")
            .on_press(super::Message::OpenFolder)
            .width(150),
    );

    let save = MenuTree::new(button("Save").on_press(super::Message::Save).width(150));

    let save_as = MenuTree::new(
        button("Save As")
            .on_press(super::Message::SaveAs)
            .width(150),
    );

    let quit = MenuTree::new(button("Quit").on_press(super::Message::Quit).width(150));

    let root = MenuTree::with_children(
        button("File"),
        vec![new_file, open_file, open_folder, save, save_as, quit],
    );

    root
}

pub fn tab_header(active: usize, data: &[Tab]) -> TabBar<Message, usize, Renderer> {
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

    tab_bar.set_active_tab(&active)
}
