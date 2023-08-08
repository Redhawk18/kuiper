use iced::widget::button;
use iced_aw::menu::{MenuBar, MenuTree};
use iced_aw::{TabBar, TabLabel};

use super::{FileTab, Message, Tab};

pub fn menu_bar<'a>() -> MenuBar<'a, Message, iced::Renderer> {
    MenuBar::new(vec![file()])
}

fn file<'a>() -> MenuTree<'a, Message, iced::Renderer> {
    let new_file =
        MenuTree::new(button("New File").on_press(Message::TabNew(Tab::File(FileTab::default()))));

    let open_file = MenuTree::new(button("Open File").on_press(Message::OpenFile));

    let open_folder = MenuTree::new(button("Open Folder").on_press(Message::OpenFolder));

    let save = MenuTree::new(button("Save").on_press(Message::Save));

    let save_as = MenuTree::new(button("Save As").on_press(Message::SaveAs));

    let quit = MenuTree::new(button("Quit").on_press(Message::Quit));

    let root = MenuTree::with_children(
        button("File"),
        vec![new_file, open_file, open_folder, save, save_as, quit],
    );

    root
}

pub fn tab_header(active: usize, data: &[Tab]) -> TabBar<Message, usize> {
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
