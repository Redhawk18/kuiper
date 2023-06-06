use iced::widget::button;
use iced_aw::menu::MenuTree;

pub fn file<'a>(_app: &super::State) -> MenuTree<'a, super::Message, iced::Renderer> {
    let new_file = MenuTree::new(button("New File").on_press(super::Message::NewFile()));

    let open_file = MenuTree::new(button("Open File").on_press(super::Message::OpenFile()));

    let open_folder = MenuTree::new(button("Open Folder").on_press(super::Message::OpenFolder()));

    let save = MenuTree::new(button("Save").on_press(super::Message::Save()));

    let save_as = MenuTree::new(button("Save As").on_press(super::Message::SaveAs()));

    let root = MenuTree::with_children(
        button("File"),
        vec![new_file, open_file, open_folder, save, save_as],
    );

    root
}
