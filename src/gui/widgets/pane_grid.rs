use crate::gui::{theme::Renderer, widgets::tab_bar::tab_bar, Message, PaneState, Panes, Tabs};

use iced::widget::pane_grid::{Content, PaneGrid, TitleBar};

pub fn pane_grid<'a>(panes: &'a Panes, tabs: &'a Tabs) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let is_focused = panes.active == Some(pane);
        Content::new(match state {
            PaneState::Tab => tab_bar(tabs.active, &tabs.data),
        })
        .style(crate::gui::theme::Container::PaneGridContent(is_focused))
        .title_bar(
            TitleBar::new("content")
                .style(crate::gui::theme::Container::PaneGridTitleBar(is_focused)),
        )
    })
    .on_click(Message::PaneClick)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
