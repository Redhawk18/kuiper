use crate::gui::theme::Renderer;
use crate::gui::{Tabs, Panes};

use crate::gui::{Message, PaneState, Tab};

use iced::widget::pane_grid::{Content, PaneGrid, State, TitleBar};

use super::tab_bar::tab_bar;

pub fn pane_grid<'a>(panes: &'a Panes, tabs: &'a Tabs) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        Content::new(match state {
            PaneState::Tab => tab_bar(tabs.active, &tabs.data),
        })
        .style(crate::gui::theme::Container::PaneGridTitleBar(true))
        .title_bar(
            TitleBar::new("content").style(crate::gui::theme::Container::PaneGridTitleBar(true)),
        )
    })
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
