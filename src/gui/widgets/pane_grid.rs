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
use std::vec::Vec;

pub fn pane_grid<'a>(
    panes: &'a Panes,
    tabs: &'a [DefaultKey],
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let active = panes.active == pane;

        let mut pane_tabs = Vec::new(); //use array since we know the size
        for key in state.data.iter() {
            pane_tabs.push(map.get(*key).unwrap());
        }

        Content::new(match state.tab {
            Tab::File(_) => tab_bar(state.active_tab, &pane_tabs),
        })
        .style(Container::PaneGridContent(active))
        .title_bar(TitleBar::new(title_bar(pane, state)).style(Container::PaneGridTitleBar(active)))
    })
    .on_click(Message::PaneClicked)
    .on_drag(Message::PaneDragged)
    .on_resize(10, Message::PaneResized)
    .spacing(15)
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
