use iced::{Element, Length, Renderer, Theme};
use iced_aw::{TabBar, TabLabel};

use crate::buffer::Buffer;
use crate::icon::{self, path_to_char};

#[derive(Debug, Clone)]
pub enum Message {
    Selected(usize),
    Closed(usize),
}

pub(super) fn view<'a, I>(active: usize, buffers: I) -> Element<'a, Message, Theme, Renderer>
where
    I: Iterator<Item = &'a Buffer>,
{
    let mut tab_bar = TabBar::new(Message::Selected).on_close(Message::Closed);

    for (i, buffer) in buffers.enumerate() {
        match buffer {
            Buffer::File(file_buffer) => {
                let file_name = if let Some(path) = &file_buffer.path {
                    path.file_name()
                        .expect("path is some, filename should not be none")
                        .to_string_lossy()
                        .to_string()
                } else {
                    "New Tab".to_string()
                };
                let file_icon = if let Some(path) = &file_buffer.path {
                    path_to_char(path)
                } else {
                    icon::file()
                };

                tab_bar = tab_bar.push(i, TabLabel::IconText(file_icon, file_name));
            }
        }
    }

    Element::from(tab_bar.set_active_tab(&active).tab_width(Length::Shrink))
}
