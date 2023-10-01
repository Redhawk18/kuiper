use crate::{Message, Blaze};

use cursive::{event::Key, menu::Tree, Cursive};

/// Due to how to part of the ui is designed, It's not possible to return the view created.
/// A mut reference is required to add a menubar.
pub fn menu_bar(siv: &mut Cursive) {
    siv.menubar().add_subtree("File", file());
    siv.set_autohide_menu(false);
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());
}

fn file() -> Tree {
    Tree::new()
        .leaf("New File", |s| {s.user_data::<Blaze>().unwrap().update(Message::NewFile)})
        .leaf("Open File", |s| {s.user_data::<Blaze>().unwrap().update(Message::OpenFile)})
        .leaf("Open Folder", |s| {s.user_data::<Blaze>().unwrap().update(Message::OpenFolder)})
        .leaf("Save", |s| {s.user_data::<Blaze>().unwrap().update(Message::Save)})
        .leaf("Save As", |s| {s.user_data::<Blaze>().unwrap().update(Message::SaveAs)})
        .leaf("Quit", |s| s.quit())
}
