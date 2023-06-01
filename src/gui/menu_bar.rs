use iced::widget::button;
use iced_aw::menu::{MenuBar, MenuTree};

pub fn file<'a>(_app: &super::State) -> MenuTree<'a, super::Message, iced::Renderer> {
    let open_file = MenuTree::new(button("Open File",)); 

    let open_folder = MenuTree::new(button("Open Folder",));

    let save = MenuTree::new(button("Save",));

    let save_as = MenuTree::new(button("Open File",));

    let root = MenuTree::with_children(button("File"), vec![
        open_file,
        open_folder,
        save,
        save_as,
    ]);

    root
}
