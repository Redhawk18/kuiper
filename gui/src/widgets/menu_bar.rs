use crate::{theme::Renderer, Message};

use iced::widget::button;
use iced_aw::{MenuBar, MenuTree};

pub(crate) fn menu_bar() -> MenuBar<'static, Message, Renderer> {
    MenuBar::new(vec![file()])
}

fn file() -> MenuTree<'static, Message, Renderer> {
    let new_file = MenuTree::new(button("New File").on_press(Message::NewFile).width(150));

    let open_file = MenuTree::new(button("Open File").on_press(Message::OpenFile).width(150));

    let open_folder = MenuTree::new(
        button("Open Folder")
            .on_press(Message::OpenFolder)
            .width(150),
    );

    let save = MenuTree::new(button("Save").on_press(Message::Save).width(150));

    let save_as = MenuTree::new(button("Save As").on_press(Message::SaveAs).width(150));

    let quit = MenuTree::new(button("Quit").on_press(Message::Quit).width(150));

    let root = MenuTree::with_children(
        button("File"),
        vec![new_file, open_file, open_folder, save, save_as, quit],
    );

    root
}
