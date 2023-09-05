use crate::gui::{
    theme::{Container, Renderer},
    widgets::tab_bar::tab_bar,
    Message, Panes, Tab,
};

use iced::{
    widget::{
        button,
        pane_grid::{Axis, Content, Pane, PaneGrid, TitleBar},
        row,
    },
    Alignment, Length,
};
use slotmap::{DefaultKey, SlotMap};

pub fn pane_grid<'a>(
    panes: &'a Panes,
    map: &SlotMap<DefaultKey, Tab>,
) -> PaneGrid<'a, Message, Renderer> {
    PaneGrid::new(&panes.data, |pane, state, _is_maximized| {
        let active = panes.active == pane;

        Content::new(tab_bar(state.active_tab, &state.get_data(map))) // currently this is fine if we want all gui elements to be tabs
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
            row!().width(Length::Fill),
            row!("What to put here?").width(Length::Fill),
            row!(
                button("|").on_press(Message::PaneSplit(Axis::Vertical, pane)),
                button("--").on_press(Message::PaneSplit(Axis::Horizontal, pane)),
                button("x").on_press(Message::PaneClosed(pane)),
            )
        )
        .align_items(Alignment::Center),
    )
    .style(Container::PaneGridTitleBar(active))
    .padding(25) //adding padding makes a grabable section?
}
