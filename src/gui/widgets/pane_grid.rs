use crate::gui::{theme::{Renderer, Container}, widgets::tab_bar::tab_bar, Message, PaneState, Panes, Tabs};

use iced::widget::pane_grid::{Content, PaneGrid, TitleBar};

pub fn pane_grid<'a>(panes: &'a Panes, tabs: &'a [GenerationalIndex], gen_array: &GenerationalArray<Tab>) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let is_focused = panes.active == pane;
        Content::new(match state.tab {
            crate::gui::Tab::File(_) => tab_bar(state.active_tab, &tabs, gen_array),
        })
        .style(Container::PaneGridContent(is_focused))
        .title_bar(
            TitleBar::new("content")
                .style(Container::PaneGridTitleBar(is_focused)),
        )
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}
