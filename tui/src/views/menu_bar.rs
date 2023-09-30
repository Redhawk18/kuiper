use cursive::{menu::Tree, Cursive};

/// Due to how to part of the ui is designed, It's not possible to return the view created.
/// A mut reference is required to add a menubar.
pub fn menu_bar(siv: &mut Cursive) {
    siv.menubar().add_subtree("File", file());
    siv.set_autohide_menu(false);
}

fn file() -> Tree {
    Tree::new()
        .leaf("New File", |_| {})
        .leaf("Open File", |_| {})
        .leaf("Open Folder", |_| {})
        .leaf("Save", |_| {})
        .leaf("Save As", |_| {})
        .leaf("Quit", |s| s.quit())
}
