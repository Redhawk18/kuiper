use crate::gui::{
    theme::{Container, Element, Renderer},
    widgets::tab_bar::tab_bar,
    Message, PaneState, Panes, Tab,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row, text,
    },
    Alignment,
};
use slotmap::{DefaultKey, SlotMap};

pub fn pane_grid<'a>(
    panes: &'a Panes,
    tabs: &'a [DefaultKey],
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let is_focused = panes.active == pane;
        Content::new(match state.tab {
            crate::gui::Tab::File(_) => tab_bar(state.active_tab, &tabs, map),
        })
        .style(Container::PaneGridContent(is_focused))
        .title_bar(
            TitleBar::new(title_bar(pane, state)).style(Container::PaneGridTitleBar(is_focused)),
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
