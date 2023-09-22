use cursive::{menu::Tree, Cursive};

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
