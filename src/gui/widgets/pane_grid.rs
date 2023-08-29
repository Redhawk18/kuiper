use crate::gui::theme::{Container, Renderer, Theme};
use crate::gui::{Message, PaneState};

use iced::widget::pane_grid::{Content, PaneGrid, State, TitleBar};
use iced::widget::{container, text};

pub fn pane_grid<'a>(
    state: &'a State<PaneState>
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(state, |pane, state, is_maximized| {
        Content::new(match state {
            PaneState::SomePane => text("This is some pane"),
            PaneState::AnotherKindOfPane => text("This is another kind of pane"),
        })
        .title_bar(TitleBar::new("content").style(crate::gui::theme::Container::Primary))
    })        .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
