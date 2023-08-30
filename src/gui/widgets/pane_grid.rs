use crate::gui::{
    theme::{Element, Renderer},
    widgets::tab_bar::tab_bar,
    Message, PaneState, Panes, Tabs,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row, text,
    },
    Alignment,
};

pub fn pane_grid<'a>(panes: &'a Panes, tabs: &'a Tabs) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let is_focused = panes.active == Some(pane);
        Content::new(match state {
            PaneState::Tab => tab_bar(tabs.active, &tabs.data),
        })
        .style(crate::gui::theme::Container::PaneGridContent(is_focused))
        .title_bar(
            TitleBar::new(title_bar(pane, state))
                .style(crate::gui::theme::Container::PaneGridTitleBar(is_focused)),
        )
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
}

fn title_bar(pane: Pane, _state: &PaneState) -> Element<'static, Message> {
    row!(
        text("content"),
        button("split |").on_press(Message::PaneSplit(Axis::Vertical, pane)),
        button("split -").on_press(Message::PaneSplit(Axis::Horizontal, pane)),
        button("close THIS pane").on_press(Message::PaneClosed(pane))
    )
    .align_items(Alignment::Center)
    .into()
}
