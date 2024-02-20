use crate::Message;

use iced::{
    widget::{button, row, Row},
    // Renderer,
};
// use iced_aw::{MenuBar, MenuTree};

// pub(crate) fn menu_bar() -> MenuBar<'static, Message, Renderer> {
//     MenuBar::new(vec![file()])
// }

// menu is broken currently
pub(crate) fn menu_bar() -> Row<'static, Message> {
    let new_file = button("New File").on_press(Message::NewFile).width(150);

    let open_file = button("Open File").on_press(Message::OpenFile).width(150);

    let open_folder = button("Open Folder")
        .on_press(Message::OpenFolder)
        .width(150);

    let save = button("Save").on_press(Message::Save).width(150);

    let save_as = button("Save As").on_press(Message::SaveAs).width(150);

    let quit = button("Quit").on_press(Message::Quit).width(150);

    row![new_file, open_file, open_folder, save, save_as, quit]
}

// fn file() -> MenuTree<'static, Message, Renderer> {
//     let new_file = MenuTree::new(button("New File").on_press(Message::NewFile).width(150));

//     let open_file = MenuTree::new(button("Open File").on_press(Message::OpenFile).width(150));

//     let open_folder = MenuTree::new(
//         button("Open Folder")
//             .on_press(Message::OpenFolder)
//             .width(150),
//     );

//     let save = MenuTree::new(button("Save").on_press(Message::Save).width(150));

//     let save_as = MenuTree::new(button("Save As").on_press(Message::SaveAs).width(150));

//     let quit = MenuTree::new(button("Quit").on_press(Message::Quit).width(150));

//     MenuTree::with_children(
//         button("File"),
//         vec![new_file, open_file, open_folder, save, save_as, quit],
//     )
// }
