use crate::{Button, Message, Widgets};

use iced::{
    widget::{button, row},
    Element,
};

pub(crate) fn menu_bar() -> Element<'static, Message> {
    let new_file = button("New File").on_press(Button::NewFile).width(150);

    let open_file = button("Open File").on_press(Button::OpenFile).width(150);

    let open_folder = button("Open Folder")
        .on_press(Button::OpenFolder)
        .width(150);

    let save = button("Save").on_press(Button::Save).width(150);

    let save_as = button("Save As").on_press(Button::SaveAs).width(150);

    let quit = button("Quit").on_press(Button::Quit).width(150);

    Element::from(row![new_file, open_file, open_folder, save, save_as, quit])
        .map(|x| Message::Widgets(Widgets::Button(x)))
}
