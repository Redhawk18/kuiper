use crate::gui::theme::{Container, Renderer, Theme};
use crate::gui::widgets::tab_bar;
use crate::gui::{Message, PaneState, Tab};

use iced::widget::pane_grid::{Content, PaneGrid, State, TitleBar};
use iced::widget::{container, text};

use super::tab_bar::tab_bar;

pub fn pane_grid<'a>(
    state: &'a State<PaneState>,
    active: usize,
    data: &'a [Tab],
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(state, |pane, state, is_maximized| {
        Content::new(match state {
            PaneState::Tab => tab_bar(active, data),
        })
        .title_bar(
            TitleBar::new("content").style(crate::gui::theme::Container::PaneGridTitleBar(true))
        )
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
