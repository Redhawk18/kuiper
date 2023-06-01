use iced::widget::button;
use iced_aw::menu::{MenuBar, MenuTree};

pub fn file<'a>(_app: &super::State) -> MenuTree<'a, super::Message, iced::Renderer> {
    let root = MenuTree::new(button("File",));

    root
}
