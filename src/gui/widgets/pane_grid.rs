use crate::gui::{
    theme::{Container, Renderer},
    widgets::tab_bar::tab_bar,
    Message, Panes, Tab,
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
use std::vec::Vec;

pub fn pane_grid<'a>(
    panes: &'a Panes,
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let active = panes.active == pane;
        let pane_tabs: Vec<_> = state // this is kind of stupid TODO
            .data
            .iter()
            .map(|key| map.get(*key).unwrap())
            .collect();

        Content::new(match state.tab {
            Tab::File(_) => tab_bar(state.active_tab, &pane_tabs),
        })
        .style(Container::PaneGridContent(active))
        .title_bar(title_bar(active, pane))
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .spacing(15)
}

fn title_bar(active: bool, pane: Pane) -> TitleBar<'static, Message, Renderer> {
    TitleBar::new(
        row!(
            text("content"),
            button("split |").on_press(Message::PaneSplit(Axis::Vertical, pane)),
            button("split -").on_press(Message::PaneSplit(Axis::Horizontal, pane)),
            button("close THIS pane").on_press(Message::PaneClosed(pane))
        )
        .align_items(Alignment::Center),
    )
    .style(Container::PaneGridTitleBar(active))
}
